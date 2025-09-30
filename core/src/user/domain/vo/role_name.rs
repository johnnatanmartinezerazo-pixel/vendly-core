use std::fmt;
use std::convert::TryFrom;
use regex::Regex;

use super::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleName(String);

impl RoleName {
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let trimmed = value.trim();

        if trimmed.len() < 3 || trimmed.len() > 50 {
            return Err(ValidationError::InvalidRole);
        }

        let regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();

        if !regex.is_match(trimmed) {
            return Err(ValidationError::InvalidRole);
        }

        Ok(Self(trimmed.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RoleName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RoleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for RoleName {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        RoleName::new(value)
    }
}
