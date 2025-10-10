use chrono::{DateTime, Utc};
use std::convert::TryFrom;

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

    pub fn value(&self) -> DateTime<Utc> {
        self.0
    }
}

impl TryFrom<&str> for OccurredAt {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<DateTime<Utc>>() {
            Ok(dt) => Ok(OccurredAt(dt)),
            Err(_) => Err((CategoryError::ConsentType, TypeError::Format { format: "datetime-utc".into() }).into()),
        }
    }
}
