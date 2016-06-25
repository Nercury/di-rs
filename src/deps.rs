use std::any::{ Any, TypeId };
use std::ops::{ Deref, DerefMut };
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use Collection;

pub trait Features {
    fn register(&mut Deps);
}

pub struct Deps {
    /// List of functions that constructs all childs for a type
    /// and returns value wrapped in Any that must live as long as the parent type.
    type_child_constructors: HashMap<
        TypeId,
        Vec<Box<
            Fn(&Deps, &mut Any) -> Option<Box<Any>> + Send + Sync
        >>
    >,
    type_scope_created: HashMap<
        TypeId,
        Vec<Box<
            Fn(&Deps, &mut Any) + Send + Sync
        >>
    >,
}

impl Deps {
    pub fn new() -> Deps {
        Deps {
            type_child_constructors: HashMap::new(),
            type_scope_created: HashMap::new(),
        }
    }

    /// Create dependencies for specified `obj` and return a wrapper `Scope` object.
    ///
    /// The wrapper `Scope` keeps ownership of all children together with parent object.
    pub fn create_for<P: Any>(&self, mut obj: P) -> Scope<P> {
        match self.type_child_constructors.get(&TypeId::of::<P>()) {
            // if there are type child constructors
            Some(list) => {
                // run each child constructor and receive list of objects that will be kept inside
                // the parent scope.
                let deps: Vec<_> = list.iter()
                    .filter_map(|any_constructor| any_constructor(&self, &mut obj))
                    .collect();

                if let Some(actions) = self.type_scope_created.get(&TypeId::of::<P>()) {
                    for action in actions {
                        action(&self, &mut obj);
                    }
                }

                Scope { obj: obj, childs: deps }
            },
            // if there are no type childs, wrap the type in scope anyways with empty child list.
            None => Scope { obj: obj, childs: vec![] },
        }
    }

    /// Collect all the items of the type into a Vec.
    pub fn collect<C: Any>(&self) -> Vec<C> {
        self.create_for(Collection::new()).explode().into()
    }

    /// Register child constructor that will be invoked when the parent `P` type is
    /// created.
    pub fn register_child_constructor<P: Any>(
        &mut self,
        any_constructor: Box<Fn(&Deps, &mut Any) -> Option<Box<Any>> + Send + Sync>
    ) {
        match self.type_child_constructors.entry(TypeId::of::<P>()) {
            Entry::Occupied(mut list) => {
                list.get_mut().push(any_constructor);
            },
            Entry::Vacant(e) => {
                e.insert(vec![any_constructor]);
            },
        };
    }

    /// Register a bunch of features at once.
    pub fn with<F: Features>(&mut self) -> &mut Self {
        F::register(self);
        self
    }

    pub fn on_create<P, F>(&mut self, action: F)
        where
            P: 'static + Any,
            F: for<'r> Fn(&Deps, Parent<P>) + 'static + Send + Sync
    {
        match self.type_scope_created.entry(TypeId::of::<P>()) {
            Entry::Occupied(mut list) => {
                list.get_mut().push(into_action_with_deps(action));
            },
            Entry::Vacant(e) => {
                e.insert(vec![into_action_with_deps(action)]);
            },
        };
    }

    /// Single dependency on parent.
    pub fn on<P, C, F>(&mut self, constructor: F)
        where
            P: 'static + Any, C: 'static + Any,
            F: for<'r> Fn(&Deps, Parent<P>) -> C + 'static + Send + Sync
    {
        self.register_child_constructor::<P>(
            into_constructor_with_child_deps(constructor)
        );
    }

    pub fn collectable<C, F>(&mut self, constructor: F)
        where
            C: 'static + Any,
            F: for<'r> Fn(&Deps) -> C + 'static + Send + Sync
    {
        self.register_child_constructor::<Collection<C>>(
            into_constructor_without_child_deps(move |deps: &Deps, mut parent: Parent<Collection<C>>| {
                parent.push(constructor(deps))
            })
        );
    }
}

fn into_action_with_deps<P, F>(action: F) -> Box<Fn(&Deps, &mut Any) + Send + Sync>
    where F: for<'r> Fn(&Deps, Parent<P>) + 'static + Send + Sync, P: 'static + Any
{
    Box::new(move |deps: &Deps, parent: &mut Any| {
        let concrete_parent = parent.downcast_mut::<P>().unwrap();
        action(deps, Parent::<P> { obj: concrete_parent })
    })
}

fn into_constructor_with_child_deps<P, C, F>(constructor: F) -> Box<Fn(&Deps, &mut Any) -> Option<Box<Any>> + Send + Sync>
    where F: for<'r> Fn(&Deps, Parent<P>) -> C + 'static + Send + Sync, P: 'static + Any, C: 'static + Any
{
    Box::new(move |deps: &Deps, parent: &mut Any| -> Option<Box<Any>> {
        let concrete_parent = parent.downcast_mut::<P>().unwrap();
        let child = deps.create_for(constructor(deps, Parent::<P> { obj: concrete_parent }));
        Some(Box::new(child))
    })
}

fn into_constructor_without_child_deps<P, C, F>(constructor: F) -> Box<Fn(&Deps, &mut Any) -> Option<Box<Any>> + Send + Sync>
    where F: for<'r> Fn(&Deps, Parent<P>) -> C + 'static + Send + Sync, P: 'static + Any
{
    Box::new(move |deps: &Deps, parent: &mut Any| -> Option<Box<Any>> {
        let concrete_parent = parent.downcast_mut::<P>().unwrap();
        constructor(deps, Parent::<P> { obj: concrete_parent });
        None
    })
}

#[derive(Debug)]
pub struct Parent<'a, T: 'a> {
    pub obj: &'a mut T,
}

impl<'a, T> Deref for Parent<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.obj
    }
}

impl<'a, T> DerefMut for Parent<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.obj
    }
}

#[derive(Debug)]
pub struct Scope<T> {
    pub obj: T,
    childs: Vec<Box<Any>>,
}

impl<T> Scope<T> {
    pub fn explode(self) -> T {
        self.obj
    }
}

impl<T> Deref for Scope<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.obj
    }
}

impl<T> DerefMut for Scope<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.obj
    }
}
