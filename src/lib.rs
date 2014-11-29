//! Dependency injection container.
//!
//! Dependency injection provides a way to create objects without specifying
//! constructor arguments. In other words, it makes creation of the object
//! someone else's problem.
//!
//! The main objects you will likely care about are `Registry` and `Container`.
//!
//! `Registry` will be used to initialize string-indexed map of object
//! factories and specifying their dependencies.
//!
//! An immutable, optimized `Container` will be __assembled__ from the registry
//! and it will be used as runtime workhorse for object creation.
//!
//! This architecture allows several benefits.
//!
//! First, you are free to to manipulate `Registry` as required by your
//! project. For example you can have multiple registry construction passes
//! that collect definitions from various modules. It also lets us worry less
//! about performance at this stage, do nice validation pass with
//! comprehensive error messages, and notify about any issues as soon as
//! application starts.
//!
//! Second, the `Container` has baked-in call list for creation of all
//! definitions, with all issues like missing/circular dependencies or
//! type mismatches already solved at __assembly__ phase. `Container` also
//! has no hashmap lookup for dependencies, so it can be fast at runtime.

#![experimental]

pub use registry::{ Registry };
pub use registry::item::{ RegistryItem, RegistryItemCandidate };
pub use registry::getter_err::{
    GetterErr,
    GetterErrKind,
    DefinitionTypeErr,
    ArgTypeErr,
    GetterTypeErr,
    ArgCountMismatchErr
};

pub use container::{ Container };

mod registry;
mod container;

pub mod getter;
pub mod definition;

pub fn output_errors(errors: Vec<GetterErr>) -> String {
    errors
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .connect("\n")
}
