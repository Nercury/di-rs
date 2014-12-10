use metafactory::{ ToMetaFactory, MetaFactory };
use self::one_of::{ OneOf };

pub mod argument_builder;
pub mod one_of;

pub struct Registry;

impl Registry {
    pub fn new() -> Registry {
        Registry
    }

    pub fn one_of<'r, T: ToMetaFactory>(&'r mut self, collection_id: &str, id: &str, value: T)
        -> OneOf<'r>
    {
        OneOf::new(
            self,
            collection_id,
            id,
            value.to_metafactory()
        )
    }

    fn insert_with_args_one_of<'r>(&mut self, _collection_id: &str, _id: &str, _value: Box<MetaFactory + 'r>, _args: Vec<String>) {
        
    }
}
