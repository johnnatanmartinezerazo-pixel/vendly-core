use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
    USERNAME_REGEX,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    /// Longitudes mínimas y máximas recomendadas
    const MIN_USERNAME_LEN: usize = 3;
    const MAX_USERNAME_LEN: usize = 32;

    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::Username, TypeError::Empty).into());
        }

        let len = trimmed.len();
        if len < Self::MIN_USERNAME_LEN {
            return Err((CategoryError::Username, TypeError::TooShort { short: Self::MIN_USERNAME_LEN as u16 }).into());
        }

        if len > Self::MAX_USERNAME_LEN {
            return Err((CategoryError::Username, TypeError::TooLong { long: Self::MAX_USERNAME_LEN as u32 }).into());
        }

        if !USERNAME_REGEX.regex.is_match(trimmed) {
            return Err((CategoryError::Username, TypeError::Format { format: USERNAME_REGEX.name.into() }).into());
        }

        Ok(Self(trimmed.to_ascii_lowercase()))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

impl TryFrom<&str> for Username {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for Username {
    type Err = UserDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
