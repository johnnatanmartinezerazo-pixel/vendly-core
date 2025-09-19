use std::fmt;
use std::convert::TryFrom;

use super::ValidationError;

/// Estados válidos de una suscripción de usuario.
/// VO que garantiza consistencia con las reglas de negocio.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionStatus {
    Active,
    Inactive,
    Pending,
    Canceled,
    Expired,
}

impl SubscriptionStatus {
    /// Crea un `SubscriptionStatus` validando el valor recibido.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        match value.to_lowercase().as_str() {
            "active" => Ok(SubscriptionStatus::Active),
            "inactive" => Ok(SubscriptionStatus::Inactive),
            "pending" => Ok(SubscriptionStatus::Pending),
            "canceled" | "cancelled" => Ok(SubscriptionStatus::Canceled),
            "expired" => Ok(SubscriptionStatus::Expired),
            _ => Err(ValidationError::InvalidSubscriptionStatus),
        }
    }

    /// Devuelve el estado como &str (para persistencia/serialización).
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

impl fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for SubscriptionStatus {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SubscriptionStatus::new(value)
    }
}
