use registry::error;
use registry::Registry;

pub mod overrides;
pub mod arguments;

pub trait Validator {
    fn validate(
        &self,
        registry: &Registry,
        error_summary: &mut Vec<error::CompileError>
    );
}
