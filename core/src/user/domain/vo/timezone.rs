use std::fmt;
use std::convert::TryFrom;
use chrono_tz::Tz;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Timezone(String);

impl Timezone {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        match trimmed.parse::<Tz>() {
            Ok(_) => Ok(Self(trimmed.to_string())),
            Err(_) => Err((CategoryError::Timezone, TypeError::NotSupported).into()),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_tz(&self) -> Tz {
        self.0.parse::<Tz>().unwrap()
    }
}

impl AsRef<str> for Timezone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Timezone {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Timezone::new(value)
    }
}

impl Default for Timezone {
    fn default() -> Self {
        Timezone("America/Bogota".to_string())
    }
}
