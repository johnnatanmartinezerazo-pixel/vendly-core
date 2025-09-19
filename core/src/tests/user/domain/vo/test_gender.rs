#[cfg(test)]
mod tests_gender {
    use crate::user::domain::{Gender, ValidationError};
    use std::convert::TryFrom;

    #[test]
    fn test_gender_valid() {
        let valid_cases = vec![
            ("male", Gender::Male, "male"),
            ("female", Gender::Female, "female"),
            ("non_binary", Gender::NonBinary, "non_binary"),
            ("non-binary", Gender::NonBinary, "non_binary"),
            ("other", Gender::Other, "other"),
            ("prefer_not_to_say", Gender::PreferNotToSay, "prefer_not_to_say"),
            ("prefer-not-to-say", Gender::PreferNotToSay, "prefer_not_to_say"),
        ];

        for (input, expected_enum, expected_str) in valid_cases {
            let gender = Gender::try_from(input).unwrap();
            println!("Probando válido '{}': {:?}", input, gender);
            assert_eq!(gender, expected_enum);
            assert_eq!(gender.as_str(), expected_str);
            assert_eq!(gender.to_string(), expected_str);
        }
    }

    #[test]
    fn test_gender_invalid() {
        let invalid_inputs = vec!["man", "woman", "nb", "unspecified", "123"];
        for v in invalid_inputs {
            let result = Gender::try_from(v);
            println!("Probando inválido '{}': {:?}", v, result);
            assert!(matches!(result, Err(ValidationError::InvalidGender)));
        }
    }

    #[test]
    fn test_display_trait() {
        let gender = Gender::NonBinary;
        println!("Display de {:?} = {}", gender, gender);
        assert_eq!(gender.to_string(), "non_binary");
    }
}
