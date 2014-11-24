use std::any::{ Any };

use super::{ Definition, ToDefinition, TypeDef };
use super::super::getter::{ GetterWrap, Getter };

struct ClonedDef<T> {
    value: T,
}

/// Creates `Definition` from cloneable value.
#[stable]
impl<T: 'static + Clone> ToDefinition for T {
    fn to_definition<'a>(self) -> Box<Definition + 'a> {
        // We wrap this one in a ClonedDef just to avoid conflicting
        // with definitions that need simple Cloned T for other purposes,
        // like Rc.
        box ClonedDef { value : self }
    }
}

impl<T: 'static + Clone> Definition for ClonedDef<T> {
    fn get_type(&self) -> TypeDef {
        TypeDef::of::<T>()
    }

    fn get_arg_types(&self) -> Vec<TypeDef> {
        Vec::new()
    }

    fn get_getter(&self, _arg_getters: &[Box<Any>]) -> Box<Any> {
        box GetterWrap::new(
            box self.value.clone()
        ) as Box<Any>
    }
}

impl<T: 'static + Clone> Getter<T> for T {
    fn get(&self) -> T {
        self.clone()
    }
}
