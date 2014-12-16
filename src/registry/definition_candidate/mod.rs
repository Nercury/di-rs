use typedef::TypeDef;
use metafactory::MetaFactory;

#[deriving(Ord, Eq, PartialEq, PartialOrd)]
struct DefinitionCandidateKey {
    id: String,
    type_name: &'static str,
    collection_id: Option<String>,
}

/// Definition candidate, must have unique id/type/collection combination.
pub struct DefinitionCandidate {
    key: DefinitionCandidateKey,
    metafactory: Box<MetaFactory + 'static>,
    arg_sources: Vec<String>,
}
ord_for!(DefinitionCandidate { key })

impl DefinitionCandidate {
    pub fn new(
        id: &str,
        collection_id: Option<&str>,
        metafactory: Box<MetaFactory + 'static>,
        arg_sources: Vec<String>
    )
        -> DefinitionCandidate
    {
        DefinitionCandidate {
            key: DefinitionCandidateKey {
                id: id.to_string(),
                type_name: metafactory.get_type().get_str(),
                collection_id: match collection_id {
                    Some(id) => Some(id.to_string()),
                    None => None,
                },
            },
            metafactory: metafactory,
            arg_sources: arg_sources,
        }
    }
}
