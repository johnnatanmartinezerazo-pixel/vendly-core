#[cfg(test)]
mod tests_role_name {
    use crate::user::domain::{RoleName, ValidationError};
    use std::convert::TryFrom;

    #[test]
    fn test_role_name_valid() {
        let max_len = "a".repeat(50); // vive todo el scope del test

        let valid_cases = vec![
            ("Admin", "admin"),             // normaliza a minúsculas
            ("user_role", "user_role"),
            ("super-user", "super-user"),
            ("ABC123", "abc123"),
            ("xyz", "xyz"),                 // longitud mínima
            (max_len.as_str(), max_len.as_str()), // longitud máxima
        ];

        for (input, expected) in valid_cases {
            let role = RoleName::try_from(input).unwrap();
            println!("Probando válido '{}': '{}'", input, role);
            assert_eq!(role.as_str(), expected);
            assert_eq!(role.to_string(), expected);
        }
    }

    #[test]
    fn test_role_name_invalid_length() {
        let too_short = "ab"; // < 3
        let too_long = "a".repeat(51); // > 50
        for v in [too_short, too_long.as_str()] {
            let result = RoleName::try_from(v);
            println!("Probando inválido (longitud) '{}': {:?}", v, result);
            assert!(matches!(result, Err(ValidationError::InvalidRole)));
        }
    }

    #[test]
    fn test_role_name_invalid_chars() {
        let invalid_cases = vec![
            "role name",   // espacios no permitidos
            "role@name",   // símbolo inválido
            "role$name",   // símbolo inválido
            "rolé",        // acento no permitido
        ];

        for v in invalid_cases {
            let result = RoleName::try_from(v);
            println!("Probando inválido (caracteres) '{}': {:?}", v, result);
            assert!(matches!(result, Err(ValidationError::InvalidRole)));
        }
    }

    #[test]
    fn test_display_trait() {
        let role = RoleName::try_from("Manager").unwrap();
        println!("Display = {}", role);
        assert_eq!(role.to_string(), "manager"); // normaliza a minúsculas
    }
}

