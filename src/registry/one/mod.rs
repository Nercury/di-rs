use metafactory::{ MetaFactory };
use super::argument_builder::ArgumentBuilder;
use super::Registry;

pub struct OneParams {
    pub id: String,
    pub value: Box<MetaFactory + 'static>,
}

impl OneParams {
    pub fn new(id: &str, value: Box<MetaFactory + 'static>) -> OneParams {
        OneParams {
            id: id.to_string(),
            value: value,
        }
    }
}

pub struct One<'a> {
    pub finalizer: &'a mut (OneFinalizer + 'a),
    pub params: OneParams,
    pub arg_builder: ArgumentBuilder,
}

impl<'a> One<'a> {
    pub fn new(finalizer: &'a mut OneFinalizer, id: &str, value: Box<MetaFactory + 'static>) -> One<'a> {
        One {
            finalizer: finalizer,
            params: OneParams {
                id: id.to_string(),
                value: value,
            },
            arg_builder: ArgumentBuilder::new(),
        }
    }

    pub fn with_args(
        mut self,
        arg_sources: &[&str]
    )
        -> One<'a>
    {
        self.arg_builder.set_arg_sources(arg_sources);
        self
    }

    pub fn with_arg(
        mut self,
        arg_source: &str
    )
        -> One<'a>
    {
        self.arg_builder.set_arg_source(arg_source);
        self
    }

    pub fn add_arg(
        mut self,
        arg_source: &str
    )
        -> One<'a>
    {
        self.arg_builder.push_arg_source(arg_source);
        self
    }

    pub fn insert(self) -> &'a mut (OneFinalizer + 'a) {
        let finalizer = self.finalizer;
        finalizer.finalize_one_of(self.params, self.arg_builder);
        finalizer
    }
}

pub trait OneFinalizer {
    fn finalize_one_of(&mut self, params: OneParams, arg_builder: ArgumentBuilder);
}

impl OneFinalizer for Registry {
    fn finalize_one_of(&mut self, params: OneParams, arg_builder: ArgumentBuilder) {
        self.finalize_with_args_one(
            params.id.as_slice(),
            params.value,
            arg_builder.arg_sources
        );
    }
}
