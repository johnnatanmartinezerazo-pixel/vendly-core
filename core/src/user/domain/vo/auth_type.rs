use std::fmt;
use std::convert::TryFrom;

use sea_orm::TryIntoModel;

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
    pub fn new(value: &str) -> Result<Self, UserDomainError> {
        let trimmed = value.trim().to_lowercase();

        if trimmed.is_empty() {
            return Err((CategoryError::AuthType, TypeError::Empty).into());
        }

        match trimmed {
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

impl fmt::Display for AuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for AuthType {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AuthType::new(value)
    }
}
