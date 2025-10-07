use std::fmt;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserStatus {
    Pending,
    Active,
    Suspended,
    Deleted,
}

impl UserStatus {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        match value.trim() {
            v if v.eq_ignore_ascii_case("pending") => Ok(UserStatus::Pending),
            v if v.eq_ignore_ascii_case("active") => Ok(UserStatus::Active),
            v if v.eq_ignore_ascii_case("suspended") => Ok(UserStatus::Suspended),
            v if v.eq_ignore_ascii_case("deleted") => Ok(UserStatus::Deleted),
            _ => Err((CategoryError::Status, TypeError::NotSupported).into()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            UserStatus::Pending => "pending",
            UserStatus::Active => "active",
            UserStatus::Suspended => "suspended",
            UserStatus::Deleted => "deleted",
        }
    }
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for UserStatus {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        UserStatus::new(value)
    }
}
