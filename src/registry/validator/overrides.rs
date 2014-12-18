use super::super::error; // Very super error.
use super::super::Registry;
use super::super::definition_candidate::DefinitionCandidate;

use super::Validator;

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
