#[cfg(test)]
mod tests {
    use crate::user::domain::vo::Username;

    #[test]
    fn test_username_creation() {
        let inputs = vec![
            "john_doe",       // ✅ válido
            "Alice123",       // ✅ válido
            "  user_name  ",  // ✅ válido (espacios alrededor)
            "UPPERCASE",      // ✅ válido (se convierte a minúsculas)
            "user.name",      // ❌ formato inválido (si el regex no lo permite)
            "ab",             // ❌ demasiado corto según regex
            "user@domain",    // ❌ formato inválido
            "user name",      // ❌ contiene espacio
            "",               // ❌ vacío
            " ",              // ❌ solo espacios
        ];

        for input in inputs {
            let result = Username::new(input);
            match result {
                Ok(username) => println!("✅ '{input}' → creado como: '{}'", username),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }

    #[test]
    fn test_username_display_and_as_ref() {
        let username = Username::new("RustDev").unwrap();
        assert_eq!(username.as_str(), "rustdev");
        assert_eq!(username.as_ref(), "rustdev");
        println!("🧩 Display username: {}", username);
    }

    #[test]
    fn test_username_try_from_trait() {
        let username = Username::try_from("   example_user   ").unwrap();
        assert_eq!(username.as_str(), "example_user");
        println!("🔁 TryFrom conversion exitosa: {}", username);
    }
}
