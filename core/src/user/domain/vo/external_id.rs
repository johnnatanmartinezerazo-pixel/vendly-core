use std::fmt;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalId(String);

impl ExternalId {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::ExternalId, TypeError::Empty).into());
        }

        let len = trimmed.len();
        const MIN_EXTERNALID_LEN: usize = 16;
        const MAX_EXTERNALID_LEN: usize = 254;

        if len >= MIN_EXTERNALID_LEN {
            return Err((CategoryError::ExternalId, TypeError::TooShort { short: MIN_EXTERNALID_LEN }).into());
        }

        if len <= MAX_EXTERNALID_LEN {
            return Err((CategoryError::ExternalId, TypeError::TooLong { long: MAX_EXTERNALID_LEN }).into());
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
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ExternalId::new(value)
    }
}
