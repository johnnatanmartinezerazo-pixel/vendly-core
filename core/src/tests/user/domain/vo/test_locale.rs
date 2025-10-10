#[cfg(test)]
mod tests {
    use crate::user::domain::vo::Locale;

    #[test]
    fn test_locale_creation() {
        let inputs = vec![
            "es",          // válido - español genérico
            "en",          // válido - inglés genérico
            "fr",          // válido - francés genérico
            "en-US",       // válido - inglés EE.UU.
            "es-ES",       // válido - español España
            "pt-BR",       // válido - portugués Brasil
            "ES",          // válido - se convierte a minúsculas
            "   es-CO   ", // válido - recorta espacios y normaliza
            "",            // error: vacío
            "  ",          // error: solo espacios
            "e",           // error: demasiado corto
            "english",     // error: demasiado largo
            "123",         // error: formato inválido
            "es_CO",       // error: formato inválido (usa guion bajo)
            "es-",         // error: formato inválido (termina en guion)
        ];

        for input in inputs {
            let result = Locale::new(input);

            match result {
                Ok(locale) => println!("✅ '{input}' → creado como: {}", locale),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }

    #[test]
    fn test_locale_default() {
        let default_locale = Locale::default();
        println!("🌐 Valor por defecto: {}", default_locale);
        assert_eq!(default_locale.as_str(), "es-ES");
    }
}
