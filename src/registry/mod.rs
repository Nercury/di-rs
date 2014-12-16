use std::collections::BTreeSet;

use typedef::TypeDef;
use metafactory::{ ToMetaFactory, MetaFactory };
//use self::definition::{ Definitions };

use self::one_of::{ OneOf };
use self::one::{ One };
use self::group_candidate::{ GroupCandidate };

mod group_candidate;

pub mod argument_builder;
//pub mod definition;
pub mod one_of;
pub mod one;

pub struct Registry {
    maybe_groups: BTreeSet<GroupCandidate>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry { maybe_groups: BTreeSet::new() }
    }

    pub fn one_of<'r, T: 'static + ToMetaFactory>(&'r mut self, collection_id: &str, id: &str, value: T)
        -> OneOf<'r>
    {
        let group_candidate = GroupCandidate::new::<T>(collection_id);
        if !self.maybe_groups.contains(&group_candidate) {
            self.maybe_groups.insert(group_candidate);
        }
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

    fn finalize_with_args_one<'r>(&mut self, _id: &str, _value: Box<MetaFactory + 'r>, _args: Vec<String>) {

    }

    fn finalize_with_args_one_of<'r>(&mut self, _collection_id: &str, _id: &str, _value: Box<MetaFactory + 'r>, _args: Vec<String>) {

    }
}
