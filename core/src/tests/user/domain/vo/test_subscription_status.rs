#[cfg(test)]
mod tests {
    use crate::user::domain::vo::SubscriptionStatus;

    #[test]
    fn test_subscription_status_creation() {
        let inputs = vec![
            "active",        // ✅ válido
            "inactive",      // ✅ válido
            "pending",       // ✅ válido
            "canceled",      // ✅ válido
            "cancelled",     // ✅ válido (británico)
            "expired",       // ✅ válido
            "  Active  ",    // ✅ válido (espacios + mayúsculas)
            "PENDING",       // ✅ válido (mayúsculas)
            "",              // ❌ vacío
            "   ",           // ❌ solo espacios
            "paused",        // ❌ no soportado
            "terminated",    // ❌ no soportado
            "unknown",       // ❌ no soportado
        ];

        for input in inputs {
            let result = SubscriptionStatus::new(input);

            match result {
                Ok(status) => println!("✅ '{input}' → creado como: {}", status),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }

    #[test]
    fn test_subscription_status_display_and_as_str() {
        let status = SubscriptionStatus::new("Canceled").unwrap();
        assert_eq!(status.as_str(), "canceled");
        println!("🧩 Display: {}", status);
    }
}
