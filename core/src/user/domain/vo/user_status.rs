use std::fmt;
use std::convert::TryFrom;

use super::ValidationError;

/// VO que representa el estado de un usuario en el sistema.
/// Siempre limitado a valores válidos y consistentes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserStatus {
    Pending,
    Active,
    Suspended,
    Deleted,
}

impl UserStatus {
    /// Crea un `UserStatus` desde &str validando que sea uno de los valores permitidos.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        match value.trim().to_lowercase().as_str() {
            "pending" => Ok(UserStatus::Pending),
            "active" => Ok(UserStatus::Active),
            "suspended" => Ok(UserStatus::Suspended),
            "deleted" => Ok(UserStatus::Deleted),
            _ => Err(ValidationError::InvalidUserStatus),
        }
    }

    /// Devuelve el estado como &str (ej: "active").
    pub fn as_str(&self) -> &str {
        match self {
            UserStatus::Pending => "pending",
            UserStatus::Active => "active",
            UserStatus::Suspended => "suspended",
            UserStatus::Deleted => "deleted",
        }
    }
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for UserStatus {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        UserStatus::new(value)
    }
}
