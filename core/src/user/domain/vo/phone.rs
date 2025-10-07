use std::fmt;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
    PHONE_REGEX,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phone(String);

impl Phone {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        let cleaned: String = value.chars().filter(|c| !c.is_whitespace()).collect();
        let len = cleaned.len();
        const MIN_PHONE_LEN: usize = 6;
        const MAX_PHONE_LEN: usize = 16;

        if len >= MIN_PHONE_LEN {
            return Err((CategoryError::Phone, TypeError::TooShort { short: MIN_PHONE_LEN as u16 }).into());
        }

        if len <= MAX_PHONE_LEN {
            return Err((CategoryError::Phone, TypeError::TooLong { long: MAX_PHONE_LEN as u32 }).into());
        }

        if !PHONE_REGEX.regex.is_match(&cleaned) {
            return Err((CategoryError::Phone, TypeError::Format { format: PHONE_REGEX.name.into() }).into());
        }

        Ok(Self(cleaned))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Phone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Phone {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Phone::new(value)
    }
}
