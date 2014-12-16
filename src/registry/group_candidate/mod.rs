use typedef::TypeDef;

use super::super::factory_container::FactoryContainer;

/// Group candidate unique key.
#[deriving(Ord, Eq, PartialEq, PartialOrd)]
pub struct GroupCandidateKey {
    collection_id: String,
    contained_type_name: &'static str,
}

impl GroupCandidateKey {
    pub fn new<T: 'static>(collection_id: &str) -> GroupCandidateKey {
        GroupCandidateKey {
            collection_id: collection_id.to_string(),
            contained_type_name: TypeDef::name_of::<T>(),
        }
    }
}

/// Group candidate value.
pub struct GroupCandidate {
    collection_typedef: TypeDef,
    factory: FactoryContainer<'static>,
}

impl GroupCandidate {
    pub fn new<T:'static>() -> GroupCandidate {
        let collection_typedef = FactoryContainer::container_of::<T>();
        GroupCandidate {
            collection_typedef: collection_typedef,
            factory: FactoryContainer::new::<T>(),
        }
    }

    pub fn get_collection_typedef(&self) -> TypeDef {
        self.collection_typedef.clone()
    }

    pub fn take_collection_factory(self) -> FactoryContainer<'static> {
        self.factory
    }
}
