use std::fmt;
use std::convert::TryFrom;

use super::ValidationError;

/// Tipos de autenticación soportados.
/// Esto puede crecer según las necesidades del dominio.
pub enum AuthType {
    Password,
    Oidc,
    Saml,
}

impl AuthType {
    /// Crea un `AuthType` a partir de un &str validando el valor.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        match value.to_lowercase().as_str() {
            "password" => Ok(AuthType::Password),
            "oidc" => Ok(AuthType::Oidc),
            "saml" => Ok(AuthType::Saml),
            _ => Err(ValidationError::InvalidAuthType),
        }
    }

    /// Devuelve el `AuthType` como `&str` para persistencia o serialización.
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
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AuthType::new(value)
    }
}
