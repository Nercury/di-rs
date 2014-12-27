use std::any::Any;
use std::collections::{ BTreeMap, BTreeSet, HashMap };
use std::collections::btree_map::{ Entry };

use metafactory::{ ToMetaFactory, MetaFactory };
use metafactory::aggregate::{ Aggregate };

use container::Container;

use self::new_definition::{ NewDefinition };
use self::candidate::{ GroupCandidate, DefinitionCandidate };
use self::error::{ CompileError, CircularDependency };

mod candidate;

pub mod argument_builder;
pub mod new_definition;
pub mod error;
pub mod validator;

pub struct Registry {
    /// Contains a list of group candidates.
    groups: BTreeMap<String, GroupCandidate>,
    /// Contains a list of definition candidates.
    definitions: BTreeMap<String, DefinitionCandidate>,
    /// Contains a list of definitions that were overriden while building
    /// the registry - so we can at least show some kind of warning.
    overriden_definitions: BTreeMap<String, Vec<DefinitionCandidate>>,

    validators: Vec<Box<validator::Validator + 'static>>,
}

impl Registry {
    pub fn new() -> Registry {
        let mut registry = Registry {
            groups: BTreeMap::new(),
            definitions: BTreeMap::new(),
            overriden_definitions: BTreeMap::new(),
            validators: Vec::new(),
        };

        registry.push_validator(validator::argument_count::ArgumentCountValidator);
        registry.push_validator(validator::overrides::NoOverridesValidator);
        registry.push_validator(validator::dependencies::DependencyValidator);

        registry
    }

    pub fn push_validator<T: validator::Validator + 'static>(
        &mut self,
        validator: T
    ) {
        self.validators.push(box validator);
    }

    fn create_missing_factory(
        &self,
        groups: &BTreeMap<String, BTreeSet<&str>>,
        factory_map: &mut HashMap<String, Box<Any>>,
        dependency_chain: &mut Vec<String>,
        id: &str
    ) -> Result<(), CircularDependency> {
        dependency_chain.push(id.to_string());

        // Find if this is a definition or a group.
        let mut result = match self.definitions.get(id) {
            Some(definition) => {
                // Check for circular dependencies.

                for source in definition.arg_sources.iter() {
                    if dependency_chain.contains(source) {
                        dependency_chain.push(source.clone());
                        return Err(CircularDependency::new(dependency_chain.clone()));
                    }
                }

                // Create definition factory.

                Some(self.create_definition_factory(
                    groups,
                    factory_map,
                    dependency_chain,
                    id,
                    definition
                ))
            },
            None => None,
        };

        if let None = result {
            result = match self.groups.get(id) {
                Some(group) => {
                    // Collect users of this group.
                    let group_dependencies = groups.get(id).expect(format!("expected to find dependencies for group {}", id).as_slice());

                    // Check for circular dependencies.
                    for source in group_dependencies.iter() {
                        if dependency_chain.contains(&source.to_string()) {
                            dependency_chain.push(source.to_string());
                            return Err(CircularDependency::new(dependency_chain.clone()));
                        }
                    }

                    Some(self.create_group_factory(
                        groups,
                        factory_map,
                        dependency_chain,
                        id,
                        group,
                        group_dependencies
                    ))
                },
                None => None,
            }
        }

        dependency_chain.pop();

        result.expect(format!("expected definition or group {} was not found", id).as_slice())
    }

    fn create_definition_factory(
        &self,
        groups: &BTreeMap<String, BTreeSet<&str>>,
        factory_map: &mut HashMap<String, Box<Any>>,
        dependency_chain: &mut Vec<String>,
        id: &str,
        definition: &DefinitionCandidate
    ) -> Result<(), CircularDependency> {

        for source in definition.arg_sources.iter() {
            let factory_contains_id = match factory_map.get(source) {
                None => false,
                _ => true,
            };
            if !factory_contains_id {
                match self.create_missing_factory(
                    groups,
                    factory_map,
                    dependency_chain,
                    source.as_slice()
                ) {
                    Err(error) => return Err(error),
                    _ => (),
                };
            }
        }

        Ok(())
    }

    fn create_group_factory(
        &self,
        groups: &BTreeMap<String, BTreeSet<&str>>,
        factory_map: &mut HashMap<String, Box<Any>>,
        dependency_chain: &mut Vec<String>,
        id: &str,
        group: &GroupCandidate,
        group_sources: &BTreeSet<&str>
    )
        -> Result<(), CircularDependency>
    {
        for source in group_sources.iter() {
            let factory_contains_id = match factory_map.get(&source.to_string()) {
                None => false,
                _ => true,
            };
            if !factory_contains_id {
                match self.create_missing_factory(
                    groups,
                    factory_map,
                    dependency_chain,
                    source.as_slice()
                ) {
                    Err(error) => return Err(error),
                    _ => (),
                };
            }
        }

        Ok(())
    }

    pub fn compile(&self) -> Result<Container, Vec<CompileError>> {
        let mut error_summary = Vec::<CompileError>::new();

        for validator in self.validators.iter() {
            validator.validate(self, &mut error_summary);
        }

        let mut factory_map = HashMap::<String, Box<Any>>::new();
        let groups = self.collect_group_dependencies();

        if error_summary.len() == 0 {
            for (id, definition) in self.definitions.iter() {
                let factory_contains_id = match factory_map.get(id) {
                    None => false,
                    _ => true,
                };
                if !factory_contains_id {
                    match self.create_missing_factory(
                        &groups,
                        &mut factory_map,
                        &mut Vec::<String>::new(),
                        id.as_slice()
                    ) {
                        Err(error) => error_summary.push(CompileError::CircularDependency(error)),
                        _ => (),
                    };
                }
            }
        }

        if error_summary.len() == 0 {
            Ok(Container::new(factory_map))
        } else {
            Err(error_summary)
        }
    }

    pub fn may_be_empty<T: 'static>(&mut self, collection_id: &str) {
        self.define_group_if_not_exists(collection_id, Aggregate::new::<T>());
    }

    pub fn one_of<'r, T: 'static + ToMetaFactory>(&'r mut self, collection_id: &str, value: T)
        -> NewDefinition<'r>
    {
        let mut id;
        let metafactory = value.to_metafactory();

        self.define_group_if_not_exists(collection_id, metafactory.new_aggregate());

        if let Some(group) = self.groups.get_mut(collection_id) {
            group.member_count += 1;
            id = format!("{}`{}", collection_id, group.member_count);
        } else {
            panic!("Expected to find defined group.")
        }

        NewDefinition::new(
            self,
            Some(collection_id.to_string()),
            id.as_slice(),
            metafactory
        )
    }

    pub fn one<'r, T: 'static + ToMetaFactory>(&'r mut self, id: &str, value: T)
        -> NewDefinition<'r>
    {
        NewDefinition::new(
            self,
            None,
            id,
            value.to_metafactory()
        )
    }

    pub fn insert_one<T: 'static + ToMetaFactory>(&mut self, id: &str, value: T) {
        self.define(
            None,
            id,
            value.to_metafactory(),
            Vec::new()
        );
    }

    fn collect_group_dependencies<'r>(&'r self) -> BTreeMap<String, BTreeSet<&'r str>> {
        let mut groups: BTreeMap<String, BTreeSet<&str>> = BTreeMap::new();

        for (id, value) in self.definitions.iter()
            .filter(|&(_, v)| v.collection_id != None)
        {
            match groups.entry(value.collection_id.clone().unwrap()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(id.as_slice());
                },
                Entry::Vacant(entry) => {
                    let mut set: BTreeSet<&str> = BTreeSet::new();
                    set.insert(id.as_slice());
                    entry.set(set);
                }
            }
        }

        groups
    }

    fn define_group_if_not_exists(&mut self, collection_id: &str, type_aggregate: Aggregate<'static>) {
        if !self.groups.contains_key(collection_id) {
            self.groups.insert(
                collection_id.to_string(),
                GroupCandidate::new(type_aggregate)
            );
        }
    }

    fn define(&mut self, collection_id: Option<String>, id: &str, value: Box<MetaFactory + 'static>, args: Vec<String>) {
        if let Some(overriden_candidate) = self.definitions.remove(id) {
            match self.overriden_definitions.entry(id.to_string()) {
                Entry::Vacant(entry) => { entry.set(vec![overriden_candidate]); },
                Entry::Occupied(mut entry) => { entry.get_mut().push(overriden_candidate); },
            };
        }

        let candidate = DefinitionCandidate::new(
            value,
            args,
            collection_id
        );

        self.definitions.insert(
            id.to_string(),
            candidate
        );
    }
}
