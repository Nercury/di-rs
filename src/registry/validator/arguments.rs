use std::collections::{ HashMap, HashSet };
use std::collections::hash_map::{ Occupied, Vacant };
use typedef::TypeDef;

use registry::error;
use registry::Registry;
use registry::definition_candidate::DefinitionCandidate;
use registry::group_candidate::GroupCandidate;

use super::Validator;

#[deriving(Copy)]
pub struct ArgumentCountValidator;

impl Validator for ArgumentCountValidator {
    fn validate(&self, registry: &Registry, error_summary: &mut Vec<error::CompileError>) {
        for error in registry.maybe_definitions.iter()
            .filter_map(|(key, candidate)| {
                if candidate.arg_sources.len() == candidate.metafactory.get_arg_types().len() {
                    None
                } else {
                    Some(error::CompileError::ArgumentCountMismatch(error::ArgumentCountMismatch::new(
                        key, candidate
                    )))
                }
            })
        {
            error_summary.push(error);
        }
    }
}

#[deriving(Copy)]
pub struct ArgumentTypeValidator;

struct DefinitionRequirements<'a> {
    typedef: TypeDef,
    arguments: HashMap<&'a str, TypeDef>,
}

impl Validator for ArgumentTypeValidator {
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

        for (id, requirements) in definitions.iter() {

        }
    }
}

#[cfg(test)]
mod test {
    use typedef::TypeDef;

    use registry::Registry;
    use registry::error;

    use registry::validator::Validator;
    use super::ArgumentCountValidator;

    #[test]
    fn should_not_return_error_for_no_items() {
        let registry = Registry::new();
        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 0);
    }

    #[test]
    fn should_not_return_error_if_no_arguments() {
        let mut registry = Registry::new();

        registry.one("something", || "result").insert();

        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 0);
    }

    #[test]
    fn should_not_return_error_if_argument_count_is_same() {
        let mut registry = Registry::new();

        registry.one("something", |_a: int, _b: bool| "result")
            .with_args(&["a", "b"])
            .insert();

        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 0);
    }

    #[test]
    fn should_return_error_if_argument_count_does_not_match() {
        let mut registry = Registry::new();

        registry.one("something", |_a: int, _b: bool| "result")
            .with_args(&["a"])
            .insert();

        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 1);

        if let &error::CompileError::ArgumentCountMismatch(ref e) = error_summary.get(0).unwrap() {
            assert_eq!(e.arg_sources, vec!["a"]);
            assert_eq!(e.arg_types, vec![TypeDef::of::<int>(), TypeDef::of::<bool>()]);
        } else {
            panic!("Expected ArgumentCountMismatch error!");
        }
    }

    fn validate_and_summarize<'r>(registry: &Registry) -> Vec<error::CompileError> {
        let mut error_summary = Vec::<error::CompileError>::new();
        ArgumentCountValidator.validate(registry, &mut error_summary);
        error_summary
    }
}
