use std::fmt;
use std::convert::TryFrom;

// Import ValidationError from the parent module
use super::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalId(String);

impl ExternalId {
    /// Crea un `ExternalId` validando reglas generales.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::InvalidExternalId);
        }

        if trimmed.len() > 255 {
            return Err(ValidationError::InvalidExternalId);
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Devuelve el identificador como &str
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ExternalId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExternalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for ExternalId {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ExternalId::new(value)
    }
}
