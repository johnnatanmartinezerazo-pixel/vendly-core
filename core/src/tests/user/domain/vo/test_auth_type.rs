#[cfg(test)]
mod tests_auth_type {
    use crate::user::domain::AuthType;
    use std::convert::TryFrom;

    #[test]
    fn test_auth_type_valid() {
        let values = vec!["password", "oauth", "saml", "oidc", "mfa"];
        for input in values {
          let result = AuthType::try_from(input);

          match result {
              Ok(_) => panic!("'{}' debería fallar", input),
              Err(e) => {
                  // Aquí usas Display ({}), no Debug ({:?})
                  println!("Probando inválido '{}': error -> {}", input, e);
              }
          }
      }
    }

    #[test]
    fn test_auth_type_invalid() {
        let invalid_values = vec!["token", "jwt", "google", "fb", "123"];
        for input in invalid_values {
          let result = AuthType::try_from(input);

          match result {
              Ok(_) => panic!("'{}' debería fallar", input),
              Err(e) => {
                  // Aquí usas Display ({}), no Debug ({:?})
                  println!("Probando inválido '{}': error -> {}", input, e);
              }
          }
      }
    }

    #[test]
    fn test_display_trait() {
        let auth = AuthType::Password;
        println!("Display de AuthType::Password: {}", auth);
        assert_eq!(auth.to_string(), "password");
    }
}
