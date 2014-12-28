use metafactory::aggregate::Aggregate;
use metafactory::MetaFactory;
use typedef::TypeDef;

/// Definition candidate.
pub struct DefinitionCandidate {
    pub metafactory: Box<MetaFactory + 'static>,
    pub arg_sources: Vec<String>,
    pub collection_id: Option<String>,
}

impl DefinitionCandidate {
    pub fn new(
        metafactory: Box<MetaFactory + 'static>,
        arg_sources: Vec<String>,
        collection_id: Option<String>
    )
        -> DefinitionCandidate
    {
        DefinitionCandidate {
            metafactory: metafactory,
            arg_sources: arg_sources,
            collection_id: collection_id,
        }
    }
}

/// Group candidate value.
pub struct GroupCandidate {
    pub collection_typedef: TypeDef,
    pub aggregate: Aggregate<'static>,
    pub member_count: u32,
}

impl GroupCandidate {
    pub fn new(aggregate: Aggregate<'static>) -> GroupCandidate {
        GroupCandidate {
            collection_typedef: aggregate.get_container_type(),
            aggregate: aggregate,
            member_count: 0,
        }
    }
}
