use std::fmt;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionStatus {
    Active,
    Inactive,
    Pending,
    Canceled,
    Expired,
}

impl SubscriptionStatus {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::SubscriptionStatus, TypeError::Empty).into());
        }

        match trimmed.to_ascii_lowercase().as_str() {
            "active" => Ok(SubscriptionStatus::Active),
            "inactive" => Ok(SubscriptionStatus::Inactive),
            "pending" => Ok(SubscriptionStatus::Pending),
            "canceled" | "cancelled" => Ok(SubscriptionStatus::Canceled),
            "expired" => Ok(SubscriptionStatus::Expired),
            _ => Err((CategoryError::SubscriptionStatus, TypeError::NotSupported).into()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Inactive => "inactive",
            SubscriptionStatus::Pending => "pending",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::Expired => "expired",
        }
    }
}

impl fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for SubscriptionStatus {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SubscriptionStatus::new(value)
    }
}
