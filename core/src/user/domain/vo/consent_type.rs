use std::fmt;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConsentType {
    TermsOfService,
    PrivacyPolicy,
    MarketingEmails,
    DataRetention,
}

impl ConsentType {
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::ConsentType, TypeError::Empty).into());
        }

        match trimmed.to_ascii_lowercase().as_str() {
            "terms_of_service" => Ok(ConsentType::TermsOfService),
            "privacy_policy" => Ok(ConsentType::PrivacyPolicy),
            "marketing_emails" => Ok(ConsentType::MarketingEmails),
            "data_retention" => Ok(ConsentType::DataRetention),
            _ => Err((CategoryError::ConsentType, TypeError::NotSupported).into()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConsentType::TermsOfService => "terms_of_service",
            ConsentType::PrivacyPolicy => "privacy_policy",
            ConsentType::MarketingEmails => "marketing_emails",
            ConsentType::DataRetention => "data_retention",
        }
    }
}

impl fmt::Display for ConsentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for ConsentType {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ConsentType::new(value)
    }
}
