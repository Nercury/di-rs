/*!

Fluent builder for new definition.

*/

use metafactory::{ MetaFactory };
use registry::Registry;

struct ArgumentBuilder {
    pub arg_sources: Vec<String>,
}

impl ArgumentBuilder {
    fn new() -> ArgumentBuilder {
        ArgumentBuilder {
            arg_sources: Vec::new(),
        }
    }

    fn set_arg_sources<'r>(&'r mut self, arg_sources: &[&str]) {
        self.arg_sources.truncate(0);
        for str in arg_sources.iter() {
            self.arg_sources.push(str.to_string());
        }
    }

    fn set_arg_source<'r>(&'r mut self, arg_source: &str) {
        self.arg_sources.truncate(0);
        self.arg_sources.push(arg_source.to_string());
    }

    fn push_arg_source<'r>(&'r mut self, arg_source: &str) {
        self.arg_sources.push(arg_source.to_string());
    }
}

struct NewDefinitionParams {
    collection_id: Option<String>,
    id: String,
    value: Box<MetaFactory + 'static>,
}

impl NewDefinitionParams {
    fn new(collection_id: Option<String>, id: &str, value: Box<MetaFactory + 'static>) -> NewDefinitionParams {
        NewDefinitionParams {
            collection_id: collection_id,
            id: id.to_string(),
            value: value,
        }
    }
}

/// New definition builder.
pub struct NewDefinition<'a> {
    registry: &'a mut Registry,
    params: NewDefinitionParams,
    arg_builder: ArgumentBuilder,
}

impl<'a> NewDefinition<'a> {
    /// Create new definition builder.
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

    /// Specify the argument dependencies for definition.
    ///
    /// Removes previous arguments.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut registry = di::Registry::new();
    ///
    /// registry
    ///     .one("sum", |a: int, b:int| a + b)
    ///     .with_args(&["a", "b"])
    ///     .insert();
    /// ```
    pub fn with_args(
        mut self,
        arg_sources: &[&str]
    )
        -> NewDefinition<'a>
    {
        self.arg_builder.set_arg_sources(arg_sources);
        self
    }

    /// Specify argument dependency for definition.
    ///
    /// Removes previous arguments.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut registry = di::Registry::new();
    ///
    /// registry
    ///     .one("inc", |a: int| a + 1)
    ///     .with_arg("a")
    ///     .insert();
    /// ```
    pub fn with_arg(
        mut self,
        arg_source: &str
    )
        -> NewDefinition<'a>
    {
        self.arg_builder.set_arg_source(arg_source);
        self
    }

    /// Add argument dependencies for definition.
    ///
    /// Appends newly added argument.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut registry = di::Registry::new();
    ///
    /// registry
    ///     .one("sum", |a: int, b:int| a + b)
    ///     .add_arg("a")
    ///     .add_arg("b")
    ///     .insert();
    /// ```
    pub fn add_arg(
        mut self,
        arg_source: &str
    )
        -> NewDefinition<'a>
    {
        self.arg_builder.push_arg_source(arg_source);
        self
    }

    /// Sets a group for this definition.
    ///
    /// Replaces previous group.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut registry = di::Registry::new();
    ///
    /// registry
    ///     .one("value", || 5i)
    ///     .in_group("values")
    ///     .insert();
    /// ```
    pub fn in_group(
        mut self,
        collection_id: &str
    )
        -> NewDefinition<'a>
    {
        self.params.collection_id = Some(collection_id.to_string());
        self
    }

    /// Sets an id for this definition.
    ///
    /// Replaces previous id. Usefull if an id is required for value
    /// grouped with `one_of` builder.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut registry = di::Registry::new();
    ///
    /// registry
    ///     .one_of("values", || 5i)
    ///     .with_id("value")
    ///     .insert();
    /// ```
    pub fn with_id(
        mut self,
        id: &str
    )
        -> NewDefinition<'a>
    {
        self.params.id = id.to_string();
        self
    }

    /// Finish the construction of a new definition and insert it into the
    /// `Registry`.
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
