use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult };

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Phone {
    pub country_code: u16,
    pub number: u64,
}

impl Phone {
    pub(crate) fn new(country_code: &str, number: &str) -> Result<Self, UserDomainError> {
        let country_code_trimmed: String = country_code.chars().filter(|c| !c.is_whitespace()).collect();

        if country_code_trimmed.is_empty() {
            return Err((CategoryError::PhoneCountryCode, TypeError::Empty).into());
        }

        if !country_code_trimmed.chars().all(char::is_numeric) {
            return Err((CategoryError::PhoneCountryCode, TypeError::Format { format: "ONLY_DIGITS".into() }).into());
        }

        let len = country_code_trimmed.len();
        const MIN_LEN: usize = 1;
        const MAX_LEN: usize = 3;

        if len < MIN_LEN {
            return Err((CategoryError::PhoneCountryCode, TypeError::TooShort { short: MIN_LEN as u16 }).into());
        }

        if len > MAX_LEN {
            return Err((CategoryError::PhoneCountryCode, TypeError::TooLong { long: MAX_LEN as u32 }).into());
        }

        let number_trimmed: String = number.chars().filter(|c| !c.is_whitespace()).collect();

        if number_trimmed.is_empty() {
            return Err((CategoryError::PhoneNumber, TypeError::Empty).into());
        }

        if !number_trimmed.chars().all(char::is_numeric) {
            return Err((CategoryError::PhoneNumber, TypeError::Format { format: "ONLY_DIGITS".into() }).into());
        }

        let len = number_trimmed.len();
        const MIN_NUM_LEN: usize = 6;
        const MAX_NUM_LEN: usize = 14;

        if len < MIN_NUM_LEN {
            return Err((CategoryError::PhoneNumber, TypeError::TooShort { short: MIN_NUM_LEN as u16 }).into());
        }

        if len > MAX_NUM_LEN {
            return Err((CategoryError::PhoneNumber, TypeError::TooLong { long: MAX_NUM_LEN as u32 }).into());
        }

        let country_code = country_code_trimmed.parse::<u16>().map_err(|_| {
            (CategoryError::PhoneCountryCode, TypeError::Format { format: "ONLY_DIGITS".into() })
        })?;

        let number = number_trimmed.parse::<u64>().map_err(|_| {
            (CategoryError::PhoneNumber, TypeError::Format { format: "ONLY_DIGITS".into() })
        })?;

        Ok(Self { country_code, number })
    }

    pub fn as_full(&self) -> String {
        format!("{} {}", self.country_code, self.number)
    }

    pub fn as_e164(&self) -> String {
        format!("+{}{}", self.country_code, self.number)
    }

    pub fn country_code(&self) -> u16 {
        self.country_code
    }

    pub fn number(&self) -> u64 {
        self.number
    }
}

impl Display for Phone {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "+{} {}", self.country_code, self.number)
    }
}

impl TryFrom<(&str, &str)> for Phone {
    type Error = UserDomainError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        Phone::new(value.0, value.1)
    }
}

impl FromStr for Phone {
    type Err = UserDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s.trim().trim_start_matches('+');
        let parts: Vec<&str> = cleaned.split(|c| c == ' ' || c == '-').collect();

        if parts.len() != 2 {
            return Err((CategoryError::Phone, TypeError::Format { format: "COUNTRY_CODE NUMBER".into() }).into());
        }

        Self::try_from((parts[0], parts[1]))
    }
}
