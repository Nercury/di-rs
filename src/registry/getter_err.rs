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

/// Getter creation error types.
pub enum GetterErrKind {
    /// There was no definition with requested name.
    NotFound,
    /// There was a definition with requested type, but defintion returned
    /// a getter with different type.
    GetterTypeMismatch(GetterTypeErr),
    /// There was a definition, but definition type did not match requested type.
    DefinitionTypeMismatch(DefinitionTypeErr),
}

/// Getter creation error information.
pub struct GetterErr {
    /// Error type, describes the reason of failure.
    pub kind: GetterErrKind,
    /// The name of requested getter.
    pub name: String,
    /// The name sequence of all successfull getters.
    pub success_path: Vec<String>,
}

impl GetterErr {
    pub fn new(
        kind: GetterErrKind,
        name: &str,
        success_path: Vec<String>
    )
        -> GetterErr
    {
        GetterErr {
            kind: kind,
            name: name.to_string(),
            success_path: success_path,
        }
    }

    /// Convert `GetterErr` to string for output.
    pub fn to_string(&self) -> String {
        match self.kind {
            GetterErrKind::NotFound => {
                format!("Requested \"{}\" was not found.", self.name)
            },
            GetterErrKind::DefinitionTypeMismatch(type_err) => {
                format!(
                    "Requested \"{}\" of \"{}\" type, but type \"{}\" found.",
                    self.name,
                    type_err.requested.get_name(),
                    type_err.found.get_name()
                )
            },
            GetterErrKind::GetterTypeMismatch(type_err) => {
                format!(
                    "Requested \"{}\" failed to return \"{}\" getter.",
                    self.name,
                    type_err.expected.get_name(),
                )
            }
        }
    }
}
