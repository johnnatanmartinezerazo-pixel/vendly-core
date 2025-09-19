#[cfg(test)]
mod tests_role_name {
    use crate::user::domain::SubscriptionTier;
    use std::convert::TryFrom;

    #[test]
    fn test_subscription_tier_valid() {
        let valid_cases = vec![
            ("free", SubscriptionTier::Free),
            ("basic", SubscriptionTier::Basic),
            ("premium", SubscriptionTier::Premium),
            ("enterprise", SubscriptionTier::Enterprise),
            ("FREE", SubscriptionTier::Free),         // mayúsculas -> normaliza
            ("Premium", SubscriptionTier::Premium),   // mixto -> normaliza
        ];

        for (input, expected) in valid_cases {
            let tier = SubscriptionTier::try_from(input).unwrap();
            println!("Probando válido '{}': {:?}", input, tier);
            assert_eq!(tier, expected);
            assert_eq!(tier.as_str(), expected.as_str());
            assert_eq!(tier.to_string(), expected.as_str());
        }
    }

    #[test]
    fn test_subscription_tier_invalid() {
        let invalid_cases = vec![
            "",          // vacío
            "fremium",   // typo
            "pro",       // inexistente
            "gold",      // no soportado
            "platinum",  // no soportado
            "standard",  // no soportado
        ];

        for input in invalid_cases {
            let result = SubscriptionTier::try_from(input);
            println!("Probando inválido '{}': {:?}", input, result);
            assert!(result.is_err(), "Se esperaba error para '{}'", input);
        }
    }
}
