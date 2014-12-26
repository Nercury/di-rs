use metafactory::aggregate::Aggregate;
use metafactory::MetaFactory;
use typedef::TypeDef;

#[deriving(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct DefinitionCandidateKey {
    pub id: String,
    pub type_name: &'static str,
    pub collection_id: Option<String>,
}

/// Definition candidate.
pub struct DefinitionCandidate {
    pub metafactory: Box<MetaFactory + 'static>,
    pub arg_sources: Vec<String>,
}

impl DefinitionCandidateKey {
    pub fn new(
        id: &str,
        collection_id: Option<&str>,
        type_name: &'static str
    )
        -> DefinitionCandidateKey
    {
        DefinitionCandidateKey {
            id: id.to_string(),
            type_name: type_name,
            collection_id: match collection_id {
                Some(id) => Some(id.to_string()),
                None => None,
            },
        }
    }
}

impl DefinitionCandidate {
    pub fn new(
        metafactory: Box<MetaFactory + 'static>,
        arg_sources: Vec<String>
    )
        -> DefinitionCandidate
    {
        DefinitionCandidate {
            metafactory: metafactory,
            arg_sources: arg_sources,
        }
    }
}

/// Group candidate unique key.
#[deriving(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct GroupCandidateKey {
    pub collection_id: String,
}

impl GroupCandidateKey {
    pub fn new(collection_id: &str) -> GroupCandidateKey {
        GroupCandidateKey {
            collection_id: collection_id.to_string(),
        }
    }
}

/// Group candidate value.
#[allow(dead_code)]
pub struct GroupCandidate {
    pub collection_typedef: TypeDef,
    pub aggregate: Aggregate<'static>,
}

#[allow(dead_code)]
impl GroupCandidate {
    pub fn new(aggregate: Aggregate<'static>) -> GroupCandidate {
        GroupCandidate {
            collection_typedef: aggregate.get_container_type(),
            aggregate: aggregate,
        }
    }

    pub fn get_collection_typedef(&self) -> TypeDef {
        self.collection_typedef.clone()
    }

    pub fn take_collection_factory(self) -> Aggregate<'static> {
        self.aggregate
    }
}
