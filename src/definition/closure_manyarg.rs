use std::any::{ Any };
use std::boxed::BoxAny;
use std::rc::{ Rc };
use std::cell::{ RefCell };

use super::{ Definition, ToDefinition, TypeDef };
use super::super::getter::{ GetterWrap, Getter };

/// Creates `Definition` from closure function.
#[stable]
impl<A0:'static, T:'static> ToDefinition for (|A0|:'static -> T) {
    fn to_definition<'a>(self) -> Box<Definition + 'a> {
        // We have only one closure, but the definition
        // will need to create many getters. There is no way around it
        // but put this single closure into reference-counted cell
        // so it can be uniquely dereferenced and called when each cloned Rc
        // is invoked as getter.
        box Rc::new(RefCell::new(self))
    }
}

impl<A0:'static, T:'static> Definition for Rc<RefCell<|A0|:'static -> T>> {
    fn get_type(&self) -> TypeDef {
        TypeDef::of::<T>()
    }

    fn get_arg_types(&self) -> Vec<TypeDef> {
        vec![TypeDef::of::<A0>()]
    }

    fn get_getter(&self, arg_getters: Vec<Box<Any>>) -> Box<Any> {
        let mut getters = arg_getters;
        box GetterWrap::<T>::new(
            box GetterScope1::<A0, T> {
                a0: *(getters.pop().expect("arg 0 missing").downcast::<GetterWrap<A0>>().ok().expect("bad arg 0")),
                closure: self.clone(),
            }
        ) as Box<Any>
    }
}

struct GetterScope1<'a, A0, T> {
    a0: GetterWrap<'a, A0>,
    closure: Rc<RefCell<|A0|:'static -> T>>,
}

impl<'a, A0: 'static, T: 'static> Getter<T> for GetterScope1<'a, A0, T> {
    fn get(&self) -> T {
        (*(self.closure.borrow_mut().deref_mut()))(
            self.a0.get()
        )
    }
}
