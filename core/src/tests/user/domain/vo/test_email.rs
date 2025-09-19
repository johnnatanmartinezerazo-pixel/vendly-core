#[cfg(test)]
mod tests_email {
    use crate::user::domain::Email;
    use std::convert::TryFrom;

    #[test]
    fn test_valid_emails() {
        let valid_emails = vec![
            "user@example.com",
            "USER@EXAMPLE.COM",
            "user.name+tag@sub.domain.org",
            " uSeR@Example.CoM ", // con espacios y mayúsculas
        ];

        for input in valid_emails {
            let result = Email::try_from(input);

            match result {
                Ok(result) => {
                    // Aquí usas Display ({}), no Debug ({:?})
                    println!("Probando válido '{}': éxito -> {}", input, result);
                }
                Err(e) => panic!("'{}' debería pasar pero falló con error: {}", input, e),
            }
        }
    }

    #[test]
    fn test_invalid_emails() {
        let rep = format!("{}@example.com", "a".repeat(310));
        let invalid_emails = vec![
            "no-at-symbol.com",
            "missing-domain@",
            "@missing-user.com",
            "bad@domain",
            "bad@domain..com",
            "with spaces@domain.com",
            "*@domain.com",
            "user@domain,com",
            "user@domain@com",
            "user@.com",
            "user@domain.c", // dominio demasiado corto
            rep.as_str(), // local demasiado largo
        ];

        for input in invalid_emails {
            let result = Email::try_from(input);

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
    fn test_email_too_long() {
        // 321 chars total: 310 'a' + "@example.com"
        let long_local = "a".repeat(310);
        let too_long = format!("{}@example.com", long_local);
        let result = Email::try_from(too_long.as_str());
        match result {
                Ok(_) => panic!("'{:?}' debería fallar", result),
                Err(e) => {
                    // Aquí usas Display ({}), no Debug ({:?})
                    println!("Probando email demasiado largo (len={}): {}", too_long.len(), e);
                }
            }
    }

    #[test]
    fn test_display_trait() {
        let email = Email::try_from("Test@Domain.com").unwrap();
        println!("Display = {}", email);
        assert_eq!(email.to_string(), "test@domain.com");
    }
}
