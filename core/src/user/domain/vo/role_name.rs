use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult };

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
    ROLE_NAME_REGEX,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleName(String);

impl RoleName {
    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim().to_ascii_lowercase();

        if trimmed.is_empty() {
            return Err((CategoryError::Role, TypeError::Empty).into());
        }

        let len = trimmed.len();
        const MIN_ROLE_LEN: usize = 3;
        const MAX_ROLE_LEN: usize = 50;

        if len < MIN_ROLE_LEN {
            return Err((CategoryError::Role, TypeError::TooShort { short: MIN_ROLE_LEN as u16 }).into());
        }

        if len > MAX_ROLE_LEN {
            return Err((CategoryError::Role, TypeError::TooLong { long: MAX_ROLE_LEN as u32 }).into());
        }

        if !ROLE_NAME_REGEX.regex.is_match(&trimmed) {
            return Err((CategoryError::Role, TypeError::Format { format: ROLE_NAME_REGEX.name.into() }).into());
        }

        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn normalized(&self) -> String {
        let mut s = self.0.clone();
        if let Some(first) = s.get_mut(0..1) {
            first.make_ascii_uppercase();
        }
        s
    }
}

impl Display for RoleName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for RoleName {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        RoleName::new(value)
    }
}

impl FromStr for RoleName {
    type Err = UserDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
