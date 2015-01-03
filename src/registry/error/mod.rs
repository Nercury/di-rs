/*!

Structures that contain detailed validation errors.

*/

use typedef::TypeDef;
use std::collections::{ VecMap, BTreeMap, HashSet };
use std::collections::btree_map::{ Entry };

use registry::candidate::{ DefinitionCandidate };

/// Possible compillation errors.
pub enum CompileError {
    DuplicateDefinitions(DuplicateDefinitions),
    ArgumentCountMismatch(ArgumentCountMismatch),
    DependenciesNotFound(DependenciesNotFound),
    IncorrectDepencencyTypes(IncorrectDepencencyTypes),
    CircularDependency(CircularDependency),
}

/// Definition argument with type and dependency name.
#[derive(Clone)]
pub struct Argument {
    pub typedef: TypeDef,
    pub source: String,
}

/// Definition information with id, collection id, type and arguments.
#[derive(Clone)]
pub struct Definition {
    pub id: String,
    pub collection_id: Option<String>,
    pub typedef: TypeDef,
    pub args: Vec<Argument>,
}

impl Definition {
    fn from_candidate(
        id: &str,
        candidate: &DefinitionCandidate
    )
        -> Definition
    {
        Definition {
            id: id.to_string(),
            collection_id: candidate.collection_id.clone(),
            typedef: candidate.metafactory.get_type(),
            args: arguments_from_candidate(candidate),
        }
    }
}

/// Information about duplicated definition.
#[derive(Clone)]
pub struct Duplicate {
    pub definition: Definition,
    pub count: uint,
}

/// Information about incorrect dependency types with lists of required types
/// and mismatched types.
pub struct IncorrectDepencencyTypes {
    pub id: String,
    pub collection_id: Option<String>,
    pub typedef: TypeDef,
    pub arg_types: Vec<TypeDef>,
    pub arg_sources: Vec<String>,
    pub mismatched_types: VecMap<TypeDef>
}

impl IncorrectDepencencyTypes {
    pub fn new(
        id: &str,
        collection_id: Option<String>,
        typedef: TypeDef,
        arg_types: Vec<TypeDef>,
        arg_sources: Vec<String>,
        mismatched_types: VecMap<TypeDef>
    )
        -> IncorrectDepencencyTypes
    {
        IncorrectDepencencyTypes {
            id: id.to_string(),
            collection_id: collection_id,
            typedef: typedef,
            arg_types: arg_types,
            arg_sources: arg_sources,
            mismatched_types: mismatched_types,
        }
    }
}

/// Circular dependency information.
///
/// Contains a dependency path where last item depends on some previous item.
pub struct CircularDependency {
    pub path: Vec<String>,
}

impl CircularDependency {
    pub fn new(
        path: Vec<String>
    )
        -> CircularDependency
    {
        CircularDependency {
            path: path,
        }
    }
}

/// Information about missing dependencies with definition id and missing
/// dependency names.
pub struct DependenciesNotFound {
    pub id: String,
    pub missing_dependencies: HashSet<String>,
}

impl DependenciesNotFound {
    pub fn new(
        id: &str,
        missing_dependencies: HashSet<String>
    )
        -> DependenciesNotFound
    {
        DependenciesNotFound {
            id: id.to_string(),
            missing_dependencies: missing_dependencies,
        }
    }
}

/// Information about argument count mismatch between definition and
/// specified dependency arguments.
pub struct ArgumentCountMismatch {
    pub id: String,
    pub collection_id: Option<String>,
    pub typedef: TypeDef,
    pub arg_types: Vec<TypeDef>,
    pub arg_sources: Vec<String>,
}

impl ArgumentCountMismatch {
    pub fn new(
        id: &str,
        candidate: &DefinitionCandidate
    )
        -> ArgumentCountMismatch
    {
        ArgumentCountMismatch {
            id: id.to_string(),
            collection_id: candidate.collection_id.clone(),
            typedef: candidate.metafactory.get_type(),
            arg_types: candidate.metafactory.get_arg_types(),
            arg_sources: candidate.arg_sources.clone(),
        }
    }
}

/// List of duplicate definitions.
pub struct DuplicateDefinitions {
    pub aliases: BTreeMap<String, Duplicate>,
}

impl DuplicateDefinitions {
    pub fn new(
        id: &str,
        duplicates: &Vec<&DefinitionCandidate>
    )
        -> DuplicateDefinitions
    {
        let mut aliases = BTreeMap::<String, Duplicate>::new();

        for duplicate in duplicates.iter() {
            let hash = argument_hash_for_candidate(*duplicate);
            match aliases.entry(hash) {
                Entry::Vacant(entry) => {
                    entry.set(Duplicate {
                        definition: Definition::from_candidate(
                            id, *duplicate
                        ),
                        count: 1,
                    });
                },
                Entry::Occupied(mut entry) => {
                    entry.get_mut().count += 1;
                },
            }
        }

        DuplicateDefinitions {
            aliases: aliases,
        }
    }
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
    candidate.metafactory.get_arg_types().iter().zip(candidate.arg_sources.iter())
        .map(|&: (typedef, source)|
            [source.as_slice(), typedef.get_str()].connect(":")
        )
        .fold(String::new(), |acc, i| {
            let mut result = acc;
            result.push_str(i.as_slice());
            result
        })
}
