use std::fmt;
use std::convert::TryFrom;
use regex::Regex;

use super::ValidationError;

/// VO que representa un Locale (ej: es-ES, en-US).
/// Siempre válido según el formato estándar.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Locale(String);

impl Locale {
    /// Crea un nuevo `Locale` validando el valor.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let trimmed = value.trim();

        // Validar longitud (ej: "es-ES" = 5)
        if trimmed.len() < 2 || trimmed.len() > 10 {
            return Err(ValidationError::InvalidLocale);
        }

        // Regex para locales básicos: xx-XX
        let regex = Regex::new(r"^[a-z]{2,3}([-_][A-Z]{2})?$").unwrap();

        if regex.is_match(trimmed) {
            Ok(Self(trimmed.replace('_', "-"))) // Normalizamos a `es-ES`
        } else {
            Err(ValidationError::InvalidLocale)
        }
    }

    /// Devuelve el locale como `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Locale {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Locale {
    type Error = ValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Locale::new(value)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale("es-ES".to_string()) // valor por defecto
    }
}
