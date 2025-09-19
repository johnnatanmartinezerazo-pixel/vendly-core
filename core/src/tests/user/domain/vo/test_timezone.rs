#[cfg(test)]
mod tests_timezone {
    use crate::user::domain::Timezone;
    use std::convert::TryFrom;

    #[test]
    fn test_timezone_valid() {
        let valid_cases = vec![
            "UTC",
            "America/New_York",
            "Europe/Madrid",
            "Asia/Tokyo",
            "America/Bogota",
        ];

        for input in valid_cases {
            let tz = Timezone::try_from(input).unwrap();
            println!("Probando válido '{}': {:?}", input, tz);
            assert_eq!(tz.as_str(), input);
            assert_eq!(tz.to_string(), input);

            // Validar que se pueda convertir a chrono_tz::Tz sin panic
            let chrono_tz = tz.as_tz();
            println!(" -> chrono_tz: {:?}", chrono_tz);
        }
    }

    #[test]
    fn test_timezone_invalid() {
        let invalid_cases = vec![
            "",
            "Invalid/Zone",
            "Mars/Base",
            "12345",
            "GMT+25", // offset inválido
        ];

        for input in invalid_cases {
            let result = Timezone::try_from(input);
            println!("Probando inválido '{}': {:?}", input, result);
            assert!(result.is_err(), "Se esperaba error para '{}'", input);
        }
    }
}
