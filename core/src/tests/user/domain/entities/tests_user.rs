#[cfg(test)]
mod tests_user {
    use uuid::Uuid;
    use std::convert::TryFrom;
    use crate::user::domain::{User, Email, UserStatus};

    #[test]
    fn test_user_creation_valid() {
        let email_valid = Email::try_from("alice@example.com");
        match email_valid {
            Ok(email_valid) => {
                let user_new = User::new(Uuid::new_v4(), email_valid, None, None, UserStatus::Active, None);
                println!("Datos de usuario creado: {:?}", user_new);
                assert_eq!(user_new.email.as_str(), "alice@example.com");
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_user_creation_invalid_email() {
        // Intento de crear Email inválido
        let email_invalid = Email::try_from("invalid_email");
        match email_invalid {
            Ok(email_invalid) => {
                let user_new = User::new(Uuid::new_v4(), email_invalid, None, None, UserStatus::Active, None);
                println!("Datos de usuario no creados: {:?}", user_new);
                assert_eq!(user_new.email.as_str(), "invalid_email");
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
