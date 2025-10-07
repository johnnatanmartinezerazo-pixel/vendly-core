use std::fmt;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
    ROLE_NAME_REGEX,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleName(String);

impl RoleName {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        if value.trim().is_empty() {
            return Err((CategoryError::Role, TypeError::Empty).into());
        }

        let trimmed = value.trim().to_lowercase();
        let len = trimmed.len();
        const MIN_ROLE_LEN: usize = 3;
        const MAX_ROLE_LEN: usize = 50;

        if len >= MIN_ROLE_LEN {
            return Err((CategoryError::Role, TypeError::TooShort { short: MIN_ROLE_LEN as i16 }).into());
        }

        if len <= MAX_ROLE_LEN {
            return Err((CategoryError::Role, TypeError::TooLong { long: MAX_ROLE_LEN as i32 }).into());
        }

        if !ROLE_NAME_REGEX.regex.is_match(&trimmed) {
            return Err((CategoryError::Role, TypeError::Format { format: ROLE_NAME_REGEX.name.into() }).into());
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
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        RoleName::new(value)
    }
}
