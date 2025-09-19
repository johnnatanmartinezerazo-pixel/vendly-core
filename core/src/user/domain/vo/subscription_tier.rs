use std::fmt;
use std::convert::TryFrom;

use super::ValidationError;

/// Niveles válidos de suscripción de usuario.
/// VO que encapsula las reglas de negocio.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionTier {
    Free,
    Basic,
    Premium,
    Enterprise,
}

impl SubscriptionTier {
    /// Crea un `SubscriptionTier` validando el valor recibido.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        match value.to_lowercase().as_str() {
            "free" => Ok(SubscriptionTier::Free),
            "basic" => Ok(SubscriptionTier::Basic),
            "premium" => Ok(SubscriptionTier::Premium),
            "enterprise" => Ok(SubscriptionTier::Enterprise),
            _ => Err(ValidationError::InvalidSubscriptionTier),
        }
    }

    /// Devuelve el tier como `&str`.
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
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SubscriptionTier::new(value)
    }
}
