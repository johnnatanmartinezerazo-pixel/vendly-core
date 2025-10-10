#[cfg(test)]
mod tests {
    use crate::user::domain::vo::Phone;

    #[test]
    fn test_phone_creation_with_parts() {
        let cases = vec![
            ("+57", "3201234567"),    // ✅ válido (Colombia)
            ("+1", "2025550147"),     // ✅ válido (EE.UU.)
            ("+44", "7700901234"),    // ✅ válido (Reino Unido)
            ("57", "3201234567"),     // ✅ válido (sin '+')
            ("+49", "15123456789"),   // ✅ válido (Alemania)
            ("+", "1234567"),         // ❌ código país inválido
            ("+1234", "9876543210"),  // ❌ país demasiado largo (>3)
            ("+57", "123"),           // ❌ número demasiado corto (<6)
            ("+57", "123456789012345"), // ❌ número demasiado largo (>14)
            ("+57", "12A45678"),      // ❌ formato inválido (letra)
            ("", ""),                 // ❌ ambos vacíos
            ("  ", "  "),             // ❌ espacios
        ];

        for (cc, num) in cases {
            let result = Phone::new(cc, num);

            match result {
                Ok(phone) => println!("✅ '{}' '{}' → creado como: {}", cc, num, phone),
                Err(err) => println!("❌ '{}' '{}' → error: {}", cc, num, err),
            }
        }
    }

    #[test]
    fn test_phone_creation_from_full() {
        let inputs = vec![
            "+573201234567",   // ✅ válido
            "+12025550147",    // ✅ válido
            "+447700901234",   // ✅ válido
            " +57 320 123 4567 ", // ✅ válido con espacios
            "+123456",         // ✅ válido (mínimo aceptable)
            "573201234567",    // ❌ falta '+'
            "+57320",          // ❌ demasiado corto
            "+5732012345678901", // ❌ demasiado largo
            "+57ABC1234",      // ❌ formato inválido
            "",                // ❌ vacío
        ];

        for input in inputs {
            let result = Phone::from_full(input);

            match result {
                Ok(phone) => println!("✅ '{input}' → creado como: {}", phone.as_full()),
                Err(err) => println!("❌ '{input}' → error: {}", err),
            }
        }
    }

    #[test]
    fn test_phone_display_and_accessors() {
        let phone = Phone::new("+57", "3201234567").unwrap();
        assert_eq!(phone.country_code(), "+57");
        assert_eq!(phone.number(), "3201234567");
        assert_eq!(phone.as_full(), "+573201234567");
        println!("📱 Display: {}", phone);
    }
}
