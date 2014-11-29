use std::boxed::BoxAny;
use std::any::Any;

use super::{ Registry };
use super::super::definition::{ Definition };
use super::super::getter::{ GetterWrap };

pub struct RegistryItemCandidate<'a> {
    name: String,
    registry: &'a mut Registry<'a>,
    registry_item: RegistryItem<'a>,
}

impl<'a> RegistryItemCandidate<'a> {
    pub fn new(
        name: &str,
        registry: &'a mut Registry<'a>,
        registry_item: RegistryItem<'a>
    )
        -> RegistryItemCandidate<'a>
    {
        RegistryItemCandidate {
            name: name.to_string(),
            registry: registry,
            registry_item: registry_item,
        }
    }

    pub fn args(
        mut self,
        arg_sources: &[&str]
    )
        -> RegistryItemCandidate<'a>
    {
        self.registry_item.set_arg_sources(arg_sources);
        self
    }

    pub fn insert(self) -> &'a mut Registry<'a> {
        self.registry.insert_single(self.name.as_slice(), self.registry_item);
        self.registry
    }
}

pub struct RegistryItem<'a> {
    pub definition: Box<Definition + 'a>,
    pub arg_sources: Vec<String>,
}

impl<'a> RegistryItem<'a> {
    pub fn new(definition: Box<Definition + 'a>) -> RegistryItem<'a> {
        RegistryItem {
            definition: definition,
            arg_sources: Vec::new(),
        }
    }

    pub fn set_arg_sources<'r>(&'r mut self, arg_sources: &[&str]) {
        self.arg_sources.truncate(0);
        for str in arg_sources.iter() {
            self.arg_sources.push(str.to_string());
        }
    }

    pub fn get_getter<T: 'static>(
        &self,
        arg_getters: Vec<Box<Any>>
    )
        -> Option<GetterWrap<T>>
    {
        let maybe_getter = self.definition.get_getter(arg_getters);
        match maybe_getter.downcast::<GetterWrap<T>>() {
            Ok(getter) => Some(*getter),
            Err(_) => None,
        }
    }
}
