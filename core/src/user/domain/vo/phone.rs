use std::fmt;
use std::convert::TryFrom;
use regex::Regex;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
    PHONE_NUMBER_REGEX,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phone {
    pub country_code: String,
    pub number: String,
}

impl Phone {
    pub fn new(country_code: &str, number: &str) -> Result<Self, UserDomainError> {
        let country_code_trimmed = country_code.trim();
        let number_trimmed = number.trim();

        if country_code_trimmed.is_empty() || number_trimmed.is_empty(){
            return Err((CategoryError::Phone, TypeError::Empty).into());
        }

        // --- INICIO DE CAMBIOS ---

        // 1. Manejar el '+' opcional, obteniendo solo la parte numérica.
        let numeric_cc = country_code_trimmed.strip_prefix('+').unwrap_or(country_code_trimmed);

        // 2. Validar la longitud de la parte numérica.
        const MIN_PHONE_CONTRY_LEN: usize = 1;
        const MAX_PHONE_CONTRY_LEN: usize = 3;
        if numeric_cc.len() < MIN_PHONE_CONTRY_LEN {
            return Err((CategoryError::Phone, TypeError::TooShort { short: MIN_PHONE_CONTRY_LEN as u16 }).into());
        }
        if numeric_cc.len() > MAX_PHONE_CONTRY_LEN {
            return Err((CategoryError::Phone, TypeError::TooLong { long: MAX_PHONE_CONTRY_LEN as u32 }).into());
        }

        // 3. Validar que la parte numérica solo contenga dígitos.
        if !numeric_cc.chars().all(char::is_numeric) {
            return Err((CategoryError::Phone, TypeError::Format { format: "COUNTRY_CODE(only digits)".into() }).into());
        }

        // --- FIN DE LA NUEVA LÓGICA ---

        let number_cleaned: String = number_trimmed.chars().filter(|c| !c.is_whitespace()).collect();
        let number_len = number_cleaned.len();
        const MIN_PHONE_NUMBER_LEN: usize = 6;
        const MAX_PHONE_NUMBER_LEN: usize = 14;

        if number_len < MIN_PHONE_NUMBER_LEN {
            return Err((CategoryError::Phone, TypeError::TooShort { short: MIN_PHONE_NUMBER_LEN as u16 }).into());
        }
        if number_len > MAX_PHONE_NUMBER_LEN {
            return Err((CategoryError::Phone, TypeError::TooLong { long: MAX_PHONE_NUMBER_LEN as u32 }).into());
        }
        if !PHONE_NUMBER_REGEX.regex.is_match(&number_cleaned) {
            return Err((CategoryError::Phone, TypeError::Format { format: PHONE_NUMBER_REGEX.name.into() }).into());
        }

        // 4. Guardar la versión normalizada del código de país (siempre con '+').
        Ok(Self {
            country_code: format!("+{}", numeric_cc),
            number: number_cleaned,
        })
    }

    // El resto del archivo no necesita cambios, ya que `from_full` llama a `new`.
    pub fn from_full(value: &str) -> Result<Self, UserDomainError> {
        let cleaned: String = value.trim().chars().filter(|c| !c.is_whitespace()).collect();
        // Esta Regex es un poco más flexible para códigos de país.
        let regex = Regex::new(r"^(?P<cc>\+\d{1,3})(?P<num>\d{6,14})$").unwrap();

        if let Some(caps) = regex.captures(&cleaned) {
            // Se reutiliza la lógica de `new` que ya corregimos.
            return Phone::new(&caps["cc"], &caps["num"]);
        }

        Err((CategoryError::Phone, TypeError::Format { format: "FULL_PHONE(+CCXXXXXXXXX)".into() }).into())
    }

    pub fn as_full(&self) -> String {
        format!("{}{}", self.country_code, self.number)
    }

    pub fn country_code(&self) -> &str {
        &self.country_code
    }

    pub fn number(&self) -> &str {
        &self.number
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // CAMBIO: Se añaden paréntesis al rededor del código de país.
        write!(f, "({}) {}", self.country_code, self.number)
    }
}

impl TryFrom<&str> for Phone {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Phone::from_full(value)
    }
}
