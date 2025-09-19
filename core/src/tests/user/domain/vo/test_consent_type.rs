#[cfg(test)]
mod tests_consent_type {
    use crate::user::domain::ConsentType;
    use std::convert::TryFrom;

    #[test]
    fn test_consent_type_valid() {
        let values = vec![
            "terms_of_service",
            "privacy_policy",
            "marketing_emails",
            "data_retention",
        ];
        for v in values {
            let consent = ConsentType::try_from(v).unwrap();
            println!("Probando válido '{}': {:?}", v, consent);
            assert_eq!(consent.as_str(), v);
        }
    }

    #[test]
    fn test_consent_type_invalid() {
        let invalid_values = vec!["tos", "pp", "ads", "logs", "123"];
        for v in invalid_values {
            let result = ConsentType::try_from(v);
            println!("Probando inválido '{}': {:?}", v, result);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_display_trait() {
        let consent = ConsentType::PrivacyPolicy;
        println!("Display de ConsentType::PrivacyPolicy = {}", consent);
        assert_eq!(consent.to_string(), "privacy_policy");
    }
}
