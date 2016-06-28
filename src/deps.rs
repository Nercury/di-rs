use std::any::{Any, TypeId};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use constructed::{Constructed, ConstructedShared, AnyInstance};
use inceptor::{Inceptor, Destructor};
use {Result, Collection, Scope};
use typedef::TypeDef;

pub struct Deps {
    empty_type: TypeId,
    /// List of functions that constructs all childs for a type
    /// and returns value wrapped in Any that must live as long as the parent type.
    isolated_constructors: HashMap<TypeId,
                                       Vec<Box<Fn(&Deps, &mut Box<Any>) -> Result<Constructed> + Send + Sync>>>,
    shared_constructors: HashMap<TypeId, Vec<Box<Fn(&Deps, &mut Box<Any>) -> Result<ConstructedShared> + Send + Sync>>>,
    type_scope_created: HashMap<TypeId,
                                Vec<Box<Fn(&Deps, &mut AnyInstance) -> Result<()> + Send + Sync>>>,
    inceptors: HashMap<(TypeId, TypeId), Box<Any>>,
}

fn to_shared<T: Any>(not_shared: Box<Any>) -> Box<Any> {
    let parent: T = *not_shared.downcast::<T>()
        .expect("expected downcast to P when \
                 changing to shared P");
    debug!("converting type {:?} to shared type",
           TypeDef::name_of::<T>());
    Box::new(Arc::new(Mutex::new(parent)))
}

impl Deps {
    pub fn new() -> Deps {
        Deps {
            empty_type: TypeId::of::<()>(),
            isolated_constructors: HashMap::new(),
            shared_constructors: HashMap::new(),
            type_scope_created: HashMap::new(),
            inceptors: HashMap::new(),
        }
    }

    /// Create dependencies for specified `obj` and return a wrapper `Scope` object.
    ///
    /// The wrapper `Scope` keeps ownership of all children together with parent object.
    pub fn create<P: Any>(&self, obj: P) -> Result<Scope<P>> {
        let type_id = TypeId::of::<P>();
        let type_name = TypeDef::name_of::<P>();

        trace!("create {:?}, id {:?}", type_name, type_id);

        let (parent, deps) =
            try!(self.create_deps_for_any_parent(type_id, Box::new(obj), to_shared::<P>));
        Ok(Scope::from_any_instance(parent, deps))
    }

    fn create_deps_for_any_parent<F>(&self,
                                     type_id: TypeId,
                                     mut parent_not_shared: Box<Any>,
                                     to_shared: F)
                                     -> Result<(AnyInstance, Vec<Box<Any>>)>
        where F: Fn(Box<Any>) -> Box<Any>
    {

        trace!("create shared type {:?}", type_id);

        let mut deps = Vec::new();

        // First, construct any instances that do not need parent wrapped in mutex

        match self.isolated_constructors.get(&type_id) {
            Some(isolated_list) => {
                trace!("go over isolated constructors");
                for any_constructor in isolated_list {
                    trace!("constructor");
                    match any_constructor(&self, &mut parent_not_shared) {
                        Ok(Constructed { children }) => {
                            trace!("constructed {:?}", children);
                            deps.extend(children)
                        }
                        Err(any_err) => {
                            trace!("construction error {:?}", any_err);
                            return Err(any_err);
                        }
                    };
                }
            }
            None => (),
        }

        // Then, check if there are shared constructors, and if so, wrap value in mutex
        // and return it in AnyInstance::Shared, otherwise, return it in AnyInstance::Isolated.

        let mut parent_result = match self.shared_constructors.get(&type_id) {
            Some(shared_list) => {
                let mut parent_shared = to_shared(parent_not_shared);

                trace!("go over shared constructors");
                for any_constructor in shared_list {
                    trace!("constructor");
                    match any_constructor(&self, &mut parent_shared) {
                        Ok(ConstructedShared { children }) => {
                            trace!("constructed {:?}", children);
                            deps.extend(children)
                        }
                        Err(any_err) => {
                            trace!("construction error {:?}", any_err);
                            return Err(any_err);
                        }
                    };
                }

                AnyInstance::Shared(parent_shared)
            }
            None => AnyInstance::Isolated(parent_not_shared),
        };

        // Execute post create actions for the value

        trace!("go over actions");
        if let Some(actions) = self.type_scope_created.get(&type_id) {
            for action in actions {
                try!(action(&self, &mut parent_result));
            }
        }

        Ok((parent_result, deps))
    }

    /// Collect all the items registered as `collectable` into a `Collection` of that type.
    pub fn collect<C: Any>(&self) -> Result<Collection<C>> {
        self.create(Collection::new()).map(|v| v.explode())
    }

    pub fn when_ready<T, F>(&mut self, action: F)
        where T: 'static + Any,
              F: for<'r> Fn(&Deps, &mut T) -> Result<()> + 'static + Send + Sync
    {
        match self.type_scope_created.entry(TypeId::of::<T>()) {
            Entry::Occupied(mut list) => {
                list.get_mut().push(into_action_with_deps(action));
            }
            Entry::Vacant(e) => {
                e.insert(vec![into_action_with_deps(action)]);
            }
        };
    }

    /// Single dependency on a parent.
    pub fn attach<P, C, F>(&mut self, constructor: F)
        where P: 'static + Any, // Parent
              C: 'static + Any, // Child
              F: for<'r> Fn(&Deps, &mut P) -> Result<C> + 'static + Send + Sync
    {
        if TypeId::of::<C>() == self.empty_type {
            self.register_isolated_constructor::<P>(into_isolated_constructor_with_ignored_child_deps(constructor));
        } else {
            self.register_isolated_constructor::<P>(into_isolated_constructor_with_child_deps(constructor));
        }
    }

    /// Single dependency on multiple parents.
    pub fn bridge<P1, P2, C, F>(&mut self, constructor: F)
        where P1: 'static + Any + Send + Sync, // Parent 1
              P2: 'static + Any + Send + Sync, // Parent 2
              C: 'static + Any, // Child
              F: for<'r> Fn(&mut P1, &mut P2) -> Result<C> + 'static + Send + Sync
    {
        // Get or insert inceptor that is used to manage P1 and P2 instances.
        let inceptor_1 = match self.inceptors
            .entry((TypeId::of::<P1>(), TypeId::of::<P2>())) {
            Entry::Occupied(entry) => {
                entry.get()
                    .downcast_ref::<Arc<Mutex<Inceptor<P1, P2>>>>()
                    .expect("expected to find Inceptor of correct type in map")
                    .clone()
            }
            Entry::Vacant(entry) => {
                let arc = Arc::new(Mutex::new(if TypeId::of::<C>() == self.empty_type {
                    Inceptor::new_with_ignored_return_val(constructor)
                } else {
                    Inceptor::new_with_return_val(constructor)
                }));
                entry.insert(Box::new(arc.clone()));
                arc
            }
        };

        // Create inceptor clone for P2 instances
        let inceptor_2 = inceptor_1.clone();

        self.register_shared_constructor::<P1>(into_shared_constructor_1::<P1, P2, C>(inceptor_1));
        self.register_shared_constructor::<P2>(into_shared_constructor_2::<P1, P2, C>(inceptor_2));
    }

    pub fn collectable<C, F>(&mut self, constructor: F)
        where C: 'static + Any,
              F: for<'r> Fn(&Deps) -> C + 'static + Send + Sync
    {
        self.register_isolated_constructor::<Collection<C>>(
            into_isolated_constructor_without_child_deps(move |deps: &Deps, parent: &mut Collection<C>| {
                parent.push(constructor(deps))
            })
        );
    }

    /// Register child constructor that will be invoked when the parent `P` type is
    /// created.
    fn register_isolated_constructor<P: Any>(&mut self,
                                             any_constructor: Box<Fn(&Deps, &mut Box<Any>)
                                                                     -> Result<Constructed> + Send + Sync>) {
        debug!("register_isolated_constructor {:?}", TypeId::of::<P>());
        match self.isolated_constructors.entry(TypeId::of::<P>()) {
            Entry::Occupied(mut list) => {
                list.get_mut().push(any_constructor);
            }
            Entry::Vacant(e) => {
                e.insert(vec![any_constructor]);
            }
        };
    }

    /// Register child constructor that will be invoked when the parent `P` type is
    /// created.
    fn register_shared_constructor<P: Any>(&mut self,
                                           any_constructor: Box<Fn(&Deps, &mut Box<Any>)
                                                                   -> Result<ConstructedShared> + Send + Sync>) {
        debug!("register_shared_constructor {:?}", TypeId::of::<P>());
        match self.shared_constructors.entry(TypeId::of::<P>()) {
            Entry::Occupied(mut list) => {
                list.get_mut().push(any_constructor);
            }
            Entry::Vacant(e) => {
                e.insert(vec![any_constructor]);
            }
        };
    }
}

unsafe impl Send for Deps {}
unsafe impl Sync for Deps {}

fn into_action_with_deps<P, F>(action: F)
                               -> Box<Fn(&Deps, &mut AnyInstance) -> Result<()> + Send + Sync>
    where F: for<'r> Fn(&Deps, &mut P) -> Result<()> + 'static + Send + Sync,
          P: 'static + Any
{
    debug!("into_action_with_deps for type {:?}", TypeId::of::<P>());

    Box::new(move |deps: &Deps, parent: &mut AnyInstance| -> Result<()> {
        match *parent {
            AnyInstance::Isolated(ref mut value) => {
                try!(action(deps,
                            &mut value.downcast_mut::<P>()
                                .expect("expected to downcast type in post create action")))
            }
            AnyInstance::Shared(ref mut value) => {
                try!(action(deps,
                            &mut value.downcast_mut::<Arc<Mutex<P>>>()
                                .expect("expected to downcast type in post create action")
                                .lock()
                                .expect("expected to lock value for AnyInstance::Shared action")))
            }
        };
        Ok(())
    })
}

fn into_shared_constructor_1<P1, P2, C>
    (inceptor: Arc<Mutex<Inceptor<P1, P2>>>)
     -> Box<Fn(&Deps, &mut Box<Any>) -> Result<ConstructedShared> + Send + Sync>
    where P1: 'static + Any + Send + Sync, // Parent 1
          P2: 'static + Any + Send + Sync, // Parent 2
          C: 'static + Any // Child
{
    Box::new(move |deps: &Deps, parent: &mut Box<Any>| -> Result<ConstructedShared> {
        let parent_for_inceptor = parent.downcast_mut::<Arc<Mutex<P1>>>()
            .expect("expected downcast")
            .clone();
        let (id, instances) = try!(inceptor.lock()
            .expect("failed to lock")
            .incept_1(parent_for_inceptor));

        let mut children: Vec<Box<Any>> = Vec::with_capacity(instances.len() + 1);

        for instance in instances {
            let instance_artifacts =
                try!(deps.create_deps_for_any_parent(TypeId::of::<C>(), instance, to_shared::<C>));
            children.push(Box::new(instance_artifacts));
        }

        children.push(Box::new(Destructor::new(inceptor.clone(), 1, id)));

        Ok(ConstructedShared { children: children })
    })
}

fn into_shared_constructor_2<P1, P2, C>
    (inceptor: Arc<Mutex<Inceptor<P1, P2>>>)
     -> Box<Fn(&Deps, &mut Box<Any>) -> Result<ConstructedShared> + Send + Sync>
    where P1: 'static + Any + Send + Sync, // Parent 1
          P2: 'static + Any + Send + Sync, // Parent 2
          C: 'static + Any // Child
{
    Box::new(move |deps: &Deps, parent: &mut Box<Any>| -> Result<ConstructedShared> {
        let parent_for_inceptor = parent.downcast_mut::<Arc<Mutex<P2>>>()
            .expect("expected downcast")
            .clone();
        let (id, instances) = try!(inceptor.lock()
            .expect("failed to lock")
            .incept_2(parent_for_inceptor));

        let mut children: Vec<Box<Any>> = Vec::with_capacity(instances.len() + 1);

        for instance in instances {
            let instance_artifacts =
                try!(deps.create_deps_for_any_parent(TypeId::of::<C>(), instance, to_shared::<C>));
            children.push(Box::new(instance_artifacts));
        }

        children.push(Box::new(Destructor::new(inceptor.clone(), 2, id)));

        Ok(ConstructedShared { children: children })
    })
}

fn into_isolated_constructor_with_child_deps<P, C, F>
    (constructor: F)
     -> Box<Fn(&Deps, &mut Box<Any>) -> Result<Constructed> + Send + Sync>
    where F: for<'r> Fn(&Deps, &mut P) -> Result<C> + 'static + Send + Sync,
          P: 'static + Any,
          C: 'static + Any
{
    debug!("into_isolated_constructor_with_child_deps for type {:?}",
           TypeId::of::<P>());

    Box::new(move |deps: &Deps, parent: &mut Box<Any>| -> Result<Constructed> {
        let child = {
            let concrete_parent = parent.downcast_mut::<P>()
                .expect("expected to downcast type in into_isolated_constructor_with_child_deps");
            try!(deps.create(try!(constructor(deps, concrete_parent))))
        };
        Ok(Constructed { children: vec![Box::new(child)] })
    })
}

fn into_isolated_constructor_with_ignored_child_deps<P, C, F>
    (constructor: F)
     -> Box<Fn(&Deps, &mut Box<Any>) -> Result<Constructed> + Send + Sync>
    where F: for<'r> Fn(&Deps, &mut P) -> Result<C> + 'static + Send + Sync,
          P: 'static + Any,
          C: 'static + Any
{
    debug!("into_isolated_constructor_with_ignored_child_deps for type {:?}",
           TypeId::of::<P>());

    Box::new(move |deps: &Deps, parent: &mut Box<Any>| -> Result<Constructed> {
        try!(constructor(deps,
                         parent.downcast_mut::<P>()
                             .expect("expected to downcast type in \
                                      into_isolated_constructor_with_ignored_child_deps")));
        Ok(Constructed { children: vec![] })
    })
}

fn into_isolated_constructor_without_child_deps<P, F>
    (constructor: F)
     -> Box<Fn(&Deps, &mut Box<Any>) -> Result<Constructed> + Send + Sync>
    where F: for<'r> Fn(&Deps, &mut P) + 'static + Send + Sync,
          P: 'static + Any
{
    debug!("into_isolated_constructor_without_child_deps for type {:?}",
           TypeId::of::<P>());

    Box::new(move |deps: &Deps, parent: &mut Box<Any>| -> Result<Constructed> {
        constructor(deps,
                    parent.downcast_mut::<P>()
                        .expect("expected to downcast type in \
                                 into_isolated_constructor_without_child_deps"));
        Ok(Constructed { children: vec![] })
    })
}

#[cfg(test)]
mod test {
    use env_logger;
    use Deps;
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct A(String);

    #[derive(Clone)]
    struct B(String);

    #[derive(Clone)]
    struct C(String);

    #[test]
    fn creates_dependency() {
        let mut deps = Deps::new();

        // here we want to know what is the state of dependency in closure, hence
        // shared mutable reference to it
        let created_b_ref = Arc::new(Mutex::new(None));

        deps.attach({
            let created_b_ref = created_b_ref.clone();
            move |_: &Deps, a: &mut A| {
                let b = B([&a.0[..], "+B"].concat());
                *created_b_ref.lock().unwrap() = Some(b.clone());
                Ok(b)
            }
        });

        deps.create(A("Hello".into())).unwrap();

        assert_eq!("Hello+B",
                   (*created_b_ref.lock().unwrap()).clone().unwrap().0);
    }

    #[test]
    fn creates_dependency_of_dependency() {
        let mut deps = Deps::new();

        // here we want to know what is the state of dependency in closure, hence
        // shared mutable reference to it
        let created_c_ref = Arc::new(Mutex::new(None));

        deps.attach(|_: &Deps, a: &mut A| Ok(B([&a.0[..], "+B"].concat())));

        deps.attach({
            let created_c_ref = created_c_ref.clone();
            move |_: &Deps, b: &mut B| {
                let c = C([&b.0[..], "+C"].concat());
                *created_c_ref.lock().unwrap() = Some(c.clone());
                Ok(c)
            }
        });

        deps.create(A("Hello".into())).unwrap();

        assert_eq!("Hello+B+C",
                   (*created_c_ref.lock().unwrap()).clone().unwrap().0);
    }

    #[test]
    fn creates_mutable_dependency() {
        let mut deps = Deps::new();

        deps.attach(|_: &Deps, a: &mut A| {
            *a = A("Hi!".into());
            Ok(())
        });

        let mut a = deps.create(A("Hello".into())).unwrap();
        let al = a.lock().unwrap();

        assert_eq!("Hi!", al.0);
    }

    #[test]
    fn bridge_dependency() {
        env_logger::init().unwrap();

        let mut deps = Deps::new();

        let created_bridge = Arc::new(Mutex::new(None));
        let created_bridge_clone = created_bridge.clone(); // so we can modify this from inside the closure

        deps.bridge(|a: &mut A, b: &mut B| Ok(vec![a.0.clone(), b.0.clone()]));

        // Use this to copy created Vec<String> value from bridge to mutex protected clone
        deps.when_ready(move |_: &Deps, parent: &mut Vec<String>| {
            *created_bridge_clone.lock().unwrap() = Some(parent.clone());
            Ok(())
        });

        // Bind to created A and modify the value from "Hello" to "Hi"
        deps.attach(|_: &Deps, a: &mut A| {
            *a = A("Hi".into());
            Ok(5)
        });

        // Attach to any type Vec<String> and append "Nice" to last element
        deps.attach(|_: &Deps, created_bridge_result: &mut Vec<String>| {
            created_bridge_result.push("Nice".to_string());
            Ok(())
        });

        // Create both instigators and result should appear
        let mut a = deps.create(A("Hello".into())).unwrap();
        let mut b = deps.create(B("World".into())).unwrap();

        {
            let al = a.lock().unwrap();
            let bl = b.lock().unwrap();

            assert_eq!("Hi", al.0);
            assert_eq!("World", bl.0);
        }

        {
            let val = created_bridge.lock()
                .unwrap();
            assert_eq!("HiWorldNice",
                       val.as_ref().expect("expected bridge val to be created").concat());
        }

        let mut c = deps.create(B("Rust".into())).unwrap();

        {
            let cl = c.lock().unwrap();

            assert_eq!("Rust", cl.0);
        }

        {
            let val = created_bridge.lock()
                .unwrap();
            assert_eq!("HiRustNice",
                       val.as_ref().expect("expected bridge val to be created").concat());
        }
    }
}
