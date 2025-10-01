use regex::Regex;
use std::fmt;
use std::convert::TryFrom;
use std::sync::LazyLock;

use super::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

static USERNAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?=.{6,30}$)[a-z][a-z0-9](?:[._-]?[a-z0-9])*$").unwrap()
});

impl Username {
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let trimmed = value.trim().to_lowercase();

        if !USERNAME_REGEX.is_match(&trimmed) {
            return Err(ValidationError::InvalidUsername);
        }

        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Username {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Username::new(value)
    }
}
