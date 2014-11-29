use std::collections::HashMap;
use std::any::Any;
use std::boxed::BoxAny;

use super::definition::{ Definition, ToDefinition, TypeDef };
use super::getter::{ GetterWrap };

use self::getter_err::{
    GetterErr,
    GetterErrKind,
    DefinitionTypeErr,
    GetterTypeErr,
    ArgCountMismatchErr
};
use self::item::{ RegistryItem, RegistryItemCandidate };

pub mod getter_err;
pub mod item;

pub struct Registry<'a> {
    single_items: HashMap<String, RegistryItem<'a>>,
}

impl<'a> Registry<'a> {
    /// Create a new registry.
    pub fn new() -> Registry<'a> {
        Registry {
            single_items: HashMap::new(),
        }
    }

    /// Define a single instance of named value generator.
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

    /// Insert a custom registry definition.
    pub fn insert_single<'r>(
        &'r mut self,
        name: &str,
        registry_item: RegistryItem<'a>
    ) {
        self.single_items.insert(name.to_string(), registry_item);
    }

    /// Create a getter for a definition.
    pub fn getter_for<T: 'static>(
        &self, name: &str
    )
        -> Result<GetterWrap<T>, GetterErr>
    {
        let result = self.any_typechecked_getter_for(name, TypeDef::of::<T>());
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

    /// Returns all defined getter names.
    pub fn all_names(&self) -> Vec<String> {
        self.single_items.keys().map(|v| v.clone()).collect::<Vec<String>>()
    }

    /// Create anytyped getter for a definition.
    pub fn any_typechecked_getter_for(
        &self,
        name: &str,
        required_type: TypeDef
    )
        -> Result<Box<Any>, GetterErr>
    {
        let maybe_getter = self.any_getter_for(name);
        match maybe_getter {
            Ok(getter) => {
                let item = self.single_items.get(&name.to_string()).unwrap();
                let def_type = item.definition.get_type();
                match def_type == required_type {
                    true    => Ok(getter),
                    false   => Err(
                        GetterErr::new(
                            GetterErrKind::DefinitionTypeMismatch(DefinitionTypeErr::new(required_type, def_type)),
                            name
                        )
                    ),
                }
            }
            Err(e) => Err(e)
        }
    }

    /// Create anytyped getter for a definition.
    pub fn any_getter_for(
        &self,
        name: &str
    )
        -> Result<Box<Any>, GetterErr>
    {
        let maybe_item = self.single_items.get(&name.to_string());
        match maybe_item {
            Some(item) => {
                match self.any_getters_for_arguments(
                    name,
                    &item.arg_sources,
                    &item.definition.get_arg_types()
                ) {
                    Ok(arg_getters) => Ok(item.definition.get_getter(arg_getters)),
                    Err(e) => Err(e),
                }
            }
            None => Err(GetterErr::new(GetterErrKind::NotFound, name))
        }
    }

    /// Create anytyped getters for definition arguments.
    fn any_getters_for_arguments(
        &self,
        parent_name: &str,
        arg_sources: &Vec<String>,
        arg_types: &Vec<TypeDef>
    )
        -> Result<Vec<Box<Any>>, GetterErr>
    {
        if arg_types.len() != arg_sources.len() {
            return Err(GetterErr::new(
                GetterErrKind::ArgCountMismatch(ArgCountMismatchErr::new(arg_types.len(), arg_sources.len())),
                parent_name
            ))
        }

        let mut arg_getters: Vec<Box<Any>> = Vec::with_capacity(arg_types.len());

        for (source_name, arg_type) in arg_sources.iter().zip(arg_types.iter()) {
            let maybe_arg_getter = self.any_getter_for_argument(
                parent_name,
                source_name.as_slice(),
                arg_type.clone()
            );

            match maybe_arg_getter {
                Ok(boxed_getter) => arg_getters.push(boxed_getter),
                Err(e) => return Err(e),
            }
        }

        Ok(arg_getters)
    }

    /// Construct an anytyped getter for definition in case the deffinition is
    /// an argument. This simply modifies error if it occurs to include parent
    /// name.
    fn any_getter_for_argument(
        &self,
        parent_name: &str,
        name: &str,
        required_type: TypeDef
    )
        -> Result<Box<Any>, GetterErr>
    {
        match self.any_typechecked_getter_for(name, required_type) {
            Ok(boxed_getter) => Ok(boxed_getter),
            Err(e) => Err(e.to_arg_error(parent_name)),
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
