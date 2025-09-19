#[cfg(test)]
mod tests_user_status {
    use crate::user::domain::UserStatus;
    use std::convert::TryFrom;

    #[test]
    fn test_user_status_valid() {
        let valid_cases = vec![
            ("pending", UserStatus::Pending),
            ("active", UserStatus::Active),
            ("suspended", UserStatus::Suspended),
            ("deleted", UserStatus::Deleted),
            ("Active", UserStatus::Active),     // mayúscula -> normaliza
            ("  pending  ", UserStatus::Pending), // con espacios -> trim()
        ];

        for (input, expected) in valid_cases {
            let status = UserStatus::try_from(input).unwrap();
            println!("Probando válido '{}': {:?}", input, status);
            assert_eq!(status, expected);
            assert_eq!(status.as_str(), expected.as_str());
            assert_eq!(status.to_string(), expected.as_str());
        }
    }

    #[test]
    fn test_user_status_invalid() {
        let invalid_cases = vec![
            "",
            "pendding",   // typo
            "activ",      // incompleto
            "removed",    // inexistente
            "banned",     // no soportado
            "archived",   // no soportado
        ];

        for input in invalid_cases {
            let result = UserStatus::try_from(input);
            println!("Probando inválido '{}': {:?}", input, result);
            assert!(result.is_err(), "Se esperaba error para '{}'", input);
        }
    }
}
