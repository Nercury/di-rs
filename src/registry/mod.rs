use std::any::Any;
use std::collections::{ BTreeMap, HashMap };
use std::collections::btree_map::{ Occupied, Vacant };

use metafactory::{ ToMetaFactory, MetaFactory };

use super::container::Container;

use self::one_of::{ OneOf };
use self::one::{ One };
use self::group_candidate::{ GroupCandidateKey, GroupCandidate };
use self::definition_candidate::{ DefinitionCandidateKey, DefinitionCandidate };
use self::error::{ CompileError, DuplicateDefinitions };

mod group_candidate;
mod definition_candidate;

pub mod argument_builder;
pub mod one_of;
pub mod one;
pub mod error;
pub mod validator;

pub struct Registry {
    /// Contains a list of group candidates that are unique for
    /// id+type.
    maybe_groups: BTreeMap<GroupCandidateKey, GroupCandidate>,
    /// Contains a list of definition candidates that are unique for
    /// id+collection_id+type.
    maybe_definitions: BTreeMap<DefinitionCandidateKey, DefinitionCandidate>,
    /// Contains a list of definitions that were overriden while building
    /// the registry - so we can at least show some kind of warning.
    overriden_definitions: BTreeMap<DefinitionCandidateKey, Vec<DefinitionCandidate>>,

    validators: Vec<Box<validator::Validator + 'static>>,
}

impl Registry {
    pub fn new() -> Registry {
        let mut validators = Vec::new();

        let mut registry = Registry {
            maybe_groups: BTreeMap::new(),
            maybe_definitions: BTreeMap::new(),
            overriden_definitions: BTreeMap::new(),
            validators: validators,
        };

        registry.push_validator(validator::overrides::NoOverridesValidator);

        registry
    }

    pub fn push_validator<T: validator::Validator + 'static>(
        &mut self,
        validator: T
    ) {
        self.validators.push(box validator);
    }

    pub fn compile(&self) -> Result<Container, Vec<CompileError>> {
        let mut error_summary = Vec::<CompileError>::new();

        for validator in self.validators.iter() {
            validator.validate(self, &mut error_summary);
        }

        let factory_map = HashMap::<String, Box<Any>>::new();

        if error_summary.len() == 0 {
            Ok(Container::new(factory_map))
        } else {
            Err(error_summary)
        }
    }

    pub fn has_many<T: 'static>(&mut self, collection_id: &str) {
        let group_candidate_key = GroupCandidateKey::new::<T>(collection_id);
        if !self.maybe_groups.contains_key(&group_candidate_key) {
            self.maybe_groups.insert(
                group_candidate_key,
                GroupCandidate::new::<T>()
            );
        }
    }

    pub fn one_of<'r, T: 'static + ToMetaFactory>(&'r mut self, collection_id: &str, id: &str, value: T)
        -> OneOf<'r>
    {
        self.has_many::<T>(collection_id);

        OneOf::new(
            self,
            collection_id,
            id,
            value.to_metafactory()
        )
    }

    pub fn one<'r, T: 'static + ToMetaFactory>(&'r mut self, id: &str, value: T)
        -> One<'r>
    {
        One::new(
            self,
            id,
            value.to_metafactory()
        )
    }

    pub fn insert_one<T: 'static + ToMetaFactory>(&mut self, id: &str, value: T) {
        self.finalize(
            None,
            id,
            value.to_metafactory(),
            Vec::new()
        );
    }

    pub fn insert_with_args_one<T: 'static + ToMetaFactory>(&mut self, id: &str, arg_sources: &[&str], value: T) {
        self.finalize(
            None,
            id,
            value.to_metafactory(),
            arg_sources.iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    pub fn insert_with_arg_one<T: 'static + ToMetaFactory>(&mut self, id: &str, arg_source: &str, value: T) {
        self.finalize(
            None,
            id,
            value.to_metafactory(),
            [arg_source].iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    pub fn insert_one_of<T: 'static + ToMetaFactory>(&mut self, collection_id: &str, id: &str, value: T) {
        self.finalize(
            Some(collection_id),
            id,
            value.to_metafactory(),
            Vec::new()
        );
    }

    pub fn insert_with_args_one_of<T: 'static + ToMetaFactory>(&mut self, collection_id: &str, id: &str, arg_sources: &[&str], value: T) {
        self.finalize(
            Some(collection_id),
            id,
            value.to_metafactory(),
            arg_sources.iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    pub fn insert_with_arg_one_of<T: 'static + ToMetaFactory>(&mut self, collection_id: &str, id: &str, arg_source: &str, value: T) {
        self.finalize(
            Some(collection_id),
            id,
            value.to_metafactory(),
            [arg_source].iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    fn finalize(&mut self, collection_id: Option<&str>, id: &str, value: Box<MetaFactory + 'static>, args: Vec<String>) {
        let candidate_key = DefinitionCandidateKey::new(
            id,
            collection_id,
            value.get_type().get_str()
        );

        if let Some(overriden_candidate) = self.maybe_definitions.remove(&candidate_key) {
            match self.overriden_definitions.entry(candidate_key.clone()) {
                Vacant(entry) => { entry.set(vec![overriden_candidate]); },
                Occupied(mut entry) => { entry.get_mut().push(overriden_candidate); },
            };
        }

        let candidate = DefinitionCandidate::new(
            value,
            args
        );

        self.maybe_definitions.insert(
            candidate_key,
            candidate
        );
    }
}
