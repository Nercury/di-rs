use std::any::{ Any };

use super::{ Definition, ToDefinition, TypeDef };
use super::super::getter::{ GetterWrap, Getter };

/// Creates `Definition` from cloneable value.
#[stable]
impl<T: 'static + Clone> ToDefinition for T {
    fn to_definition<'a>(self) -> Box<Definition + 'a> {
        box self
    }
}

impl<T: 'static + Clone> Definition for T {
    fn get_type(&self) -> TypeDef {
        TypeDef::of::<T>()
    }

    fn get_arg_types(&self) -> Vec<TypeDef> {
        Vec::new()
    }

    fn get_getter(&self, _arg_getters: &[Box<Any>]) -> Box<Any> {
        box GetterWrap::new(
            box self.clone()
        ) as Box<Any>
    }
}

impl<T: 'static + Clone> Getter<T> for T {
    fn get(&self) -> T {
        self.clone()
    }
}
