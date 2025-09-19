#[cfg(test)]
mod tests_phone {
    use crate::user::domain::{Phone, ValidationError};
    use std::convert::TryFrom;

    #[test]
    fn test_phone_valid() {
        let valid_cases = vec![
            ("+1234567", "+1234567"),         // mínimo válido
            ("+123456789012345678", "+123456789012345678"), // largo válido
            ("+57 300 123 4567", "+573001234567"), // con espacios
            ("+1 800 555 1234", "+18005551234"),
        ];

        for (input, expected) in valid_cases {
            let phone = Phone::try_from(input).unwrap();
            println!("Probando válido '{}': '{}'", input, phone);
            assert_eq!(phone.as_str(), expected);
            assert_eq!(phone.to_string(), expected);
        }
    }

    #[test]
    fn test_phone_invalid_no_plus() {
        let invalid = "123456789";
        let result = Phone::try_from(invalid);
        println!("Probando sin '+': {:?}", result);
        assert!(matches!(result, Err(ValidationError::InvalidPhone)));
    }

    #[test]
    fn test_phone_invalid_length() {
        let too_short = "+123"; // menos de 7
        let too_long = format!("+{}", "1".repeat(25)); // más de 20
        for v in [too_short, too_long.as_str()] {
            let result = Phone::try_from(v);
            println!("Probando longitud inválida '{}': {:?}", v, result);
            assert!(matches!(result, Err(ValidationError::InvalidPhone)));
        }
    }

    #[test]
    fn test_phone_invalid_characters() {
        let invalid_cases = vec!["+123-4567", "+12abc345", "+12#345678"];
        for v in invalid_cases {
            let result = Phone::try_from(v);
            println!("Probando inválido con caracteres raros '{}': {:?}", v, result);
            assert!(matches!(result, Err(ValidationError::InvalidPhone)));
        }
    }

    #[test]
    fn test_display_trait() {
        let phone = Phone::try_from("+573001234567").unwrap();
        println!("Display = {}", phone);
        assert_eq!(phone.to_string(), "+573001234567");
    }
}
