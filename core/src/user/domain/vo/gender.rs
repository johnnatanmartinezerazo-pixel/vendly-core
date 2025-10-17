use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

impl Gender {
    pub const VALUES: [&'static str; 4] = [
        "male",
        "female",
        "other",
        "prefer_not_to_say",
    ];

    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::Gender, TypeError::Empty).into());
        }

        let lowered = trimmed.to_ascii_lowercase();

        match lowered.as_str() {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "other" => Ok(Gender::Other),
            "prefer_not_to_say" => Ok(Gender::PreferNotToSay),
            _ => Err((CategoryError::Gender, TypeError::NotSupported).into()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Other => "other",
            Gender::PreferNotToSay => "prefer_not_to_say",
        }
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for Gender {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Gender::new(value)
    }
}

impl FromStr for Gender {
    type Err = UserDomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Gender::new(value)
    }
}
