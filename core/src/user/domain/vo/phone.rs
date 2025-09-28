use regex::Regex;
use std::fmt;
use std::convert::TryFrom;
use std::sync::LazyLock;

use super::ValidationError;

pub struct Phone(String);

static PHONE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\+[0-9]{7,15}$").unwrap()
});

impl Phone {
    /// Crea un nuevo `Phone` validando y normalizando el valor recibido.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        // Eliminar espacios
        let cleaned: String = value.chars().filter(|c| !c.is_whitespace()).collect();

        // Validar con regex (E.164)
        if !PHONE_REGEX.is_match(&cleaned) {
            return Err(ValidationError::InvalidPhone);
        }

        Ok(Self(cleaned))
    }

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
