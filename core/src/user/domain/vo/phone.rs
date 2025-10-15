use std::fmt;

use crate::user::domain::validations::{
    CategoryError,
    TypeError,
    UserDomainError
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phone {
    pub country_code: usize,
    pub number: usize,
}

impl Phone {
    pub fn new(country_code: &str, number: &str) -> Result<Self, UserDomainError> {
        let country_code_trimmed: String = country_code.chars().filter(|c| !c.is_whitespace()).collect();

        if country_code_trimmed.is_empty(){
            return Err((CategoryError::PhoneCountryCode, TypeError::Empty).into());
        }

        if !country_code_trimmed.chars().all(char::is_numeric) {
            return Err((CategoryError::PhoneCountryCode, TypeError::Format { format: "ONLY_DIGITS".into() }).into());
        }

        let contry_code_len = country_code_trimmed.len();
        const MIN_PHONE_CONTRY_LEN: usize = 1;
        const MAX_PHONE_CONTRY_LEN: usize = 3;

        if contry_code_len < MIN_PHONE_CONTRY_LEN {
            return Err((CategoryError::PhoneCountryCode, TypeError::TooShort { short: MIN_PHONE_CONTRY_LEN as u16 }).into());
        }

        if contry_code_len > MAX_PHONE_CONTRY_LEN {
            return Err((CategoryError::PhoneCountryCode, TypeError::TooLong { long: MAX_PHONE_CONTRY_LEN as u32 }).into());
        }

        let number_trimmed: String = number.chars().filter(|c| !c.is_whitespace()).collect();

        if number_trimmed.is_empty(){
            return Err((CategoryError::PhoneNumber, TypeError::Empty).into());
        }

        if !number_trimmed.chars().all(char::is_numeric) {
            return Err((CategoryError::PhoneNumber, TypeError::Format { format: "ONLY_DIGITS".into() }).into());
        }

        let number_len = number_trimmed.len();
        const MIN_PHONE_NUMBER_LEN: usize = 6;
        const MAX_PHONE_NUMBER_LEN: usize = 14;

        if number_len < MIN_PHONE_NUMBER_LEN {
            return Err((CategoryError::PhoneNumber, TypeError::TooShort { short: MIN_PHONE_NUMBER_LEN as u16 }).into());
        }

        if number_len > MAX_PHONE_NUMBER_LEN {
            return Err((CategoryError::PhoneNumber, TypeError::TooLong { long: MAX_PHONE_NUMBER_LEN as u32 }).into());
        }

        Ok(Self {
            country_code: country_code_trimmed.parse().unwrap(),
            number: number_trimmed.parse().unwrap(),
        })
    }

    pub fn as_full(&self) -> String {
        format!("{} {}", self.country_code, self.number)
    }

    pub fn country_code(&self) -> &usize {
        &self.country_code
    }

    pub fn number(&self) -> &usize {
        &self.number
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.country_code, self.number)
    }
}
