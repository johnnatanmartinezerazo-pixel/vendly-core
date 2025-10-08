use std::fmt;
use std::convert::TryFrom;
use regex::Regex;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
    LOCALE_REGEX,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Locale(String);

impl Locale {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        if value.trim().is_empty() {
            return Err((CategoryError::Locale, TypeError::Empty).into());
        }

        let trimmed = value.trim().to_lowercase();
        let len = trimmed.len();
        const MIN_LOCALE_LEN: usize = 2;
        const MAX_LOCALE_LEN: usize = 5;

        if len < MIN_LOCALE_LEN {
            return Err((CategoryError::Locale, TypeError::TooShort { short: MIN_LOCALE_LEN as u16 }).into());
        }

        if len > MAX_LOCALE_LEN {
            return Err((CategoryError::Locale, TypeError::TooLong { long: MAX_LOCALE_LEN as u32 }).into());
        }

        if !LOCALE_REGEX.regex.is_match(&trimmed) {
            return Err((CategoryError::Locale, TypeError::Format { format: LOCALE_REGEX.name.into() }).into());
        }

        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Locale {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Locale {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Locale::new(value)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale("es-ES".to_string())
    }
}
