use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult };

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
    pub fn variants() -> &'static [&'static str] {
        &["pending", "active", "suspended", "deleted"]
    }

    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::Status, TypeError::Empty).into());
        }

        let lowered = trimmed.to_ascii_lowercase();

        match lowered.as_str() {
            "pending" => Ok(UserStatus::Pending),
            "active" => Ok(UserStatus::Active),
            "suspended" => Ok(UserStatus::Suspended),
            "deleted" => Ok(UserStatus::Deleted),
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

    pub fn is_active(&self) -> bool { matches!(self, UserStatus::Active) }
    pub fn is_pending(&self) -> bool { matches!(self, UserStatus::Pending) }
    pub fn is_suspended(&self) -> bool { matches!(self, UserStatus::Suspended) }
    pub fn is_deleted(&self) -> bool { matches!(self, UserStatus::Deleted) }
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for UserStatus {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        UserStatus::new(value)
    }
}

impl FromStr for UserStatus {
    type Err = UserDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}
