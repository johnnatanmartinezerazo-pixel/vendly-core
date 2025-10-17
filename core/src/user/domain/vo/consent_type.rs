use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

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
    pub const VALUES: [&'static str; 4] = [
        "terms_of_service",
        "privacy_policy",
        "marketing_emails",
        "data_retention",
    ];

    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::ConsentType, TypeError::Empty).into());
        }

        let lowered = trimmed.to_ascii_lowercase();

        match lowered.as_str() {
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

impl Display for ConsentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for ConsentType {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ConsentType::new(value)
    }
}

impl FromStr for ConsentType {
    type Err = UserDomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        ConsentType::new(value)
    }
}
