#[cfg(test)]
mod tests_user_factory {
    use uuid::Uuid;
    use crate::user::application::UserFactory;
    use crate::user::domain::ValidationError;
    use std::result::Result::{Ok, Err};

    #[test]
    fn test_user_creation_valid() {
        let result = UserFactory::create(
            Uuid::new_v4(),
            "alice@example.com",
            Some("Alice_mouse"),
            Some("+123456789"),
            Some("ext_123"),
            Some("pending"),
        );

        println!("Result: {:?}", result);
    }

    #[test]
    fn test_user_creation_invalid_email() {
        let result = UserFactory::create(
            Uuid::new_v4(),
            "bad@domain..com", // correo inválido
            None,
            None,
            None,
            None,
        );

        match result {
            Ok(result) => println!("Usuario creado (no debería pasar): {:?}", result),
            Err(e) => {
                println!("Error esperado: {}", e);
                assert_eq!(e, ValidationError::InvalidEmail);
            }
        }
    }

    #[test]
    fn test_user_creation_invalid_phone() {
        let result = UserFactory::create(
            Uuid::new_v4(),
            "bob@example.com",
            Some("12345"), // inválido porque le falta el "+"
            None,
            None,
            None,
        );

        println!("Result: {:?}", result);
        assert!(result.is_err(), "Debería fallar con teléfono inválido");
        let err = result.unwrap_err();
        assert_eq!(err, ValidationError::InvalidPhone);
    }
}
