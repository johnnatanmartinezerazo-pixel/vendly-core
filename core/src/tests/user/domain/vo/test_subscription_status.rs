#[cfg(test)]
mod tests_role_name {
    use crate::user::domain::SubscriptionStatus;
    use std::convert::TryFrom;

    #[test]
    fn test_subscription_status_valid() {
        let valid_cases = vec![
            ("active", SubscriptionStatus::Active),
            ("inactive", SubscriptionStatus::Inactive),
            ("pending", SubscriptionStatus::Pending),
            ("canceled", SubscriptionStatus::Canceled),
            ("cancelled", SubscriptionStatus::Canceled), // sinónimo aceptado
            ("expired", SubscriptionStatus::Expired),
        ];

        for (input, expected) in valid_cases {
            let status = SubscriptionStatus::try_from(input).unwrap();
            println!("Probando válido '{}': {:?}", input, status);
            assert_eq!(status, expected);
            assert_eq!(status.as_str(), expected.as_str());
            assert_eq!(status.to_string(), expected.as_str());
        }
    }

    #[test]
    fn test_subscription_status_invalid() {
        let invalid_cases = vec![
            "",              // vacío
            "activ",         // mal escrito
            "inactve",       // typo
            "pendding",      // mal escrito
            "cancel",        // incompleto
            "expires",       // forma incorrecta
            "deleted",       // estado no permitido
            "archived",      // estado inexistente
        ];

        for input in invalid_cases {
            let result = SubscriptionStatus::try_from(input);
            println!("Probando inválido '{}': {:?}", input, result);
            assert!(result.is_err(), "Se esperaba error para '{}'", input);
        }
    }
}
