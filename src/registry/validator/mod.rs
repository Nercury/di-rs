/*!

Registry validators used at the start of container compillation.

*/

use registry::error;
use registry::Registry;

pub mod overrides;
pub mod argument_count;
pub mod dependencies;

pub trait Validator {
    fn validate(
        &self,
        registry: &Registry,
        error_summary: &mut Vec<error::CompileError>
    );
}
