use std::fmt;
use std::convert::TryFrom;
use regex::Regex;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
    PHONE_CONTRY_REGEX,
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

        let country_code_cleaned: String = country_code_trimmed.chars().filter(|c| !c.is_whitespace()).collect();
        let number_cleaned: String = number_trimmed.chars().filter(|c| !c.is_whitespace()).collect();

        let country_code_len = country_code_cleaned.len();
        const MIN_PHONE_CONTRY_LEN: usize = 1;
        const MAX_PHONE_CONTRY_LEN: usize = 3;

        if country_code_len < MIN_PHONE_CONTRY_LEN {
            return Err((CategoryError::Phone, TypeError::TooShort { short: MIN_PHONE_CONTRY_LEN as u16 }).into());
        }

        if country_code_len > MAX_PHONE_CONTRY_LEN {
            return Err((CategoryError::Phone, TypeError::TooLong { long: MAX_PHONE_CONTRY_LEN as u32 }).into());
        }

        if !PHONE_CONTRY_REGEX.regex.is_match(&country_code_cleaned) {
            return Err((CategoryError::Phone, TypeError::Format { format: PHONE_CONTRY_REGEX.name.into() }).into());
        }

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

        Ok(Self {
            country_code: country_code_cleaned.to_string(),
            number: number_cleaned.to_string(),
        })
    }

    pub fn from_full(value: &str) -> Result<Self, UserDomainError> {
        let cleaned: String = value.chars().filter(|c| !c.is_whitespace()).collect();
        let regex = Regex::new(r"^(\+\d{1,3})(\d{6,14})$").unwrap();

        if let Some(caps) = regex.captures(&cleaned) {
            let country_code = caps.get(1).unwrap().as_str();
            let number = caps.get(2).unwrap().as_str();
            return Phone::new(country_code, number);
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
        write!(f, "{} {}", self.country_code, self.number)
    }
}

impl TryFrom<&str> for Phone {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Phone::from_full(value)
    }
}
