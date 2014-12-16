use std::collections::BTreeSet;

use typedef::TypeDef;

use super::super::factory_container::FactoryContainer;

struct GroupCandidateDefinition {
    id: String,
    typedef: TypeDef,
}
ord_for!(GroupCandidateDefinition { id })

/// Info about the group that might be added.
///
/// Most of info here is for making greatest runtime
/// errors... err, messages possible.
pub struct GroupCandidate {
    collection_id: String,
    collection_type_name: String,
    collection_typedef: TypeDef,
    definitions: BTreeSet<GroupCandidateDefinition>,
}
impl GroupCandidate {
    pub fn new<T:'static>(collection_id: &str) -> GroupCandidate {
        let collection_typedef = FactoryContainer::container_of::<T>();
        GroupCandidate {
            collection_id: collection_id.to_string(),
            collection_type_name: collection_typedef.get_str().to_string(),
            collection_typedef: collection_typedef,
            definitions: BTreeSet::new(),
        }
    }
}
ord_for!(GroupCandidate { collection_id, collection_type_name })
