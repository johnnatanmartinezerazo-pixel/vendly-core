use std::fmt;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    CategoryError, TypeError, UserDomainError, USERNAME_REGEX
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);



impl Username {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim().to_lowercase();

        if !USERNAME_REGEX.regex.is_match(&trimmed) {
            return Err((CategoryError::Email, TypeError::Format { format: USERNAME_REGEX.name.into() }).into());
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
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Username::new(value)
    }
}
