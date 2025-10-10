#[cfg(test)]
mod tests {
    use crate::user::domain::vo::SubscriptionTier;

    #[test]
    fn test_subscription_tier_creation() {
        let inputs = vec![
            "free",         // ✅ válido
            "basic",        // ✅ válido
            "premium",      // ✅ válido
            "enterprise",   // ✅ válido
            "  Free  ",     // ✅ válido (espacios + mayúsculas)
            "PREMIUM",      // ✅ válido (mayúsculas)
            "",             // ❌ vacío
            " ",            // ❌ solo espacios
            "vip",          // ❌ no soportado
            "pro",          // ❌ no soportado
            "trial",        // ❌ no soportado
        ];

        for input in inputs {
            let result = SubscriptionTier::new(input);
            match result {
                Ok(tier) => println!("✅ '{input}' → creado como: {}", tier),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }

    #[test]
    fn test_subscription_tier_display_and_as_str() {
        let tier = SubscriptionTier::new("Premium").unwrap();
        assert_eq!(tier.as_str(), "premium");
        println!("🧩 Display: {}", tier);
    }
}
