use std::any::Any;
// use std::collections::VecDeque;
// use std::sync::mpsc::channel;

use { Parent };

/// Registers core dependencies between parents and childs.
pub trait OnMany {
    fn on_2<P1, P2, C, F>(&mut self, constructor: F)
        where
            P1: 'static + Any,
            P2: 'static + Any,
            C: 'static + Any,
            F: for<'r> Fn(Parent<P1>, Parent<P2>) -> C + 'static;
}

// impl OnMany for Deps {
//     fn on_2<P1, P2, C, F>(&mut self, constructor: F)
//         where
//             P1: 'static + Any,
//             P2: 'static + Any,
//             C: 'static + Any,
//             F: for<'r> Fn(Parent<P1>, Parent<P2>) -> C + 'static
//     {
//         let inceptions: VecDeque<Group2<P1, P2>> = VecDeque::new();
//
//         self.register_child_constructor::<P1>(Box::new(move |deps: &Deps, parent: &mut Any| -> Option<Box<Any>> {
//             let concrete_parent = parent.downcast_mut::<P1>().unwrap();
//             //let child_obj = constructor(Parent::<P1> { obj: concrete_parent });
//             //let child = deps.create_deps(child_obj);
//             //Some(Box::new(child))
//             Some(Box::new(()))
//         }));
//
//         self.register_child_constructor::<P2>(Box::new(move |deps: &Deps, parent: &mut Any| -> Option<Box<Any>> {
//             let concrete_parent = parent.downcast_mut::<P2>().unwrap();
//             //let child_obj = constructor(Parent::<P1> { obj: concrete_parent });
//             //let child = deps.create_deps(child_obj);
//             //Some(Box::new(child))
//             Some(Box::new(()))
//         }));
//     }
// }

// struct Group2<P1, P2> {
//     pub p1: Option<P1>,
//     pub p2: Option<P2>,
// }
