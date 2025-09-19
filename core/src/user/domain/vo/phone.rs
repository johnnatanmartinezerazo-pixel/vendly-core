use std::fmt;
use std::convert::TryFrom;

use super::ValidationError;

/// Value Object para Phone.
/// Inmutable, validado y normalizado.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phone(String);

impl Phone {
    /// Crea un nuevo `Phone` validando y normalizando el valor recibido.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        // Eliminamos espacios en blanco
        let cleaned: String = value.chars().filter(|c| !c.is_whitespace()).collect();

        // Validación: debe comenzar con '+' y contener solo dígitos después
        if !cleaned.starts_with('+') {
            return Err(ValidationError::InvalidPhone);
        }

        // Validación: longitud mínima y máxima
        if cleaned.len() < 7 || cleaned.len() > 20 {
            return Err(ValidationError::InvalidPhone);
        }

        // Validación: que el resto sean solo dígitos
        if !cleaned[1..].chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::InvalidPhone);
        }

        Ok(Self(cleaned))
    }

    /// Devuelve el teléfono como `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Phone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Phone {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Phone::new(value)
    }
}
