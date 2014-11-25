#![unstable]

use std::intrinsics::TypeId;
use std::intrinsics::get_tydesc;
use std::any::{ Any };

mod clone;
mod closure_zeroarg;
mod closure_manyarg;

#[stable]
#[deriving(Clone)]
pub struct TypeDef {
    type_id: TypeId,
    type_name: &'static str,
}

#[stable]
impl TypeDef {
    #[stable]
    pub fn of<T: 'static>() -> TypeDef {
        TypeDef {
            type_id: TypeId::of::<T>(),
            type_name: unsafe { (*get_tydesc::<T>()).name },
        }
    }

    #[stable]
    pub fn is<T: 'static>(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }

    #[stable]
    pub fn get_name(&self) -> &'static str {
        self.type_name
    }
}

#[unstable]
impl PartialEq for TypeDef {
    #[stable]
    fn eq(&self, other: &TypeDef) -> bool {
        self.type_id == other.type_id
    }
}

#[unstable]
pub trait Definition {
    #[unstable]
    fn get_type(&self) -> TypeDef;
    #[unstable]
    fn get_arg_types(&self) -> Vec<TypeDef>;
    #[unstable]
    fn get_getter(&self, arg_getters: Vec<Box<Any>>) -> Box<Any>;
}

/// This trait is implemented for values that can be used as
/// sources for object creation.
#[unstable]
pub trait ToDefinition {
    /// Creates a definition that has information about object
    /// creation factory: produced object type, argument types, and
    /// a method that can build object getter.
    #[unstable]
    fn to_definition<'a>(self) -> Box<Definition + 'a>;
}
