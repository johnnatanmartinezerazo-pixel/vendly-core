use std::fmt;
use std::convert::TryFrom;

use super::ValidationError;

/// Géneros soportados en el dominio.
/// Este VO asegura que solo se usen valores válidos.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Other,
    PreferNotToSay,
}

impl Gender {
    /// Crea un `Gender` validando el valor recibido.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        match value.to_lowercase().as_str() {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "non_binary" | "non-binary" => Ok(Gender::NonBinary),
            "other" => Ok(Gender::Other),
            "prefer_not_to_say" | "prefer-not-to-say" => Ok(Gender::PreferNotToSay),
            _ => Err(ValidationError::InvalidGender),
        }
    }

    /// Devuelve el género como `&str`.
    pub fn as_str(&self) -> &str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::NonBinary => "non_binary",
            Gender::Other => "other",
            Gender::PreferNotToSay => "prefer_not_to_say",
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for Gender {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Gender::new(value)
    }
}
