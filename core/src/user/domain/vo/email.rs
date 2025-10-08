use std::fmt;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
    EMAIL_REGEX,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        if value.trim().is_empty() {
            return Err((CategoryError::Email, TypeError::Empty).into());
        }

        let trimmed = value.trim().to_lowercase();
        let len = trimmed.len();
        const MIN_EMAIL_LEN: usize = 6;
        const MAX_EMAIL_LEN: usize = 254;

        if len < MIN_EMAIL_LEN {
            return Err((CategoryError::Email, TypeError::TooShort { short: MIN_EMAIL_LEN as u16 }).into());
        }

        if len > MAX_EMAIL_LEN {
            return Err((CategoryError::Email, TypeError::TooLong { long: MAX_EMAIL_LEN as u32 }).into());
        }

        if !EMAIL_REGEX.regex.is_match(&trimmed) {
            return Err((CategoryError::Email, TypeError::Format { format: EMAIL_REGEX.name.into() }).into());
        }

        Ok(Self(trimmed))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Email {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}
