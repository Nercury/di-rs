use registry::error;
use registry::Registry;
use registry::candidate::DefinitionCandidate;

use super::Validator;

#[deriving(Copy)]
pub struct NoOverridesValidator;

impl Validator for NoOverridesValidator {
    fn validate(&self, registry: &Registry, error_summary: &mut Vec<error::CompileError>) {
        for (id, candidates) in registry.overriden_definitions.iter() {

            let mut duplicates = candidates.iter()
                .map(|c| c)
                .collect::<Vec<&DefinitionCandidate>>();

            if let Some(added_candidate) = registry.maybe_definitions.get(id) {
                duplicates.push(added_candidate);

                error_summary.push(
                    error::CompileError::DuplicateDefinitions(
                        error::DuplicateDefinitions::new(
                            id.as_slice(),
                            &duplicates
                        )
                    )
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use registry::Registry;
    use registry::error;

    use registry::validator::Validator;
    use super::NoOverridesValidator;

    #[test]
    fn should_not_return_duplicates_for_no_items() {
        let registry = Registry::new();
        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 0);
    }

    #[test]
    fn should_not_return_duplicates_for_different_items() {
        let mut registry = Registry::new();

        registry.one("a", "hello").insert();
        registry.one("b", "hello").insert();

        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 0);
    }

    #[test]
    fn should_return_duplicates_for_duplicate_items() {
        let mut registry = Registry::new();

        registry.one("a", "hello").insert();
        registry.one("a", "hello").insert();

        let error_summary = validate_and_summarize(&registry);

        assert_eq!(error_summary.len(), 1);
    }

    #[test]
    fn should_group_2_dups_into_1_message() {
        let mut registry = Registry::new();

        registry.one("a", "hello").insert();
        registry.one("a", "hello").insert();

        let error_summary = validate_and_summarize(&registry);

        if let &error::CompileError::DuplicateDefinitions(ref e) = error_summary.get(0).unwrap() {
            let aliases = aliases_to_vec(&e.aliases);

            assert_eq!(aliases.len(), 1);
            assert!(aliases.get(0).unwrap().count == 2);
        } else {
            panic!("Expected DuplicateDefinitions error!");
        }
    }

    #[test]
    fn should_group_2_dups_into_1_message_and_ignore_unrelated() {
        let mut registry = Registry::new();

        registry.one("a", "hello").insert();
        registry.one("a", "hello").insert();
        registry.one("b", "bye").insert();

        let error_summary = validate_and_summarize(&registry);

        if let &error::CompileError::DuplicateDefinitions(ref e) = error_summary.get(0).unwrap() {
            let aliases = aliases_to_vec(&e.aliases);

            assert_eq!(aliases.len(), 1);
            assert!(aliases.get(0).unwrap().count == 2);
        } else {
            panic!("Expected DuplicateDefinitions error!");
        }
    }

    #[test]
    fn should_put_2_different_dups_into_2_messages_and_ignore_unrelated() {
        let mut registry = Registry::new();

        registry.one("a", |_: int| "hello").add_arg("t").insert();
        registry.one("a", "hello").insert();
        registry.one("b", "bye").insert();

        let error_summary = validate_and_summarize(&registry);

        if let &error::CompileError::DuplicateDefinitions(ref e) = error_summary.get(0).unwrap() {
            let aliases = aliases_to_vec(&e.aliases);

            assert_eq!(aliases.len(), 2);
            assert!(aliases.get(0).unwrap().count == 1);
            assert!(aliases.get(1).unwrap().count == 1);
        } else {
            panic!("Expected DuplicateDefinitions error!");
        }
    }

    fn aliases_to_vec(aliases: &BTreeMap<String, error::Duplicate>) -> Vec<error::Duplicate> {
        aliases.values().map(|v| v.clone()).collect()
    }

    fn validate_and_summarize<'r>(registry: &Registry) -> Vec<error::CompileError> {
        let mut error_summary = Vec::<error::CompileError>::new();
        NoOverridesValidator.validate(registry, &mut error_summary);
        error_summary
    }
}
