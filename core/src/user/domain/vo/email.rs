use regex::Regex;
use std::fmt;
use std::convert::TryFrom;

use super::ValidationError;

/// Value Object para Email.
/// Inmutable, siempre válido y normalizado.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// Crea un nuevo Email validando y normalizando el valor recibido.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let trimmed = value.trim().to_lowercase();

        // Regla: longitud máxima 320 (RFC 3696/5321)
        if trimmed.len() > 320 {
            return Err(ValidationError::InvalidEmail);
        }

        // Regex simple para emails válidos
        let email_regex = Regex::new(
            r"^[A-Za-z0-9](?:[A-Za-z0-9._%+-]{0,63}[A-Za-z0-9])?@[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?(?:\.[A-Za-z]{2,})+$"
        ).unwrap();

        if email_regex.is_match(&trimmed) {
            Ok(Self(trimmed))
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }

    /// Devuelve el email como `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Permite usar Email como &str (ej. en queries, logs, etc.)
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Permite mostrar el email en logs, UI, etc.
impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Permite crear un Email con `Email::try_from("foo@bar.com")`.
impl TryFrom<&str> for Email {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}
