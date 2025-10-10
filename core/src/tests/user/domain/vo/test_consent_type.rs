#[cfg(test)]
mod tests {
    use crate::user::domain::vo::ConsentType;

    #[test]
    fn test_consent_type_creation() {
        let inputs = vec![
            "terms_of_service",   // válido
            "privacy_policy",     // válido
            "marketing_emails",   // válido
            "data_retention",     // válido
            "  TERMS_OF_SERVICE  ", // válido (espacios + mayúsculas)
            "",                   // error: vacío
            "   ",                // error: solo espacios
            "newsletter",         // error: no soportado
            "tos",                // error: no soportado
            "privacy-policy",     // error: formato no soportado
        ];

        for input in inputs {
            let result = ConsentType::new(input);

            match result {
                Ok(consent_type) => println!("✅ '{input}' → creado como: {}", consent_type),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }
}

