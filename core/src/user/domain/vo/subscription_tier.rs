use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

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
    pub const VALUES: [&'static str; 4] = [
        "free",
        "basic",
        "premium",
        "enterprise",
    ];

    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::SubscriptionTier, TypeError::Empty).into());
        }

        let lowered = trimmed.to_ascii_lowercase();

        match lowered.as_str() {
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

impl Display for SubscriptionTier {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for SubscriptionTier {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SubscriptionTier::new(value)
    }
}

impl FromStr for SubscriptionTier {
    type Err = UserDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl Default for SubscriptionTier {
    fn default() -> Self {
        SubscriptionTier::Free
    }
}
