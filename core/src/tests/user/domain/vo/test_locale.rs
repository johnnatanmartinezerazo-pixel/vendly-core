#[cfg(test)]
mod tests_locale {
    use crate::user::domain::{Locale, ValidationError};
    use std::convert::TryFrom;

    #[test]
    fn test_locale_valid() {
        let valid_cases = vec![
            ("es", "es"),
            ("en", "en"),
            ("es-ES", "es-ES"),
            ("en-US", "en-US"),
            ("fr-FR", "fr-FR"),
            ("pt_BR", "pt-BR"), // normaliza "_" a "-"
        ];

        for (input, expected) in valid_cases {
            let locale = Locale::try_from(input).unwrap();
            println!("Probando válido '{}': {:?}", input, locale);
            assert_eq!(locale.as_str(), expected);
            assert_eq!(locale.to_string(), expected);
        }
    }

    #[test]
    fn test_locale_invalid_length() {
        let too_short = "e";
        let too_long = "abcdefghijk";
        for v in [too_short, too_long] {
            let result = Locale::try_from(v);
            println!("Probando inválido (longitud) '{}': {:?}", v, result);
            assert!(matches!(result, Err(ValidationError::InvalidLocale)));
        }
    }

    #[test]
    fn test_locale_invalid_format() {
        let invalid_cases = vec![
            "es-es-es",   // demasiado largo para el patrón
            "EN-us",      // minúsculas en región
            "spanish",    // palabra completa
            "es_",        // guion bajo sin región
            "123-ES",     // números no permitidos
        ];

        for v in invalid_cases {
            let result = Locale::try_from(v);
            println!("Probando inválido (formato) '{}': {:?}", v, result);
            assert!(matches!(result, Err(ValidationError::InvalidLocale)));
        }
    }

    #[test]
    fn test_display_trait() {
        let locale = Locale::try_from("pt_BR").unwrap();
        println!("Display = {}", locale);
        assert_eq!(locale.to_string(), "pt-BR");
    }
}
