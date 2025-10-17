use chrono::{DateTime, Utc};
use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccurredAt(DateTime<Utc>);

impl OccurredAt {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    pub fn value(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl Display for OccurredAt {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl TryFrom<&str> for OccurredAt {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err((CategoryError::OccurredAt, TypeError::Empty).into());
        }
        match value.parse::<DateTime<Utc>>() {
            Ok(dt) => Ok(OccurredAt(dt)),
            Err(_) => Err((CategoryError::OccurredAt, TypeError::Format { format: "datetime-utc".into() }).into()),
        }
    }
}

impl FromStr for OccurredAt {
    type Err = UserDomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
