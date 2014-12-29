/*!

Contains utilities such as `Registry` to build centralized value
construction `Container`.
*/

use std::any::Any;
use std::collections::{ BTreeMap, BTreeSet, HashMap };
use std::collections::btree_map;

use metafactory::{ ToMetaFactory, MetaFactory };
use metafactory::aggregate::{ Aggregate };

use container::Container;

use self::new_definition::{ NewDefinition };
use self::candidate::{ GroupCandidate, DefinitionCandidate };
use self::error::{ CompileError, CircularDependency };

mod candidate;

pub mod new_definition;
pub mod error;
pub mod validator;

/**
Use `Registry` to build and initialize `Container`.

The `Registry` is mutable container of value factory `definitions` and their
groups. Definition is added using `one` method variants, method group
is added by registering a definition that belongs to group using `one_of`
method variants.

Definition value must implement `ToMetafactory` trait. Currently possible
definition sources are any clonable value or closure.

## `One` definitions

```
let mut registry = di::Registry::new();

// Insert value from cloneable source immediately, but no arguments possible.
registry.insert_one("a", 5i);

// Same, but from closure.
registry.insert_one("a", || 5i);

// Same, but using the fluent interface.
registry
    .one("a", || 5i)
    .insert();

// When definition requires arguments, their source need to be explicitly
// defined using "with_arg", "with_args" or "add_arg":
registry
    .one("b", |a: int| a + 1i)
    .with_arg("a")
    .insert();

// The definition will have a value that can be known to compiler at compile
// time:
registry
    .one("sum", |a: int, b:int| a + b) // "sum" will be "int"
    .with_args(&["a", "b"])
    .insert();
```

## `One_of` definition groups

```
let mut registry = di::Registry::new();

// Definition can be assigned to a group.
registry
    .one("a", 5i)
    .in_group("integers")
    .insert();

// If the name "a" is not important, "one_of" method can be used:
registry.insert_one_of("integers", 5i);

// The same can be written using fluent interface:
registry
    .one_of("integers", 5i)
    .insert();

// If using the fluent interface, the name can be added back:
registry
    .one_of("integers", 5i)
    .with_id("a")
    .insert();

// Group members can depend on other definitions:
registry
    .one_of("integers", |previous: int| previous + 1)
    .with_arg("a")
    .with_id("b")
    .insert();

// Defined "integers" group can be used as Vec<int> argument in other
// definitions:
registry
    .one("sum", |items: Vec<int>| items.into_iter().fold(0i, |a, i| a + i))
    .with_arg("integers")
    .insert();
```

## Compiling the container

When the registry is configured, we can check for errors and build the
`Container`, that contains clonable factories for all the registered
definitions and definition groups.

This is recommended pattern:

```
let mut registry = di::Registry::new();

// Build the registry.
registry.insert_one("a", 5i32);
// < registry ... >

match registry.compile() {
    Ok(container) => {
        // Get a factory which will be used for the application lifetime
        // to construct a value.
        if let Some(a_factory) = container.get::<i32>("a") {
            assert_eq!(5, a_factory.take());
        }
    },
    Err(errors) => di::error_printer::pretty_print(&errors),
}
```

The errors contains detailed information about all errors at once. Console
applications can use provided pretty printer.
*/
pub struct Registry {
    /// Contains a list of group candidates.
    groups: BTreeMap<String, GroupCandidate>,
    /// Contains a list of definition candidates.
    definitions: BTreeMap<String, DefinitionCandidate>,
    /// Contains a list of definitions that were overriden while building
    /// the registry - so we can at least show some kind of warning.
    overriden_definitions: BTreeMap<String, Vec<DefinitionCandidate>>,
    /// Validator list that is run before the compilation.
    validators: Vec<Box<validator::Validator + 'static>>,
}

impl Registry {
    /// Produces a new `Registry`.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut registry = di::Registry::new();
    /// ```
    pub fn new() -> Registry {
        let mut registry = Registry {
            groups: BTreeMap::new(),
            definitions: BTreeMap::new(),
            overriden_definitions: BTreeMap::new(),
            validators: Vec::new(),
        };

        registry.push_validator(validator::argument_count::ArgumentCountValidator);
        registry.push_validator(validator::overrides::NoOverridesValidator);
        registry.push_validator(validator::dependencies::DependencyValidator);

        registry
    }

    fn push_validator<T: validator::Validator + 'static>(
        &mut self,
        validator: T
    ) {
        self.validators.push(box validator);
    }

    /**
    Compile a new `Container` that contains validated factories for
    all definitions.

    ## Example

    ```
    # let mut registry = di::Registry::new();
    match registry.compile() {
        Ok(container) => {
            // Validated successfuly, can be used.
        },
        Err(errors) => di::error_printer::pretty_print(&errors),
    }
    ```
    */
    pub fn compile(&self) -> Result<Container, Vec<CompileError>> {
        let mut error_summary = Vec::<CompileError>::new();

        for validator in self.validators.iter() {
            validator.validate(self, &mut error_summary);
        }

        let mut factory_map = HashMap::<String, Box<Any>>::new();
        let groups = self.collect_group_dependencies();

        if error_summary.len() == 0 {
            // Compile definitions.
            for id in self.definitions.keys() {
                let create_factory_result = self.create_factory(
                    &groups,
                    &mut Vec::<String>::new(),
                    id.as_slice()
                );
                match create_factory_result {
                    Err(error) => {
                        error_summary.push(CompileError::CircularDependency(error));
                        break;
                    },
                    Ok(factory) => { factory_map.insert(id.to_string(), factory); },
                };
            }
            // Compile groups.
            for group in groups.keys() {
                let create_factory_result = self.create_factory(
                    &groups,
                    &mut Vec::<String>::new(),
                    group.as_slice()
                );
                match create_factory_result {
                    Err(error) => {
                        error_summary.push(CompileError::CircularDependency(error));
                        break;
                    },
                    Ok(factory) => { factory_map.insert(group.clone(), factory); },
                };
            }
        }

        if error_summary.len() == 0 {
            Ok(Container::new(factory_map))
        } else {
            Err(error_summary)
        }
    }

    /**
    Insert a new definition without arguments.

    ## Example

    ```
    # let mut registry = di::Registry::new();
    registry.insert_one("a", 5i32); // Clonable value
    registry.insert_one("b", || -> i32 { 5 }); // Closure

    match registry.compile() {
        Ok(container) => {
            if let Some(a) = container.get::<i32>("a") {
                assert_eq!(5, a.take());
            }
            if let Some(b) = container.get::<i32>("b") {
                assert_eq!(5, b.take());
            }
        },
        Err(errors) => di::error_printer::pretty_print(&errors),
    }
    ```

    */
    pub fn insert_one<T: 'static + ToMetaFactory>(&mut self, id: &str, value: T) {
        self.define(
            None,
            id,
            value.to_metafactory(),
            Vec::new()
        );
    }

    /**
    Insert a new definition into a group, without arguments.

    Definition group has the Vec<T> for the definition of T type.

    ## Example

    ```
    # let mut registry = di::Registry::new();
    registry.insert_one_of("a", 1i32); // Clonable value
    registry.insert_one_of("a", || -> i32 { 2 }); // Closure

    match registry.compile() {
        Ok(container) => {
            if let Some(a) = container.get::<Vec<i32>>("a") {
                assert_eq!(vec![ 1, 2 ], a.take());
            }
        },
        Err(errors) => di::error_printer::pretty_print(&errors),
    }
    ```
    */
    pub fn insert_one_of<T: 'static + ToMetaFactory>(&mut self, collection_id: &str, value: T) {
        let mut id;
        let metafactory = value.to_metafactory();

        self.define_group_if_not_exists(collection_id, metafactory.new_aggregate());

        if let Some(group) = self.groups.get_mut(collection_id) {
            group.member_count += 1;
            id = format!("{}`{}", collection_id, group.member_count);
        } else {
            panic!("Expected to find defined group.")
        }

        self.define(
            Some(collection_id.to_string()),
            id.as_slice(),
            metafactory,
            Vec::new()
        );
    }

    /**
    Insert a new definition using fluent interface.

    This allows to specify definition dependencies and use closures with
    arguments.

    Browse [`NewDefinition`](new_definition/struct.NewDefinition.html)
    documentation for complete examples.

    ## Example

    ```
    # let mut registry = di::Registry::new();
    registry
        .one("a", || -> i32 { 5 })
        .insert();

    registry
        .one("b", |a: i32| -> i32 { a + 3 })
        .with_arg("a")
        .insert();

    match registry.compile() {
        Ok(container) => {
            if let Some(a) = container.get::<i32>("a") {
                assert_eq!(5, a.take());
            }
            if let Some(b) = container.get::<i32>("b") {
                assert_eq!(5 + 3, b.take());
            }
        },
        Err(errors) => di::error_printer::pretty_print(&errors),
    }
    ```
    */
    pub fn one<'r, T: 'static + ToMetaFactory>(&'r mut self, id: &str, value: T)
        -> NewDefinition<'r>
    {
        NewDefinition::new(
            self,
            None,
            id,
            value.to_metafactory()
        )
    }

    /**
    Insert a new definition into a group using fluent interface.

    This allows to specify definition dependencies and use closures with
    arguments.

    Definition group has the Vec<T> for the definition of T type.

    ## Example

    ```
    # let mut registry = di::Registry::new();
    registry
        .one_of("values", 1i32)
        .with_id("a")
        .insert();

    registry
        .one_of("values", |a: i32| -> i32 { a + 2 })
        .with_arg("a")
        .insert();

    match registry.compile() {
        Ok(container) => {
            if let Some(a) = container.get::<Vec<i32>>("values") {
                assert_eq!(vec![ 1, 1 + 2 ], a.take());
            }
        },
        Err(errors) => di::error_printer::pretty_print(&errors),
    }
    ```
    */
    pub fn one_of<'r, T: 'static + ToMetaFactory>(&'r mut self, collection_id: &str, value: T)
        -> NewDefinition<'r>
    {
        let mut id;
        let metafactory = value.to_metafactory();

        self.define_group_if_not_exists(collection_id, metafactory.new_aggregate());

        if let Some(group) = self.groups.get_mut(collection_id) {
            group.member_count += 1;
            id = format!("{}`{}", collection_id, group.member_count);
        } else {
            panic!("Expected to find defined group.")
        }

        NewDefinition::new(
            self,
            Some(collection_id.to_string()),
            id.as_slice(),
            metafactory
        )
    }

    /**
    Specify that a group can be empty.

    ## Example

    ```
    # let mut registry = di::Registry::new();
    registry
        .one("sum", |values: Vec<i32>| values.into_iter().fold(0i32, |a, i| a + 1))
        .with_arg("values")
        .insert();

    // If there is no definition of any member of "values", our "sum"
    // will still work and receive empty list.
    registry.may_be_empty::<i32>("values");

    match registry.compile() {
        Ok(container) => {
            if let Some(sum) = container.get::<i32>("sum") {
                assert_eq!(0, sum.take());
            }
        },
        Err(errors) => di::error_printer::pretty_print(&errors),
    }
    ```
    */
    pub fn may_be_empty<T: 'static>(&mut self, collection_id: &str) {
        self.define_group_if_not_exists(collection_id, Aggregate::new::<T>());
    }

    fn collect_group_dependencies<'r>(&'r self) -> BTreeMap<String, BTreeSet<&'r str>> {
        let mut groups: BTreeMap<String, BTreeSet<&str>> = BTreeMap::new();

        for (id, value) in self.definitions.iter()
            .filter(|&(_, v)| v.collection_id != None)
        {
            match groups.entry(value.collection_id.clone().unwrap()) {
                btree_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(id.as_slice());
                },
                btree_map::Entry::Vacant(entry) => {
                    let mut set: BTreeSet<&str> = BTreeSet::new();
                    set.insert(id.as_slice());
                    entry.set(set);
                }
            }
        }

        for id in self.groups.keys() {
            if !groups.contains_key(id) {
                groups.insert(id.to_string(), BTreeSet::new());
            }
        }

        groups
    }

    fn define_group_if_not_exists(&mut self, collection_id: &str, type_aggregate: Aggregate<'static>) {
        if !self.groups.contains_key(collection_id) {
            self.groups.insert(
                collection_id.to_string(),
                GroupCandidate::new(type_aggregate)
            );
        }
    }

    fn define(&mut self, collection_id: Option<String>, id: &str, value: Box<MetaFactory + 'static>, args: Vec<String>) {
        if let Some(overriden_candidate) = self.definitions.remove(id) {
            match self.overriden_definitions.entry(id.to_string()) {
                btree_map::Entry::Vacant(entry) => { entry.set(vec![overriden_candidate]); },
                btree_map::Entry::Occupied(mut entry) => { entry.get_mut().push(overriden_candidate); },
            };
        }

        let candidate = DefinitionCandidate::new(
            value,
            args,
            collection_id
        );

        self.definitions.insert(
            id.to_string(),
            candidate
        );
    }

    fn create_factory(
        &self,
        groups: &BTreeMap<String, BTreeSet<&str>>,
        dependency_chain: &mut Vec<String>,
        id: &str
    ) -> Result<Box<Any>, CircularDependency> {
        dependency_chain.push(id.to_string());

        // Find if this is a definition or a group.
        let mut result = match self.definitions.get(id) {
            Some(definition) => {
                // Check for circular dependencies.

                for source in definition.arg_sources.iter() {
                    if dependency_chain.contains(source) {
                        dependency_chain.push(source.clone());
                        return Err(CircularDependency::new(dependency_chain.clone()));
                    }
                }

                // Create definition factory.

                Some(self.create_definition_factory(
                    groups,
                    dependency_chain,
                    id,
                    definition
                ))
            },
            None => None,
        };

        if let None = result {
            result = match self.groups.get(id) {
                Some(group) => {
                    // Collect users of this group.
                    let group_dependencies = groups.get(id).expect(format!("expected to find dependencies for group {}", id).as_slice());

                    // Check for circular dependencies.
                    for source in group_dependencies.iter() {
                        if dependency_chain.contains(&source.to_string()) {
                            dependency_chain.push(source.to_string());
                            return Err(CircularDependency::new(dependency_chain.clone()));
                        }
                    }

                    Some(self.create_group_factory(
                        groups,
                        dependency_chain,
                        group,
                        group_dependencies
                    ))
                },
                None => None,
            }
        }

        dependency_chain.pop();

        result.expect(format!("expected definition or group {} was not found", id).as_slice())
    }

    fn create_definition_factory(
        &self,
        groups: &BTreeMap<String, BTreeSet<&str>>,
        dependency_chain: &mut Vec<String>,
        id: &str,
        definition: &DefinitionCandidate
    ) -> Result<Box<Any>, CircularDependency> {

        let mut argument_factories = Vec::<Box<Any>>::with_capacity(definition.arg_sources.len());

        for source in definition.arg_sources.iter() {
            match self.create_factory(
                groups,
                dependency_chain,
                source.as_slice()
            ) {
                Err(error) => return Err(error),
                Ok(factory) => argument_factories.push(factory),
            };
        }

        Ok(
            definition.metafactory.new(argument_factories)
                .ok()
                .expect(
                    format!(
                        "failed to create factory {} with arguments \"{}\" - most likely argument types are not the same",
                        id,
                        definition.arg_sources.connect("\", \"")
                    ).as_slice()
                )
        )
    }

    fn create_group_factory(
        &self,
        groups: &BTreeMap<String, BTreeSet<&str>>,
        dependency_chain: &mut Vec<String>,
        group: &GroupCandidate,
        group_sources: &BTreeSet<&str>
    )
        -> Result<Box<Any>, CircularDependency>
    {
        let mut argument_factories = Vec::<Box<Any>>::with_capacity(group_sources.len());

        for source in group_sources.iter() {
            match self.create_factory(
                groups,
                dependency_chain,
                *source
            ) {
                Err(error) => return Err(error),
                Ok(factory) => argument_factories.push(factory),
            };
        }

        Ok(group.aggregate.new_factory(
            argument_factories
        ))
    }
}
