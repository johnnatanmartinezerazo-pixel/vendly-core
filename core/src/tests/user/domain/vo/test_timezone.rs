#[cfg(test)]
mod tests {
    use crate::user::domain::vo::Timezone;

    #[test]
    fn test_timezone_creation() {
        let inputs = vec![
            "America/Bogota",     // ✅ válido
            "Europe/London",      // ✅ válido
            "Asia/Tokyo",         // ✅ válido
            "UTC",                // ✅ válido
            "  America/New_York ",// ✅ válido (espacios)
            "invalid/timezone",   // ❌ no soportado
            "",                   // ❌ vacío
            " ",                  // ❌ solo espacios
            "Mars/Crater",        // ❌ no soportado
        ];

        for input in inputs {
            let result = Timezone::new(input);
            match result {
                Ok(tz) => println!("✅ '{input}' → creado como: {}", tz),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }

    #[test]
    fn test_timezone_display_and_as_tz() {
        let tz = Timezone::new("Europe/Berlin").unwrap();
        assert_eq!(tz.as_str(), "Europe/Berlin");
        assert_eq!(tz.as_tz().name(), "Europe/Berlin");
        println!("🧩 Display: {}", tz);
    }

    #[test]
    fn test_timezone_default() {
        let default_tz = Timezone::default();
        assert_eq!(default_tz.as_str(), "America/Bogota");
        println!("🌎 Default timezone: {}", default_tz);
    }
}
