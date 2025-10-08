use std::fmt;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionTier {
    Free,
    Basic,
    Premium,
    Enterprise,
}

impl SubscriptionTier {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();
        
        if trimmed.is_empty() {
            return Err((CategoryError::SubscriptionTier, TypeError::Empty).into());
        }

        match trimmed.to_lowercase().as_str() {
            "free" => Ok(SubscriptionTier::Free),
            "basic" => Ok(SubscriptionTier::Basic),
            "premium" => Ok(SubscriptionTier::Premium),
            "enterprise" => Ok(SubscriptionTier::Enterprise),
            _ => Err((CategoryError::SubscriptionTier, TypeError::NotSupported).into()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionTier::Free => "free",
            SubscriptionTier::Basic => "basic",
            SubscriptionTier::Premium => "premium",
            SubscriptionTier::Enterprise => "enterprise",
        }
    }
}

impl fmt::Display for SubscriptionTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for SubscriptionTier {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SubscriptionTier::new(value)
    }
}
