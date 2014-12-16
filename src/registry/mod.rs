use std::collections::{ BTreeSet, BTreeMap };

use metafactory::{ ToMetaFactory, MetaFactory };

use self::one_of::{ OneOf };
use self::one::{ One };
use self::group_candidate::{ GroupCandidateKey, GroupCandidate };
use self::definition_candidate::{ DefinitionCandidate };

mod group_candidate;
mod definition_candidate;

pub mod argument_builder;
pub mod one_of;
pub mod one;

pub struct Registry {
    maybe_groups: BTreeMap<GroupCandidateKey, GroupCandidate>,
    maybe_definitions: BTreeSet<DefinitionCandidate>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            maybe_groups: BTreeMap::new(),
            maybe_definitions: BTreeSet::new(),
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
        self.finalize_with_args_one(
            id,
            value.to_metafactory(),
            Vec::new()
        );
    }

    pub fn insert_with_args_one<T: 'static + ToMetaFactory>(&mut self, id: &str, arg_sources: &[&str], value: T) {
        self.finalize_with_args_one(
            id,
            value.to_metafactory(),
            arg_sources.iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    pub fn insert_with_arg_one<T: 'static + ToMetaFactory>(&mut self, id: &str, arg_source: &str, value: T) {
        self.finalize_with_args_one(
            id,
            value.to_metafactory(),
            [arg_source].iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    pub fn insert_one_of<T: 'static + ToMetaFactory>(&mut self, collection_id: &str, id: &str, value: T) {
        self.finalize_with_args_one_of(
            collection_id,
            id,
            value.to_metafactory(),
            Vec::new()
        );
    }

    pub fn insert_with_args_one_of<T: 'static + ToMetaFactory>(&mut self, collection_id: &str, id: &str, arg_sources: &[&str], value: T) {
        self.finalize_with_args_one_of(
            collection_id,
            id,
            value.to_metafactory(),
            arg_sources.iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    pub fn insert_with_arg_one_of<T: 'static + ToMetaFactory>(&mut self, collection_id: &str, id: &str, arg_source: &str, value: T) {
        self.finalize_with_args_one_of(
            collection_id,
            id,
            value.to_metafactory(),
            [arg_source].iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    fn finalize_with_args_one(&mut self, id: &str, value: Box<MetaFactory + 'static>, args: Vec<String>) {
        self.maybe_definitions.insert(
            DefinitionCandidate::new(
                id,
                None,
                value,
                args
            )
        );
    }

    fn finalize_with_args_one_of(&mut self, collection_id: &str, id: &str, value: Box<MetaFactory + 'static>, args: Vec<String>) {
        self.maybe_definitions.insert(
            DefinitionCandidate::new(
                id,
                Some(collection_id),
                value,
                args
            )
        );
    }
}
