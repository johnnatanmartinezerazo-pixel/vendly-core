use std::fmt;
use std::convert::TryFrom;
use regex::Regex;

use super::ValidationError;

/// VO que representa el nombre de usuario.
/// Garantiza que sea único, válido y cumpla con las restricciones de dominio.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    /// Crea un `Username` validando reglas de negocio.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let trimmed = value.trim();

        // Validar longitud
        if trimmed.len() < 6 || trimmed.len() > 30 {
            return Err(ValidationError::InvalidUsernameLength);
        }

        // Validar regex: debe empezar con letra, luego letras/números/._ permitidos
        let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9._]*$").unwrap();
        if !re.is_match(trimmed) {
            return Err(ValidationError::InvalidUsernameFormat);
        }

        // Validar que no contenga secuencias inválidas como ".." o "__"
        if trimmed.contains("..") || trimmed.contains("__") {
            return Err(ValidationError::InvalidUsernameFormat);
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Devuelve el username como &str.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Username {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Username::new(value)
    }
}
