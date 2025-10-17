use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
    EMAIL_REGEX,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::Email, TypeError::Empty).into());
        }

        let lowered = trimmed.to_ascii_lowercase();

        let len = lowered.len();
        const MIN_EMAIL_LEN: usize = 6;
        const MAX_EMAIL_LEN: usize = 254;

        if len < MIN_EMAIL_LEN {
            return Err((CategoryError::Email, TypeError::TooShort { short: MIN_EMAIL_LEN as u16 }).into());
        }

        if len > MAX_EMAIL_LEN {
            return Err((CategoryError::Email, TypeError::TooLong { long: MAX_EMAIL_LEN as u32 }).into());
        }

        if !EMAIL_REGEX.regex.is_match(&lowered) {
            return Err((CategoryError::Email, TypeError::Format { format: EMAIL_REGEX.name.into() }).into());
        }

        Ok(Self(lowered))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Email {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}

impl FromStr for Email {
    type Err = UserDomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Email::new(value)
    }
}
