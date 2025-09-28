#[cfg(test)]
mod tests_phone {
    use crate::user::domain::Phone;
    use std::convert::TryFrom;

    #[test]
    fn test_validated_phones() {
        let valid_phones = vec![
            "+573001234567",
            "+34123456789",
            "+441632960961",
            "+4915123456789",
            "+5511987654321",
            "+1 2025550192",
            "1 2025550192",
            "+44 7912345678",
            "202-555-0192",
            "202.555.0192",
            "(202) 555-0192",
            "2025550192",
            "+52 5512345678",
            "+34 61234567",
            "+(34) 612-34-56-78",
            "+593 612345678"
        ];

        for input in valid_phones {
            let result = Phone::try_from(input);

            match result {
                Ok(result) => {
                    // Aquí usas Display ({}), no Debug ({:?})
                    println!("Teléfono válido '{}': éxito -> {}", input, result);
                }
                Err(e) => println!("Teléfono inválido '{}': error -> {}", input, e),
            }
        }
    }

    #[test]
    fn test_display_trait() {
        let phone = Phone::try_from("+573001234567").unwrap();
        println!("Display = {}", phone);
        assert_eq!(phone.to_string(), "+573001234567");
    }
}
