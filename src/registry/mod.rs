use std::kinds::marker;
use typedef::TypeDef;
use metafactory::{ ToMetaFactory, MetaFactory };
//use self::definition::{ Definitions };
use self::one_of::{ OneOf };
use self::one::{ One };

pub mod argument_builder;
//pub mod definition;
pub mod one_of;
pub mod one;

struct GroupCandidate;

pub struct Registry {
    _marker: marker::NoCopy,
}

impl Registry {
    pub fn new() -> Registry {
        Registry { _marker: marker::NoCopy }
    }

    pub fn one_of<'r, T: ToMetaFactory>(&'r mut self, collection_id: &str, id: &str, value: T)
        -> OneOf<'r>
    {
        //let type_of_group = TypeDef::name_of::<T>();
        OneOf::new(
            self,
            collection_id,
            id,
            value.to_metafactory()
        )
    }

    pub fn one<'r, T: ToMetaFactory>(&'r mut self, id: &str, value: T)
        -> One<'r>
    {
        One::new(
            self,
            id,
            value.to_metafactory()
        )
    }

    pub fn insert_one<T: ToMetaFactory>(&mut self, id: &str, value: T) {
        self.finalize_with_args_one(
            id,
            value.to_metafactory(),
            Vec::new()
        );
    }

    pub fn insert_with_args_one<T: ToMetaFactory>(&mut self, id: &str, arg_sources: &[&str], value: T) {
        self.finalize_with_args_one(
            id,
            value.to_metafactory(),
            arg_sources.iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    pub fn insert_with_arg_one<T: ToMetaFactory>(&mut self, id: &str, arg_source: &str, value: T) {
        self.finalize_with_args_one(
            id,
            value.to_metafactory(),
            [arg_source].iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    pub fn insert_one_of<T: ToMetaFactory>(&mut self, collection_id: &str, id: &str, value: T) {
        self.finalize_with_args_one_of(
            collection_id,
            id,
            value.to_metafactory(),
            Vec::new()
        );
    }

    pub fn insert_with_args_one_of<T: ToMetaFactory>(&mut self, collection_id: &str, id: &str, arg_sources: &[&str], value: T) {
        self.finalize_with_args_one_of(
            collection_id,
            id,
            value.to_metafactory(),
            arg_sources.iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    pub fn insert_with_arg_one_of<T: ToMetaFactory>(&mut self, collection_id: &str, id: &str, arg_source: &str, value: T) {
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
