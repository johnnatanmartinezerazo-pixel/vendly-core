#[cfg(test)]
mod tests {
    use crate::user::domain::vo::Email;

    #[test]
    fn test_email_creation() {
        let temp_max_email = "verylongemail".repeat(30);
        let max_email = temp_max_email.as_str();
        let inputs = vec![
            "user@example.com",             // válido
            " USER@Example.COM  ",          // válido (espacios + mayúsculas)
            "a@b.c",                        // error: demasiado corto
            "",                             // error: vacío
            "   ",                          // error: solo espacios
            "no-at-symbol.com",             // error: formato inválido
            "user@domain",                  // error: formato inválido
            "user@domain.",                 // error: formato inválido
            "user@@domain.com",             // error: formato inválido
            max_email,                      // error: demasiado largo (>254)
            "valid_user+tag@gmail.com",     // válido
            "user@-cast.sub.domain.co",           // válido
            "user@domain.c",                // error: dominio demasiado corto
        ];

        for input in inputs {
            let result = Email::new(&input);

            match result {
                Ok(email) => println!("✅ '{input}' → creado como: {}", email),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }
}
