use metafactory::{ MetaFactory };
use registry::argument_builder::ArgumentBuilder;
use registry::Registry;

pub struct OneOfParams {
    pub collection_id: Option<String>,
    pub id: String,
    pub value: Box<MetaFactory + 'static>,
}

impl OneOfParams {
    pub fn new(collection_id: Option<String>, id: &str, value: Box<MetaFactory + 'static>) -> OneOfParams {
        OneOfParams {
            collection_id: collection_id,
            id: id.to_string(),
            value: value,
        }
    }
}

pub struct OneOf<'a> {
    pub registry: &'a mut Registry,
    pub params: OneOfParams,
    pub arg_builder: ArgumentBuilder,
}

impl<'a> OneOf<'a> {
    pub fn new(registry: &'a mut Registry, collection_id: Option<String>, id: &str, value: Box<MetaFactory + 'static>) -> OneOf<'a> {
        OneOf {
            registry: registry,
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

    pub fn insert(self) -> &'a mut Registry {
        let registry = self.registry;
        registry.finalize(
            self.params.collection_id,
            self.params.id.as_slice(),
            self.params.value,
            self.arg_builder.arg_sources
        );
        registry
    }
}
