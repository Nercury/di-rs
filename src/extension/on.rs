use std::any::Any;
use Deps;

/// Registers core dependencies between parents and childs.
pub trait On {
    fn on<P, C, F>(&mut self, constructor: F)
        where
            P: 'static + Any, C: 'static + Any,
            F: for<'r> Fn(&'r Deps, &P) -> C + 'static;
}

impl On for Deps {
    /// Single dependency on parent.
    fn on<P, C, F>(&mut self, constructor: F)
        where
            P: 'static + Any, C: 'static + Any,
            F: for<'r> Fn(&'r Deps, &P) -> C + 'static
    {
        self.register_child_constructor::<P>(into_constructor(constructor));
    }
}

fn into_constructor<P, C, F>(constructor: F) -> Box<Fn(&Deps, &Any) -> Option<Box<Any>>>
    where F: for<'r> Fn(&'r Deps, &P) -> C + 'static, P: 'static + Any, C: 'static + Any
{
    Box::new(move |deps: &Deps, parent: &Any| -> Option<Box<Any>> {
        let concrete_parent = parent.downcast_ref::<P>().unwrap();
        let child = constructor(deps, concrete_parent);
        Some(Box::new(child))
    })
}
