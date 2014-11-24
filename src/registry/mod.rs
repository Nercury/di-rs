use std::collections::HashMap;

use super::definition::{ Definition, ToDefinition, TypeDef };
use super::getter::{ GetterWrap };

use self::getter_err::{
    GetterErr,
    GetterErrKind,
    DefinitionTypeErr,
    GetterTypeErr
};
use self::item::{ RegistryItem, RegistryItemCandidate };

pub mod getter_err;
pub mod item;

pub struct Registry<'a> {
    items: HashMap<String, RegistryItem<'a>>,
}

impl<'a> Registry<'a> {
    pub fn new() -> Registry<'a> {
        Registry {
            items: HashMap::new(),
        }
    }

    pub fn one<T: ToDefinition>(
        &'a mut self,
        name: &str,
        source: T
    )
        -> RegistryItemCandidate<'a>
    {
        RegistryItemCandidate::new(
            name,
            self,
            RegistryItem::new(source.to_definition())
        )
    }

    pub fn insert<'r>(
        &'r mut self,
        name: &str,
        registry_item: RegistryItem<'a>
    ) {
        self.items.insert(name.to_string(), registry_item);
    }

    pub fn getter_for<T: 'static>(
        &self, name: &str
    )
        -> GetterWrap<T>
    {
        let maybe_getter = self.maybe_getter_for::<T>(name);
        match maybe_getter {
            Ok(getter) => getter,
            Err(err) => {
                panic!(err.to_string())
            }
        }
    }

    pub fn maybe_getter_for<T: 'static>(
        &self, name: &str
    )
        -> Result<GetterWrap<T>, GetterErr>
    {
        self.inner_getter_construct_for::<T>(name, Vec::new())
    }

    fn inner_getter_construct_for<T: 'static>(
        &self,
        name: &str,
        success_path: Vec<String>
    )
        -> Result<GetterWrap<T>, GetterErr>
    {
        let maybe_item = self.items.get(&name.to_string());
        match maybe_item {
            Some(item) => {
                let def_type = item.definition.get_type();
                match def_type.is::<T>() {
                    true    => {
                        let arg_types = item.definition.get_arg_types();

                        if arg_types.len() != item.arg_sources.len() {
                            return Err(GetterErr::new(
                                GetterErrKind::GetterTypeMismatch(GetterTypeErr::new(TypeDef::of::<GetterWrap<T>>())),
                                name,
                                success_path
                            ))
                        }

                        match item.get_getter::<T>(&[]) {
                            Some(getter) => Ok(getter),
                            None => Err(GetterErr::new(
                                GetterErrKind::GetterTypeMismatch(GetterTypeErr::new(TypeDef::of::<GetterWrap<T>>())),
                                name,
                                success_path
                            )),
                        }
                    },
                    false   => {
                        Err(GetterErr::new(
                            GetterErrKind::DefinitionTypeMismatch(DefinitionTypeErr::new(TypeDef::of::<T>(), def_type)),
                            name,
                            success_path
                        ))
                    },
                }
            }
            None => Err(GetterErr::new(GetterErrKind::NotFound, name, success_path))
        }
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use super::{ Registry };
    use super::super::definition::{ Definition, TypeDef };
    use super::super::getter::{ GetterWrap };
    use super::getter_err::{ GetterErr, GetterErrKind };
    use super::item::{ RegistryItem };

    /// Lies that is int, but it returns float.
    struct IntLie;

    impl Definition for IntLie {
        fn get_type(&self) -> TypeDef {
            TypeDef::of::<int>()
        }

        fn get_arg_types<'r>(&'r self) -> Vec<TypeDef> {
            Vec::new()
        }

        fn get_getter(&self, _arg_getters: &[Box<Any>]) -> Box<Any> {
            box GetterWrap::new(
                box 16f32
            ) as Box<Any>
        }
    }

    #[test]
    fn returns_not_found() {
        let err = one_static_and_expect_getter_err::
        <
            i8, // define
            i8  // try to get
        >
        (
            "a", // definition name
            15,  // definition value
            "b"  // try to get name
        );
        match err.kind {
            GetterErrKind::NotFound => {},
            _ => panic!(
                "expected GetterErrKind::NotFound, but got => {}",
                err.to_string()
            ),
        }
    }

    #[test]
    fn returns_type_mismatch() {
        let err = one_static_and_expect_getter_err::
            <
                i8, // define
                u8  // try to get
            >
        (
            "a", // definition name
            15,  // definition value
            "a"  // try to get name
        );
        match err.kind {
            GetterErrKind::DefinitionTypeMismatch(_) => {},
            _ => panic!(
                "expected GetterErrKind::DefinitionTypeMismatch, but got => {}",
                err.to_string()
            ),
        }
    }

    #[test]
    fn returns_getter_mismatch() {
        let mut registry = Registry::new();
        registry.insert("a", RegistryItem::new(box IntLie));

        let err = registry.maybe_getter_for::<int>("a")
            .err()
            .unwrap();

        match err.kind {
            GetterErrKind::GetterTypeMismatch(_) => {},
            _ => panic!(
                "expected GetterErrKind::GetterTypeMismatch, but got => {}",
                err.to_string()
            ),
        }
    }

    fn one_static_and_expect_getter_err
    <
        OneT: 'static + Clone,
        GetterT: 'static + Clone
    >
    (
        one_name: &str,
        one_def: OneT,
        get_name: &str
    )
        -> GetterErr
    {
        let mut registry = Registry::new();
        registry
            .one::<OneT>(one_name, one_def)
            .insert()
            .maybe_getter_for::<GetterT>(get_name)
            .err()
            .unwrap()
    }
}
