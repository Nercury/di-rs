use registry::error;
use registry::Registry;
use registry::definition_candidate::DefinitionCandidate;

use super::Validator;

#[deriving(Copy)]
pub struct NoOverridesValidator;

impl Validator for NoOverridesValidator {
    fn validate(&self, registry: &Registry, error_summary: &mut Vec<error::CompileError>) {
        for (key, candidates) in registry.overriden_definitions.iter() {

            let mut duplicates = candidates.iter()
                .map(|c| c)
                .collect::<Vec<&DefinitionCandidate>>();

            if let Some(added_candidate) = registry.maybe_definitions.get(key) {
                duplicates.push(added_candidate);

                error_summary.push(
                    error::CompileError::DuplicateDefinitions(
                        error::DuplicateDefinitions::new(
                            key,
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
    use registry::Registry;
    use registry::error;

    use registry::validator::Validator;
    use super::NoOverridesValidator;

    #[test]
    fn should_not_return_duplicates_for_no_items() {
        let registry = Registry::new();
        let mut error_summary = Vec::<error::CompileError>::new();

        NoOverridesValidator.validate(&registry, &mut error_summary);

        assert_eq!(error_summary.len(), 0);
    }
}
