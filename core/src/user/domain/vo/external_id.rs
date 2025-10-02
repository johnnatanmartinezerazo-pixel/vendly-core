use std::fmt;
use std::convert::TryFrom;

use super::{ValidationError, ExternalIdErrorKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalId(String);

impl ExternalId {
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(ExternalIdErrorKind::Empty.into());
        }

        if trimmed.len() > 255 {
            return Err(ExternalIdErrorKind::TooLong.into());
        }

        Ok(Self(trimmed.to_string()))
    }

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
