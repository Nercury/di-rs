use typedef::TypeDef;
use std::collections::BTreeMap;
use std::collections::btree_map::{ Occupied, Vacant };

use registry::definition_candidate::{ DefinitionCandidateKey, DefinitionCandidate };

#[deriving(Clone)]
pub struct Argument {
    pub typedef: TypeDef,
    pub source: String,
}

#[deriving(Clone)]
pub struct Definition {
    pub id: String,
    pub collection_id: Option<String>,
    pub typedef: TypeDef,
    pub args: Vec<Argument>,
}

impl Definition {
    fn from_key_and_candidate(
        key: &DefinitionCandidateKey,
        candidate: &DefinitionCandidate
    )
        -> Definition
    {
        Definition {
            id: key.id.clone(),
            collection_id: key.collection_id.clone(),
            typedef: candidate.metafactory.get_type(),
            args: arguments_from_candidate(candidate),
        }
    }
}

#[deriving(Clone)]
pub struct Duplicate {
    pub definition: Definition,
    pub count: uint,
}

fn arguments_from_candidate(candidate: &DefinitionCandidate) -> Vec<Argument> {
    candidate.metafactory.get_arg_types()
        .iter().zip(candidate.arg_sources.iter())
        .map(|&: (typedef, source)| Argument {
            typedef: typedef.clone(),
            source: source.clone(),
        })
        .collect()
}

fn argument_hash_for_candidate(candidate: &DefinitionCandidate) -> String {
    candidate.metafactory.get_arg_types()
        .iter().zip(candidate.arg_sources.iter())
        .map(|&: (typedef, source)|
            [source.as_slice(), typedef.get_str()].connect(":")
        )
        .fold(String::new(), |acc, i| {
            let mut result = acc;
            result.push_str(i.as_slice());
            result
        })
}

pub struct ArgumentCountMismatch {
    pub id: String,
    pub collection_id: Option<String>,
    pub typedef: TypeDef,
    pub arg_types: Vec<TypeDef>,
    pub arg_sources: Vec<String>,
}

impl ArgumentCountMismatch {
    pub fn new(
        key: &DefinitionCandidateKey,
        candidate: &DefinitionCandidate
    )
        -> ArgumentCountMismatch
    {
        ArgumentCountMismatch {
            id: key.id.clone(),
            collection_id: key.collection_id.clone(),
            typedef: candidate.metafactory.get_type(),
            arg_types: candidate.metafactory.get_arg_types(),
            arg_sources: candidate.arg_sources.clone(),
        }
    }
}

pub struct DuplicateDefinitions {
    pub aliases: BTreeMap<String, Duplicate>,
}

impl DuplicateDefinitions {
    pub fn new(
        key: &DefinitionCandidateKey,
        duplicates: &Vec<&DefinitionCandidate>
    )
        -> DuplicateDefinitions
    {
        let mut aliases = BTreeMap::<String, Duplicate>::new();

        for duplicate in duplicates.iter() {
            let hash = argument_hash_for_candidate(*duplicate);
            match aliases.entry(hash) {
                Vacant(entry) => {
                    entry.set(Duplicate {
                        definition: Definition::from_key_and_candidate(
                            key, *duplicate
                        ),
                        count: 1,
                    });
                },
                Occupied(mut entry) => {
                    entry.get_mut().count += 1;
                },
            }
        }

        DuplicateDefinitions {
            aliases: aliases,
        }
    }
}

pub enum CompileError {
    DuplicateDefinitions(DuplicateDefinitions),
    ArgumentCountMismatch(ArgumentCountMismatch),
}
