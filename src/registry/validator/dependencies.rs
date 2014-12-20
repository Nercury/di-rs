use std::collections::{ HashMap, HashSet };
use std::collections::hash_map::{ Occupied, Vacant };
use typedef::TypeDef;

use registry::error;
use registry::Registry;
use registry::definition_candidate::DefinitionCandidate;
use registry::group_candidate::GroupCandidate;

use super::Validator;

#[deriving(Copy)]
pub struct DependencyValidator;

struct DefinitionRequirements<'a> {
    typedef: TypeDef,
    arguments: HashMap<&'a str, TypeDef>,
}

impl Validator for DependencyValidator {
    fn validate(&self, registry: &Registry, error_summary: &mut Vec<error::CompileError>) {
        // Collect group_id -> [ child_id ] map.

        let mut groups: HashMap<String, HashSet<&str>> = HashMap::new();
        for (key, definition) in registry.maybe_definitions.iter()
            .filter(|&(k, d)| k.collection_id != None)
        {
            match groups.entry(key.collection_id.clone().unwrap()) {
                Occupied(mut entry) => {
                    entry.get_mut().insert(key.id.as_slice());
                },
                Vacant(entry) => {
                    let mut set: HashSet<&str> = HashSet::new();
                    set.insert(key.id.as_slice());
                    entry.set(set);
                }
            }
        }

        // Collect all_definition_id -> required definition_id + type map

        let definitions: HashMap<&str, DefinitionRequirements> = registry.maybe_definitions.iter()
            .map(|(k, c)|
                (k.id.as_slice(), DefinitionRequirements {
                    typedef: c.metafactory.get_type(),
                    arguments: c.arg_sources.iter()
                        .map(|s| s.as_slice())
                        .zip(
                            c.metafactory.get_arg_types().iter()
                            .map(|td| td.clone())
                        )
                        .collect(),
                })
            )
            .chain(
                registry.maybe_groups.iter()
                .map(|(k, g)|
                    (k.collection_id.as_slice(), DefinitionRequirements {
                        typedef: g.collection_typedef,
                        arguments:
                            if groups.contains_key(&k.collection_id) {
                                groups.get(&k.collection_id).unwrap()
                                    .iter()
                                    .map(|arg_source| (*arg_source, g.factory.get_arg_type()))
                                    .collect()
                            } else {
                                HashMap::new()
                            }
                        ,
                    })
                )
            )
            .collect();

        let mut missing_dependencies = HashSet::<String>::new();

        for (id, requirements) in definitions.iter() {
            missing_dependencies.clear();

            for (arg_id, arg_type) in requirements.arguments.iter() {
                if !definitions.contains_key(arg_id) {
                    missing_dependencies.insert(arg_id.to_string());
                }
            }

            if missing_dependencies.len() > 0 {
                error_summary.push(error::CompileError::DependenciesNotFound(error::DependenciesNotFound::new(
                    *id,
                    missing_dependencies.clone()
                )));
            }
        }
    }
}
