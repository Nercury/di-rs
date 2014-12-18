use super::error; // Super error.
use super::Registry;

pub mod overrides;

pub trait Validator {
    fn validate(
        &self,
        registry: &Registry,
        error_summary: &mut Vec<error::CompileError>
    );
}
