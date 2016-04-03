use std::any::Any;
use Deps;
use Parent;

/// Registers core dependencies between parents and childs.
pub trait On {
    /// Single dependency on parent.
    fn loader<P, C, F>(&mut self, constructor: F)
        where
            P: 'static + Any, C: 'static + Any,
            F: for<'r> Fn(&Deps, Parent<P>) -> C + 'static + Send + Sync;
}

impl On for Deps {
    /// Single dependency on parent.
    fn loader<P, C, F>(&mut self, constructor: F)
        where
            P: 'static + Any, C: 'static + Any,
            F: for<'r> Fn(&Deps, Parent<P>) -> C + 'static + Send + Sync
    {
        self.register_child_constructor::<P>(into_constructor_with_deps(constructor));
    }
}

fn into_constructor_with_deps<P, C, F>(constructor: F) -> Box<Fn(&Deps, &mut Any) -> Option<Box<Any>> + Send + Sync>
    where F: for<'r> Fn(&Deps, Parent<P>) -> C + 'static + Send + Sync, P: 'static + Any, C: 'static + Any
{
    Box::new(move |deps: &Deps, parent: &mut Any| -> Option<Box<Any>> {
        let concrete_parent = parent.downcast_mut::<P>().unwrap();
        let child = deps.create_deps(constructor(deps, Parent::<P> { obj: concrete_parent }));
        Some(Box::new(child))
    })
}

#[cfg(test)]
mod test {
    use { Deps, Parent, WithDeps };
    use super::*;
    use std::sync::{ Arc, Mutex };

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

        deps.loader({
            let created_b_ref = created_b_ref.clone();
            move |_: &Deps, a: Parent<A>| {
                let b = B([&a.0[..], "+B"].concat());
                *created_b_ref.lock().unwrap() = Some(b.clone());
                b
            }
        });

        A("Hello".into()).with_deps(&deps);

        assert_eq!("Hello+B", (*created_b_ref.lock().unwrap()).clone().unwrap().0);
    }

    #[test]
    fn creates_dependency_of_dependency() {
        let mut deps = Deps::new();

        // here we want to know what is the state of dependency in closure, hence
        // shared mutable reference to it
        let created_c_ref = Arc::new(Mutex::new(None));

        deps.loader(|_: &Deps, a: Parent<A>| B([&a.0[..], "+B"].concat()));

        deps.loader({
            let created_c_ref = created_c_ref.clone();
            move |_: &Deps, b: Parent<B>| {
                let c = C([&b.0[..], "+C"].concat());
                *created_c_ref.lock().unwrap() = Some(c.clone());
                c
            }
        });

        A("Hello".into()).with_deps(&deps);

        assert_eq!("Hello+B+C", (*created_c_ref.lock().unwrap()).clone().unwrap().0);
    }

    #[test]
    fn creates_mutable_dependency() {
        let mut deps = Deps::new();

        deps.loader(|_: &Deps, mut a: Parent<A>| *a = A("Hi!".into()));

        let a = A("Hello".into()).with_deps(&deps);

        assert_eq!("Hi!", a.obj.0);
    }
}
