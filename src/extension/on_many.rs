use std::any::Any;
use { Deps, Parent };

/// Registers core dependencies between parents and childs.
pub trait OnMany {
    fn on_2<P1, P2, C, F>(&mut self, constructor: F)
        where
            P1: 'static + Any,
            P2: 'static + Any,
            C: 'static + Any,
            F: for<'r> Fn(Parent<P1>, Parent<P2>) -> C + 'static;
}

impl OnMany for Deps {
    fn on_2<P1, P2, C, F>(&mut self, constructor: F)
        where
            P1: 'static + Any,
            P2: 'static + Any,
            C: 'static + Any,
            F: for<'r> Fn(Parent<P1>, Parent<P2>) -> C + 'static
    {
        //self.register_child_constructor::<P>(into_constructor(constructor));
    }
}
