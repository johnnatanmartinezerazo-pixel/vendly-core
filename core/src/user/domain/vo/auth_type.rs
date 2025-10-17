use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuthType {
    Password,
    Oidc,
    Saml,
}

impl AuthType {
    pub const VALUES: [&'static str; 3] = [ "password", "oidc", "saml"];

    pub(crate) fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err((CategoryError::AuthType, TypeError::Empty).into());
        }

        let lowered = trimmed.to_ascii_lowercase();

        match lowered.as_str() {
            "password" => Ok(AuthType::Password),
            "oidc" => Ok(AuthType::Oidc),
            "saml" => Ok(AuthType::Saml),
            _ => Err((CategoryError::AuthType, TypeError::NotSupported).into()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AuthType::Password => "password",
            AuthType::Oidc => "oidc",
            AuthType::Saml => "saml",
        }
    }
}

impl Display for AuthType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for AuthType {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AuthType::new(value)
    }
}

impl FromStr for AuthType {
    type Err = UserDomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        AuthType::new(value)
    }
}
