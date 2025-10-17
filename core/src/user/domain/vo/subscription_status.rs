use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

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
    pub const VALUES: [&'static str; 5] = [
        "active",
        "inactive",
        "pending",
        "canceled",
        "expired",
    ];

    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::SubscriptionStatus, TypeError::Empty).into());
        }

        let lowered = trimmed.to_ascii_lowercase();

        match lowered.as_str() {
            "active" => Ok(SubscriptionStatus::Active),
            "inactive" => Ok(SubscriptionStatus::Inactive),
            "pending" => Ok(SubscriptionStatus::Pending),
            "canceled" => Ok(SubscriptionStatus::Canceled),
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

impl Display for SubscriptionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for SubscriptionStatus {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SubscriptionStatus::new(value)
    }
}

impl FromStr for SubscriptionStatus {
    type Err = UserDomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        SubscriptionStatus::new(value)
    }
}
