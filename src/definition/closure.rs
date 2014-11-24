use std::any::{ Any };
use std::rc::{ Rc };
use std::cell::{ RefCell };

use super::{ Definition, ToDefinition, TypeDef };
use super::super::getter::{ GetterWrap, Getter };

/// Creates `Definition` from closure function.
#[stable]
impl<T:'static> ToDefinition for (||:'static -> T) {
    fn to_definition<'a>(self) -> Box<Definition + 'a> {
        // We have only one closure, but the definition
        // will need to create many getters. There is no way around it
        // but put this single closure into reference-counted cell
        // so it can be uniquely dereferenced and called when each cloned Rc
        // is invoked as getter.
        box Rc::new(RefCell::new(self))
    }
}

impl<T:'static> Definition for Rc<RefCell<||:'static -> T>> {
    fn get_type(&self) -> TypeDef {
        TypeDef::of::<T>()
    }

    fn get_arg_types(&self) -> Vec<TypeDef> {
        Vec::new()
    }

    fn get_getter(&self, _arg_getters: &[Box<Any>]) -> Box<Any> {
        box GetterWrap::<T>::new(
            box self.clone()
        ) as Box<Any>
    }
}

impl<T: 'static> Getter<T> for Rc<RefCell<||:'static -> T>> {
    fn get(&self) -> T {
        (*(self.borrow_mut().deref_mut()))()
    }
}
