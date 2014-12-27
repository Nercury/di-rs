use metafactory::{ MetaFactory };
use registry::argument_builder::ArgumentBuilder;
use registry::Registry;

pub struct NewDefinitionParams {
    pub collection_id: Option<String>,
    pub id: String,
    pub value: Box<MetaFactory + 'static>,
}

impl NewDefinitionParams {
    pub fn new(collection_id: Option<String>, id: &str, value: Box<MetaFactory + 'static>) -> NewDefinitionParams {
        NewDefinitionParams {
            collection_id: collection_id,
            id: id.to_string(),
            value: value,
        }
    }
}

pub struct NewDefinition<'a> {
    pub registry: &'a mut Registry,
    pub params: NewDefinitionParams,
    pub arg_builder: ArgumentBuilder,
}

impl<'a> NewDefinition<'a> {
    pub fn new(registry: &'a mut Registry, collection_id: Option<String>, id: &str, value: Box<MetaFactory + 'static>) -> NewDefinition<'a> {
        NewDefinition {
            registry: registry,
            params: NewDefinitionParams::new(
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
        -> NewDefinition<'a>
    {
        self.arg_builder.set_arg_sources(arg_sources);
        self
    }

    pub fn with_arg(
        mut self,
        arg_source: &str
    )
        -> NewDefinition<'a>
    {
        self.arg_builder.set_arg_source(arg_source);
        self
    }

    pub fn add_arg(
        mut self,
        arg_source: &str
    )
        -> NewDefinition<'a>
    {
        self.arg_builder.push_arg_source(arg_source);
        self
    }

    pub fn in_group(
        mut self,
        collection_id: &str
    )
        -> NewDefinition<'a>
    {
        self.params.collection_id = Some(collection_id.to_string());
        self
    }

    pub fn with_id(
        mut self,
        id: &str
    )
        -> NewDefinition<'a>
    {
        self.params.id = id.to_string();
        self
    }

    pub fn insert(self) -> &'a mut Registry {
        let registry = self.registry;
        registry.define(
            self.params.collection_id,
            self.params.id.as_slice(),
            self.params.value,
            self.arg_builder.arg_sources
        );
        registry
    }
}
