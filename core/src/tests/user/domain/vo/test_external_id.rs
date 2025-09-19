#[cfg(test)]
mod tests_external_id {
    use crate::user::domain::{ExternalId, ValidationError};
    use std::convert::TryFrom;

    #[test]
    fn test_valid_external_id() {
        let valid_ids = vec![
            "google-oauth2|1234567890",
            "microsoft-abcdef",
            "github_999",
            "  trimmed-id  ", // con espacios al inicio/fin
        ];

        for v in valid_ids {
            let ext_id = ExternalId::try_from(v).unwrap();
            println!("Probando válido '{}': '{}'", v, ext_id);
            assert_eq!(ext_id.as_str(), v.trim());
            assert_eq!(ext_id.to_string(), v.trim());
        }
    }

    #[test]
    fn test_empty_external_id() {
        let values = vec!["", "   "];
        for v in values {
            let result = ExternalId::try_from(v);
            println!("Probando vacío '{}': {:?}", v, result);
            assert!(matches!(result, Err(ValidationError::InvalidExternalIdEmpty)));
        }
    }

    #[test]
    fn test_external_id_too_long() {
        let long_id = "a".repeat(256);
        let result = ExternalId::try_from(long_id.as_str());
        println!("Probando demasiado largo (len={}): {:?}", long_id.len(), result);
        assert!(matches!(result, Err(ValidationError::InvalidExternalIdLength)));
    }

    #[test]
    fn test_external_id_with_control_chars() {
        let bad_ids = vec!["abc\n123", "id\tid", "null\x00char"];
        for v in bad_ids {
            let result = ExternalId::try_from(v);
            println!("Probando inválido con control '{}': {:?}", v, result);
            assert!(matches!(result, Err(ValidationError::InvalidExternalIdFormat)));
        }
    }

    #[test]
    fn test_display_trait() {
        let ext_id = ExternalId::try_from("github_123").unwrap();
        println!("Display = {}", ext_id);
        assert_eq!(ext_id.to_string(), "github_123");
    }
}
