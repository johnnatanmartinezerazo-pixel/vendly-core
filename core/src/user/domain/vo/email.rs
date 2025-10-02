use regex::Regex;
use std::fmt;
use std::convert::TryFrom;
use std::sync::LazyLock;

use super::{ValidationError, EmailErrorKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^[A-Za-z0-9](?:[A-Za-z0-9._-]{2,63}[A-Za-z0-9])?@[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?(?:\.[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?)*\.[A-Za-z]{2,24}$"
    ).unwrap()
});

impl Email {
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        if value.trim().is_empty() {
            return Err(EmailErrorKind::Empty.into());
        }

        let trimmed = value.trim().to_lowercase();

        if trimmed.len() > 254 {
            return Err(EmailErrorKind::TooLong.into());
        }

        if !EMAIL_REGEX.is_match(&trimmed) {
            return Err(EmailErrorKind::Format.into());
        }

        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
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
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}
