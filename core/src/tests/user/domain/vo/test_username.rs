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
}
