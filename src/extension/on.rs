use std::any::Any;
use Deps;
use Parent;

/// Registers core dependencies between parents and childs.
pub trait On {
    fn on<P, C, F>(&mut self, constructor: F)
        where
            P: 'static + Any, C: 'static + Any,
            F: for<'r> Fn(Parent<P>) -> C + 'static;
}

impl On for Deps {
    /// Single dependency on parent.
    fn on<P, C, F>(&mut self, constructor: F)
        where
            P: 'static + Any, C: 'static + Any,
            F: for<'r> Fn(Parent<P>) -> C + 'static
    {
        self.register_child_constructor::<P>(into_constructor(constructor));
    }
}

fn into_constructor<P, C, F>(constructor: F) -> Box<Fn(&Deps, &mut Any) -> Option<Box<Any>>>
    where F: for<'r> Fn(Parent<P>) -> C + 'static, P: 'static + Any, C: 'static + Any
{
    Box::new(move |deps: &Deps, parent: &mut Any| -> Option<Box<Any>> {
        let concrete_parent = parent.downcast_mut::<P>().unwrap();
        let child = deps.create_deps(constructor(Parent::<P> { obj: concrete_parent }));
        Some(Box::new(child))
    })
}

#[cfg(test)]
mod test {
    use { Deps, Parent, WithAll };
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

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
        let created_b_ref = Rc::new(RefCell::new(None));

        deps.on({
            let created_b_ref = created_b_ref.clone();
            move |a: Parent<A>| {
                let b = B([&a.0[..], "+B"].concat());
                *created_b_ref.borrow_mut() = Some(b.clone());
                b
            }
        });

        A("Hello".into()).with_all(&deps);

        assert_eq!("Hello+B", (*created_b_ref.borrow()).clone().unwrap().0);
    }

    #[test]
    fn creates_dependency_of_dependency() {
        let mut deps = Deps::new();

        // here we want to know what is the state of dependency in closure, hence
        // shared mutable reference to it
        let created_c_ref = Rc::new(RefCell::new(None));

        deps.on(|a: Parent<A>| B([&a.0[..], "+B"].concat()));

        deps.on({
            let created_c_ref = created_c_ref.clone();
            move |b: Parent<B>| {
                let c = C([&b.0[..], "+C"].concat());
                *created_c_ref.borrow_mut() = Some(c.clone());
                c
            }
        });

        A("Hello".into()).with_all(&deps);

        assert_eq!("Hello+B+C", (*created_c_ref.borrow()).clone().unwrap().0);
    }

    #[test]
    fn creates_mutable_dependency() {
        let mut deps = Deps::new();

        deps.on(|mut a: Parent<A>| *a = A("Hi!".into()));

        let a = A("Hello".into()).with_all(&deps);

        assert_eq!("Hi!", a.obj.0);
    }
}
