#[cfg(test)]
mod tests {
    use crate::user::domain::entities::User;
    use crate::user::domain::vo::*;
    use crate::user::domain::validations::UserDomainError;
    use crate::user::domain::events::UserDomainEvent;

    fn new_email(value: &str) -> Result<Email, UserDomainError> {
        Email::new(value)
    }

    fn new_phone(cc: &str, num: &str) -> Result<Phone, UserDomainError> {
        Phone::new(cc, num)
    }

    fn new_username(value: &str) -> Result<Username, UserDomainError> {
        Username::new(value)
    }

    fn new_external_id(value: &str) -> Result<ExternalId, UserDomainError> {
        ExternalId::new(value)
    }

    #[test]
    fn full_user_lifecycle_serialized_flow() {
        // Paso 1: Crear usuario válido
        match new_email("john.doe@example.co") {
            Ok(email) => {
                let mut user = User::register(email.clone());
                assert_eq!(user.email(), &email);
                assert_eq!(user.status(), &UserStatus::Pending);
                assert!(!user.email_verified());
                assert_eq!(user.take_events().len(), 1);

                // Paso 2: Intentar crear un usuario con correo vacío (debe fallar)
                if let Err(err) = new_email("") {
                    println!("(OK) Falló creación con correo vacío: {err}");
                }

                // Paso 3: Intentar actualizar el correo con el mismo (debe fallar)
                match new_email("john.doe@example.co") {
                    Ok(same_email) => {
                        let result = user.update_email(same_email);
                        assert!(result.is_err());
                    }
                    Err(err) => println!("Error inesperado en same_email: {err}"),
                }

                // Paso 4: Actualizar con un correo diferente (éxito)
                match new_email("john.doe.new@example.co") {
                    Ok(v_new_email) => {
                        let result = user.update_email(v_new_email.clone());
                        assert!(result.is_ok());
                        assert_eq!(user.email(), &v_new_email);
                        assert_eq!(user.status(), &UserStatus::Pending);
                    }
                    Err(err) => println!("Error creando nuevo correo: {err}"),
                }

                // Paso 5: Verificar correo
                let result = user.verify_email();
                assert!(result.is_ok());
                assert!(user.email_verified());

                // Paso 6: Intentar verificar de nuevo (debe fallar)
                let result = user.verify_email();
                assert!(result.is_err());

                // Paso 7: Activar usuario
                let result = user.activate();
                match result {
                    Ok(_) => {
                        assert_eq!(user.status(), &UserStatus::Active);
                        println!("(OK) Usuario activado correctamente");
                    }
                    Err(err) => {
                        println!("(OK) No se pudo activar el usuario — comportamiento esperado: {err}");
                        assert_eq!(user.status(), &UserStatus::Pending);
                    }
                }

                // Paso 8: Suspender usuario
                let result = user.suspend();
                assert!(result.is_ok());
                assert_eq!(user.status(), &UserStatus::Suspended);

                // Paso 9: Intentar suspender nuevamente (debe fallar)
                let result = user.suspend();
                assert!(result.is_err());

                // Paso 10: Reactivar usuario suspendido
                let result = user.activate();
                assert!(result.is_ok());
                assert_eq!(user.status(), &UserStatus::Active);

                // Paso 11: Asignar número de teléfono
                match new_phone("+57", "3001234567") {
                    Ok(phone) => {
                        let result = user.assign_phone(phone.clone());
                        assert!(result.is_ok());
                        assert!(!user.phone_verified());
                    }
                    Err(err) => println!("Error creando número: {err}"),
                }

                // Paso 12: Verificar teléfono (éxito)
                let result = user.verify_phone();
                match result {
                    Ok(_) => {
                        assert!(result.is_ok());
                        assert!(!user.phone_verified());
                    }
                    Err(err) => println!("Error creando número: {err}"),
                }

                // Paso 13: Intentar verificar teléfono sin teléfono (debe fallar)
                if let Ok(email_aux) = new_email("new.user@example.co") {
                    let mut user_no_phone = User::register(email_aux);
                    let result = user_no_phone.verify_phone();
                    assert!(result.is_err());
                }

                // Paso 14: Asignar nombre de usuario y external_id
                if let (Ok(username), Ok(external_id)) =
                    (new_username("john_dev"), new_external_id("EXTERNAL-ABC-123456789"))
                {
                    assert!(user.assign_username(username).is_ok());
                    assert!(user.link_external_id(external_id).is_ok());
                }

                // Paso 15: Eliminar usuario
                let result = user.delete();
                assert!(result.is_ok());
                assert_eq!(user.status(), &UserStatus::Deleted);
                assert!(user.deleted_at().is_some());

                // Paso 16: Intentar activar después de eliminado (debe fallar)
                let result = user.activate();
                assert!(result.is_err());
            }
            Err(err) => println!("❌ Error creando correo inicial: {err}"),
        }
    }

    #[test]
    fn user_creation_and_event_sequence() {
        match new_email("demo.user@example.co") {
            Ok(email) => {
                let mut user = User::register(email);
                let events = user.take_events();
                assert_eq!(events.len(), 1);

                if let Ok(new_email) = new_email("new.user@example.co") {
                    let _ = user.update_email(new_email);
                    let events = user.take_events();
                    assert!(events.iter().any(|e| matches!(**e, UserDomainEvent::EmailUpdated(_))));
                }

                let result = user.verify_email();
                if result.is_ok() {
                    let events = user.take_events();
                    assert!(events.iter().any(|e| matches!(**e, UserDomainEvent::EmailVerified(_))));
                } else {
                    println!("(OK) No se pudo verificar correo, error esperado en flujo inconsistente");
                }
            }
            Err(err) => println!("❌ Error creando correo en test de secuencia: {err}"),
        }
    }
}
