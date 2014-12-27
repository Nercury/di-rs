use registry::error;
use registry::Registry;

use super::Validator;

#[deriving(Copy)]
pub struct ArgumentCountValidator;

impl Validator for ArgumentCountValidator {
    fn validate(&self, registry: &Registry, error_summary: &mut Vec<error::CompileError>) {
        for error in registry.definitions.iter()
            .filter_map(|(id, candidate)| {
                if candidate.arg_sources.len() == candidate.metafactory.get_arg_types().len() {
                    None
                } else {
                    Some(error::CompileError::ArgumentCountMismatch(error::ArgumentCountMismatch::new(
                        id.as_slice(), candidate
                    )))
                }
            })
        {
            error_summary.push(error);
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
