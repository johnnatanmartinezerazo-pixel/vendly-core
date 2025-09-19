use std::fmt;
use std::convert::TryFrom;

use super::ValidationError;

/// Tipos de autenticación soportados.
/// Esto puede crecer según las necesidades del dominio.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuthType {
    // Métodos básicos
    Password,
    EmailOtp,
    PhoneOtp,
    Totp,
    WebAuthn,

    // Enterprise
    Oidc,   // OpenID Connect (genérico, ej: Okta, Keycloak)
    Saml,   // SAML 2.0 (legacy corporativo)

    // Proveedores públicos
    Google,
    Microsoft,
}

impl AuthType {
    /// Crea un `AuthType` a partir de un &str validando el valor.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        match value.to_lowercase().as_str() {
            "password" => Ok(AuthType::Password),
            "email_otp" => Ok(AuthType::EmailOtp),
            "phone_otp" => Ok(AuthType::PhoneOtp),
            "totp" => Ok(AuthType::Totp),
            "webauthn" => Ok(AuthType::WebAuthn),
            "oidc" => Ok(AuthType::Oidc),
            "saml" => Ok(AuthType::Saml),
            "google" => Ok(AuthType::Google),
            "microsoft" => Ok(AuthType::Microsoft),
            _ => Err(ValidationError::InvalidAuthType),
        }
    }

    /// Devuelve el `AuthType` como `&str` para persistencia o serialización.
    pub fn as_str(&self) -> &str {
        match self {
            AuthType::Password => "password",
            AuthType::EmailOtp => "email_otp",
            AuthType::PhoneOtp => "phone_otp",
            AuthType::Totp => "totp",
            AuthType::WebAuthn => "webauthn",
            AuthType::Oidc => "oidc",
            AuthType::Saml => "saml",
            AuthType::Google => "google",
            AuthType::Microsoft => "microsoft",
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
