use typedef::TypeDef;

use std::iter::repeat;
use std::collections::{ VecMap, HashMap, HashSet };
use std::collections::hash_map::{ Entry };

use registry::error;
use registry::Registry;
use registry::candidate::{ DefinitionCandidateKey, GroupCandidateKey };

use super::Validator;

#[deriving(Copy)]
pub struct DependencyValidator;

struct DefinitionRequirements<'a> {
    typedef: TypeDef,
    candidate_key: Option<DefinitionCandidateKey>,
    group_key: Option<GroupCandidateKey>,
    arguments: HashMap<&'a str, TypeDef>,
}

impl Validator for DependencyValidator {
    fn validate(&self, registry: &Registry, error_summary: &mut Vec<error::CompileError>) {
        // Collect group_id -> [ child_id ] map.

        let mut groups: HashMap<String, HashSet<&str>> = HashMap::new();
        for key in registry.maybe_definitions.keys()
            .filter(|k| k.collection_id != None)
        {
            match groups.entry(key.collection_id.clone().unwrap()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(key.id.as_slice());
                },
                Entry::Vacant(entry) => {
                    let mut set: HashSet<&str> = HashSet::new();
                    set.insert(key.id.as_slice());
                    entry.set(set);
                }
            }
        }

        // Collect all_definition_id -> required definition_id + type map.

        let definitions: HashMap<&str, DefinitionRequirements> = registry.maybe_definitions.iter()
            .map(|(k, c)|
                (k.id.as_slice(), DefinitionRequirements {
                    typedef: c.metafactory.get_type(),
                    candidate_key: Some(k.clone()),
                    group_key: None,
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
                        candidate_key: None,
                        group_key: Some(k.clone()),
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

        // Check for missing dependencies and type mismatches.

        let mut missing_dependencies = HashSet::<String>::new();
        let mut mismatched_types = VecMap::<TypeDef>::new();

        for (id, requirements) in definitions.iter() {
            missing_dependencies.clear();
            mismatched_types.clear();

            for arg_id in requirements.arguments.keys() {
                if !definitions.contains_key(arg_id) {
                    missing_dependencies.insert(arg_id.to_string());
                }
            }

            if missing_dependencies.len() > 0 {

                error_summary.push(error::CompileError::DependenciesNotFound(error::DependenciesNotFound::new(
                    *id,
                    missing_dependencies.clone()
                )));

            } else {

                let mut arg_index = 0;

                for (arg_id, arg_type) in requirements.arguments.iter() {
                    if let Some(arg_requirements) = definitions.get(arg_id) {
                        if arg_requirements.typedef != *arg_type {
                            mismatched_types.insert(arg_index, arg_requirements.typedef.clone());
                        }
                    }

                    arg_index += 1;
                }

                if mismatched_types.len() > 0 {
                    if let Some(candidate_key) = requirements.candidate_key.clone() {
                        if let Some(candidate) = registry.maybe_definitions.get(&candidate_key) {
                            error_summary.push(error::CompileError::IncorrectDepencencyTypes(error::IncorrectDepencencyTypes::new(
                                *id,
                                candidate_key.collection_id.clone(),
                                candidate.metafactory.get_type(),
                                candidate.metafactory.get_arg_types(),
                                candidate.arg_sources.clone(),
                                mismatched_types.clone()
                            )));
                        } else {
                            panic!("Previously found candidate not found in registry.")
                        }
                    } else if let Some(group_key) = requirements.group_key.clone() {
                        if let Some(group) = registry.maybe_groups.get(&group_key) {
                            let childs = groups.get(&id.to_string()).expect("Failed to get childs for collection");
                            error_summary.push(error::CompileError::IncorrectDepencencyTypes(error::IncorrectDepencencyTypes::new(
                                *id,
                                None,
                                requirements.typedef.clone(),
                                repeat(group.factory.get_arg_type()).take(childs.len()).collect(),
                                childs.iter().map(|s| s.to_string()).collect(),
                                mismatched_types.clone()
                            )));
                        }
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use registry::Registry;
    use registry::error;

    use registry::validator::Validator;
    use super::DependencyValidator;

    #[test]
    fn should_not_return_error_for_no_items() {
        let registry = Registry::new();
        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 0);
    }

    #[test]
    fn one_should_not_return_error_if_no_dependencies() {
        let mut registry = Registry::new();

        registry.one("miracle", "happened").insert();

        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 0);
    }

    #[test]
    fn one_of_should_not_return_error_if_no_dependencies() {
        let mut registry = Registry::new();

        registry.one_of("miracles", "miracle", "happened").insert();

        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 0);
    }

    #[test]
    fn one_should_return_error_missing_dep() {
        let mut registry = Registry::new();

        registry
            .one("miracle", |_reason: int| "happened")
            .add_arg("reason")
            .insert();

        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 1);

        if let &error::CompileError::DependenciesNotFound(ref e) = error_summary.get(0).unwrap() {
            assert_eq!(e.id, "miracle");
            assert!(e.missing_dependencies.contains("reason"));
        } else {
            panic!("Expected DependenciesNotFound error!");
        }
    }

    #[test]
    fn one_of_should_return_error_missing_dep() {
        let mut registry = Registry::new();

        registry
            .one_of("miracles", "miracle", |_reason: int, _side_effects: bool| "happened")
            .add_arg("reason")
            .add_arg("side_effects")
            .insert();

        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 1);

        if let &error::CompileError::DependenciesNotFound(ref e) = error_summary.get(0).unwrap() {
            assert_eq!(e.id, "miracle");
            assert!(e.missing_dependencies.contains("reason"));
            assert!(e.missing_dependencies.contains("side_effects"));
        } else {
            panic!("Expected DependenciesNotFound error!");
        }
    }

    fn validate_and_summarize<'r>(registry: &Registry) -> Vec<error::CompileError> {
        let mut error_summary = Vec::<error::CompileError>::new();
        DependencyValidator.validate(registry, &mut error_summary);
        error_summary
    }
}
