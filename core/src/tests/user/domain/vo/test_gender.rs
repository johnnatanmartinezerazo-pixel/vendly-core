#[cfg(test)]
mod tests {
    use crate::user::domain::vo::Gender;

    #[test]
    fn test_gender_creation() {
        let inputs = vec![
            "male",                 // válido
            "female",               // válido
            "non_binary",           // válido
            "non-binary",           // válido (guion)
            "other",                // válido
            "prefer_not_to_say",    // válido
            "prefer-not-to-say",    // válido (guion)
            "   Female   ",          // válido (espacios + mayúsculas)
            "",                     // error: vacío
            "   ",                  // error: solo espacios
            "unknown",              // error: no soportado
            "robot",                // error: no soportado
        ];

        for input in inputs {
            let result = Gender::new(input);

            match result {
                Ok(gender) => println!("✅ '{input}' → creado como: {}", gender),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }
}
