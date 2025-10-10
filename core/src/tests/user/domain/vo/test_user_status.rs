#[cfg(test)]
mod tests {
    use crate::user::domain::vo::UserStatus;

    #[test]
    fn test_user_status_creation() {
        let inputs = vec![
            "pending",       // ✅ válido
            "active",        // ✅ válido
            "suspended",     // ✅ válido
            "deleted",       // ✅ válido
            "  Active  ",    // ✅ válido (espacios + mayúsculas)
            "PENDING",       // ✅ válido (mayúsculas)
            "",              // ❌ vacío
            "  ",            // ❌ solo espacios
            "unknown",       // ❌ no soportado
            "actived",       // ❌ no soportado
            "disable",       // ❌ no soportado
        ];

        for input in inputs {
            let result = UserStatus::new(input);

            match result {
                Ok(status) => println!("✅ '{input}' → creado como: {}", status),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }

    #[test]
    fn test_user_status_display_and_as_str() {
        let status = UserStatus::new("Active").unwrap();
        assert_eq!(status.as_str(), "active");
        println!("🧩 Display: {}", status);
    }
}
