use typedef::TypeDef;

use metafactory::aggregate::Aggregate;

/// Group candidate unique key.
#[deriving(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct GroupCandidateKey {
    pub collection_id: String,
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
#[allow(dead_code)]
pub struct GroupCandidate {
    pub collection_typedef: TypeDef,
    pub factory: Aggregate<'static>,
}

#[allow(dead_code)]
impl GroupCandidate {
    pub fn new<T:'static>() -> GroupCandidate {
        let collection_typedef = Aggregate::container_of::<T>();
        GroupCandidate {
            collection_typedef: collection_typedef,
            factory: Aggregate::new::<T>(),
        }
    }

    pub fn get_collection_typedef(&self) -> TypeDef {
        self.collection_typedef.clone()
    }

    pub fn take_collection_factory(self) -> Aggregate<'static> {
        self.factory
    }
}
