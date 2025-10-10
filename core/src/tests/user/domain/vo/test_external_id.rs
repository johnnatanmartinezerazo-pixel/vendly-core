#[cfg(test)]
mod tests {
    use crate::user::domain::vo::ExternalId;

    #[test]
    fn test_external_id_creation() {
        let external_string = "x".repeat(255);
        let external: &str = &external_string;
        let inputs = vec![
            "1234567890abcdef",               // válido (16 chars)
            "   1234567890abcdef   ",         // válido (trim)
            "abcdef1234567890abcdef1234567890", // válido (>16 chars)
            "",                               // error: vacío
            "   ",                            // error: solo espacios
            "short-id",                       // error: TooShort
            external,                 // error: TooLong
            "external_id_1234567890", // válido
        ];

        for input in inputs {
            let result = ExternalId::new(input);

            match result {
                Ok(ext_id) => println!("✅ '{input}' → creado como: {}", ext_id),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }
}
