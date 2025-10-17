use chrono_tz::Tz;
use std::sync::LazyLock;
use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

static TZ_NAMES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    chrono_tz::TZ_VARIANTS.iter().map(|tz| tz.name()).collect()
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Timezone(String);

impl Timezone {
    pub const DEFAULT: &'static str = "America/Bogota";

    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::Timezone, TypeError::Empty).into());
        }

        match trimmed.parse::<Tz>() {
            Ok(_) => Ok(Self(trimmed.to_string())),
            Err(_) => Err((CategoryError::Timezone, TypeError::NotSupported).into()),
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[inline]
    pub fn as_tz(&self) -> Tz {
        debug_assert!(
            self.0.parse::<Tz>().is_ok(),
            "Invalid Timezone: internal inconsistency (should be validated on creation)"
        );

        // Seguridad: unwrap es seguro tras validación en new()
        self.0.parse::<Tz>().unwrap()
    }

    #[inline]
    pub fn all() -> &'static [&'static str] {
        TZ_NAMES.as_slice()
    }
}

impl Display for Timezone {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

impl TryFrom<&str> for Timezone {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for Timezone {
    type Err = UserDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Default for Timezone {
    fn default() -> Self {
        Self(Self::DEFAULT.to_string())
    }
}
