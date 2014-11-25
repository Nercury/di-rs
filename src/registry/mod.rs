use std::collections::HashMap;
use std::any::Any;
use std::boxed::BoxAny;

use super::definition::{ Definition, ToDefinition, TypeDef };
use super::getter::{ GetterWrap };

use self::getter_err::{
    GetterErr,
    GetterErrKind,
    DefinitionTypeErr,
    ArgTypeErr,
    GetterTypeErr,
    ArgCountMismatchErr
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
        let result = self.inner_getter_construct_for(name, TypeDef::of::<T>());
        match result {
            Ok(boxed_getter) => match boxed_getter.downcast::<GetterWrap<T>>() {
                Ok(getter) => Ok(*getter),
                Err(_) => Err(GetterErr::new(
                    GetterErrKind::GetterTypeMismatch(GetterTypeErr::new(TypeDef::of::<GetterWrap<T>>())),
                    name
                )),
            },
            Err(e) => Err(e),
        }
    }

    fn inner_getter_construct_for(
        &self,
        name: &str,
        required_type: TypeDef
    )
        -> Result<Box<Any>, GetterErr>
    {
        let maybe_item = self.items.get(&name.to_string());
        match maybe_item {
            Some(item) => {
                let def_type = item.definition.get_type();
                match def_type == required_type {
                    true    => {
                        let arg_types = item.definition.get_arg_types();

                        if arg_types.len() != item.arg_sources.len() {
                            return Err(GetterErr::new(
                                GetterErrKind::ArgCountMismatch(ArgCountMismatchErr::new(arg_types.len(), item.arg_sources.len())),
                                name
                            ))
                        }

                        let mut arg_getters: Vec<Box<Any>> = Vec::with_capacity(arg_types.len());

                        for (source_name, arg_type) in item.arg_sources.iter().zip(arg_types.iter()) {

                            let maybe_arg_getter = self.inner_getter_construct_for(
                                source_name.as_slice(),
                                arg_type.clone()
                            );

                            match maybe_arg_getter {
                                Ok(boxed_getter) => arg_getters.push(boxed_getter),
                                Err(e) => return match e.kind {
                                    GetterErrKind::NotFound => {
                                        Err(GetterErr::new(
                                            GetterErrKind::ArgNotFound(e.name),
                                            name
                                        ))
                                    },
                                    GetterErrKind::DefinitionTypeMismatch(type_err) => {
                                        Err(GetterErr::new(
                                            GetterErrKind::ArgTypeMismatch(
                                                ArgTypeErr::new(
                                                    e.name.as_slice(),
                                                    type_err.requested,
                                                    type_err.found
                                                )
                                            ),
                                            name
                                        ))
                                    },
                                    _ => Err(e)
                                },
                            }
                        }

                        Ok(item.definition.get_getter(arg_getters))
                    },
                    false   => {
                        Err(GetterErr::new(
                            GetterErrKind::DefinitionTypeMismatch(DefinitionTypeErr::new(required_type, def_type)),
                            name
                        ))
                    },
                }
            }
            None => Err(GetterErr::new(GetterErrKind::NotFound, name))
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
