#[cfg(test)]
mod tests {
    use crate::user::domain::vo::AuthType;

    #[test]
    fn test_auth_type_creation() {
        let inputs = vec![
            "password",     // válido
            "oidc",         // válido
            "saml",         // válido
            "  OIDC  ",     // válido (espacios + mayúsculas)
            "",             // error: vacío
            "   ",          // error: solo espacios
            "api_key",      // error: no soportado
            "PaSsWoRd",     // válido (case-insensitive)
            "OAuth2",       // error: no soportado
            "SAML!",        // error: formato no soportado
        ];

        for input in inputs {
            let result = AuthType::new(input);

            match result {
                Ok(auth_type) => println!("✅ '{input}' → creado como: {}", auth_type),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }
}
