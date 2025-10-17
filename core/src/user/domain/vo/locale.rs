use icu_locid::Locale as IcuLocale;
use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Locale(IcuLocale);

impl Locale {
    pub const DEFAULT: &'static str = "es-ES";

    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::Locale, TypeError::Empty).into());
        }

        match IcuLocale::from_str(trimmed) {
            Ok(locale) => Ok(Self(locale)),
            Err(_) => Err((CategoryError::Locale, TypeError::Format { format: "bcp47".into() }).into()),
        }
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }

    pub fn language(&self) -> &str {
        self.0.id.language.as_str()
    }

    pub fn region(&self) -> Option<&str> {
        self.0.id.region.as_ref().map(|r| r.as_str())
    }

    pub fn script(&self) -> Option<&str> {
        self.0.id.script.as_ref().map(|s| s.as_str())
    }

    pub fn as_icu(&self) -> &IcuLocale {
        &self.0
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Locale {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for Locale {
    type Err = UserDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self(IcuLocale::from_str(Self::DEFAULT).unwrap())
    }
}
