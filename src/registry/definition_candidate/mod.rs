use typedef::TypeDef;
use metafactory::MetaFactory;

#[deriving(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct DefinitionCandidateKey {
    id: String,
    type_name: &'static str,
    collection_id: Option<String>,
}

/// Definition candidate.
pub struct DefinitionCandidate {
    metafactory: Box<MetaFactory + 'static>,
    arg_sources: Vec<String>,
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
