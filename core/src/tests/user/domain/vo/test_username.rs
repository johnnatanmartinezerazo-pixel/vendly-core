#[cfg(test)]
mod tests_username {
    use crate::user::domain::Username;
    use std::convert::TryFrom;

    #[test]
    fn test_username_valid() {
        let max_len = "a".to_string() + &"b".repeat(29); // 30 chars

        let valid_cases = vec![
            "abcdef",          // mínimo 6
            "user.name",
            "user_name",
            "User123",
            "a12345",          // empieza con letra + números
            "a.b_c.d",         // mezcla permitida
            max_len.as_str(),  // longitud máxima
            "   trimmedUser   ", // espacios al inicio/fin
        ];

        for input in valid_cases {
            let username = Username::try_from(input).unwrap();
            println!("Probando válido '{}': {}", input, username);
            assert!(username.as_str().len() >= 6 && username.as_str().len() <= 30);
            assert_eq!(username.to_string(), username.as_str());
        }
    }

    #[test]
    fn test_username_invalid() {
        let too_short = "abc";              // < 6
        let too_long = "a".repeat(31);      // > 30

        let invalid_cases = vec![
            "",             // vacío
            too_short,
            too_long.as_str(),
            "1username",    // no empieza con letra
            ".username",    // no empieza con letra
            "_username",    // no empieza con letra
            "user__name",   // secuencia inválida
            "user..name",   // secuencia inválida
            "user-name",    // caracter no permitido '-'
            "user@name",    // caracter no permitido '@'
            "user name",    // espacio en medio
        ];

        for input in invalid_cases {
            let result = Username::try_from(input);
            println!("Probando inválido '{}': {:?}", input, result);
            assert!(result.is_err(), "Se esperaba error para '{}'", input);
        }
    }
}
