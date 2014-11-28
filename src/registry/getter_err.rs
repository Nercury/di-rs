use super::super::definition::{ TypeDef };

/// Definition type does not match requested type.
pub struct DefinitionTypeErr {
    /// Requested type.
    pub requested: TypeDef,
    /// Found definition type.
    pub found: TypeDef,
}

impl DefinitionTypeErr {
    /// Convenience method for creating new `DefinitionTypeErr`.
    pub fn new(requested: TypeDef, found: TypeDef) -> DefinitionTypeErr {
        DefinitionTypeErr {
            requested: requested,
            found: found,
        }
    }

    /// Convert this error to arg type error when argument name is known.
    pub fn to_arg_err(self, name: &str) -> ArgTypeErr {
        ArgTypeErr::new(
            name,
            self.requested,
            self.found
        )
    }
}

/// Arg definition type does not match requested type.
pub struct ArgTypeErr {
    /// Argument definition name.
    pub name: String,
    /// Requested argument type.
    pub requested: TypeDef,
    /// Found definition type.
    pub found: TypeDef,
}

impl ArgTypeErr {
    /// Convenience method for creating new `ArgTypeErr`.
    pub fn new(name: &str, requested: TypeDef, found: TypeDef) -> ArgTypeErr {
        ArgTypeErr {
            name: name.to_string(),
            requested: requested,
            found: found,
        }
    }
}

/// Returned getter type does not match expected type.
pub struct GetterTypeErr {
    /// Expected type.
    pub expected: TypeDef,
}

impl GetterTypeErr {
    /// Convenience method for creating new `GetterTypeErr`.
    pub fn new(expected: TypeDef) -> GetterTypeErr {
        GetterTypeErr {
            expected: expected,
        }
    }
}

/// Getter argument count does not match mapped definition count.
pub struct ArgCountMismatchErr {
    pub required: uint,
    pub specified: uint,
}

impl ArgCountMismatchErr {
    /// Convenience method for creating new `ArgCountMismatchErr`.
    pub fn new(required: uint, specified: uint) -> ArgCountMismatchErr {
        ArgCountMismatchErr {
            required: required,
            specified: specified,
        }
    }
}

/// Getter creation error types.
pub enum GetterErrKind {
    /// There was no definition with requested name.
    NotFound,
    /// There was no definition for argument with requested name.
    ArgNotFound(String),
    /// There was a definition with requested type, but defintion returned
    /// a getter with different type.
    GetterTypeMismatch(GetterTypeErr),
    /// There was a definition, but definition type did not match requested type.
    DefinitionTypeMismatch(DefinitionTypeErr),
    /// There was a argument definition, but definition type did not match requested argument type.
    ArgTypeMismatch(ArgTypeErr),
    /// Incorrect number of mapped argument definitions.
    ArgCountMismatch(ArgCountMismatchErr),
}

/// Getter creation error information.
pub struct GetterErr {
    /// Error type, describes the reason of failure.
    pub kind: GetterErrKind,
    /// The name of requested getter.
    pub name: String
}

impl GetterErr {
    pub fn new(
        kind: GetterErrKind,
        name: &str
    )
        -> GetterErr
    {
        GetterErr {
            kind: kind,
            name: name.to_string()
        }
    }

    /// Convert definition error to argument error if there is parent
    /// argument name.
    pub fn to_arg_error(self, parent_name: &str) -> GetterErr {
        match self.kind {
            GetterErrKind::NotFound => {
                GetterErr::new(
                    GetterErrKind::ArgNotFound(self.name),
                    parent_name
                )
            },
            GetterErrKind::DefinitionTypeMismatch(type_err) => {
                GetterErr::new(
                    GetterErrKind::ArgTypeMismatch(
                        type_err.to_arg_err(self.name.as_slice())
                    ),
                    parent_name
                )
            },
            _ => self
        }
    }

    /// Convert `GetterErr` to string for output.
    pub fn to_string(&self) -> String {
        match self.kind {
            GetterErrKind::NotFound => {
                format!("Definition \"{}\" was not found.", self.name)
            },
            GetterErrKind::ArgNotFound(ref arg_name) => {
                format!("Definition \"{}\" requires \"{}\", but it was not found.", self.name, arg_name)
            },
            GetterErrKind::DefinitionTypeMismatch(type_err) => {
                format!(
                    "Expected \"{}\" of \"{}\" type, but type \"{}\" found.",
                    self.name,
                    type_err.requested.get_name(),
                    type_err.found.get_name()
                )
            },
            GetterErrKind::ArgTypeMismatch(ref type_err) => {
                format!(
                    "Definition \"{}\" requested \"{}\" of \"{}\" type, but type \"{}\" found.",
                    self.name,
                    type_err.name,
                    type_err.requested.get_name(),
                    type_err.found.get_name()
                )
            },
            GetterErrKind::GetterTypeMismatch(type_err) => {
                format!(
                    "Definition \"{}\" failed to return \"{}\" getter.",
                    self.name,
                    type_err.expected.get_name(),
                )
            },
            GetterErrKind::ArgCountMismatch(type_err) => {
                format!(
                    "Definition \"{}\" requires {} args, but {} specified.",
                    self.name,
                    type_err.required,
                    type_err.specified
                )
            }
        }
    }
}
