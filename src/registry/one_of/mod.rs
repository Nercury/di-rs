use metafactory::{ MetaFactory };
use super::argument_builder::ArgumentBuilder;
use super::Registry;

pub struct OneOfParams {
    pub collection_id: String,
    pub id: String,
    pub value: Box<MetaFactory + 'static>,
}

impl OneOfParams {
    pub fn new(collection_id: &str, id: &str, value: Box<MetaFactory + 'static>) -> OneOfParams {
        OneOfParams {
            collection_id: collection_id.to_string(),
            id: id.to_string(),
            value: value,
        }
    }
}

pub struct OneOf<'a> {
    pub finalizer: &'a mut (OneOfFinalizer + 'a),
    pub params: OneOfParams,
    pub arg_builder: ArgumentBuilder,
}

impl<'a> OneOf<'a> {
    pub fn new(finalizer: &'a mut OneOfFinalizer, collection_id: &str, id: &str, value: Box<MetaFactory + 'static>) -> OneOf<'a> {
        OneOf {
            finalizer: finalizer,
            params: OneOfParams::new(
                collection_id,
                id,
                value
            ),
            arg_builder: ArgumentBuilder::new(),
        }
    }

    pub fn with_args(
        mut self,
        arg_sources: &[&str]
    )
        -> OneOf<'a>
    {
        self.arg_builder.set_arg_sources(arg_sources);
        self
    }

    pub fn with_arg(
        mut self,
        arg_source: &str
    )
        -> OneOf<'a>
    {
        self.arg_builder.set_arg_source(arg_source);
        self
    }

    pub fn add_arg(
        mut self,
        arg_source: &str
    )
        -> OneOf<'a>
    {
        self.arg_builder.push_arg_source(arg_source);
        self
    }

    pub fn insert(self) -> &'a mut (OneOfFinalizer + 'a) {
        let finalizer = self.finalizer;
        finalizer.finalize_one_of(self.params, self.arg_builder);
        finalizer
    }
}

pub trait OneOfFinalizer {
    fn finalize_one_of(&mut self, params: OneOfParams, arg_builder: ArgumentBuilder);
}

impl OneOfFinalizer for Registry {
    fn finalize_one_of(&mut self, params: OneOfParams, arg_builder: ArgumentBuilder) {
        self.finalize_with_args_one_of(
            params.collection_id.as_slice(),
            params.id.as_slice(),
            params.value,
            arg_builder.arg_sources
        );
    }
}
