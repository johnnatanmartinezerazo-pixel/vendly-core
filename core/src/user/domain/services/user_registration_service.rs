use uuid::Uuid;
use chrono::Utc;

use crate::user::domain::{
    entities::user::User,
    vo::{Email, Username, ValidationError},
    repositories::user_repository::UserRepository,
    events::user_registered::UserRegisteredEvent,
};

/// Servicio de dominio para el registro de usuarios.
pub struct UserRegistrationService<'a, R: UserRepository> {
    repository: &'a mut R,
}

impl<'a, R: UserRepository> UserRegistrationService<'a, R> {
    pub fn new(repository: &'a mut R) -> Self {
        Self { repository }
    }

    /// Registra un nuevo usuario validando email y username.
    pub fn register_user(
        &mut self,
        email_raw: &str,
        username_raw: &str,
    ) -> Result<User, ValidationError> {
        let email = Email::try_from(email_raw)?;
        let username = Username::try_from(username_raw)?;

        // Verificar unicidad en repositorio
        if self.repository.exists_by_email(&email)? {
            return Err(ValidationError::InvalidEmail);
        }
        if self.repository.exists_by_username(&username)? {
            return Err(ValidationError::InvalidUsernameFormat);
        }

        let user = User {
            user_id: Uuid::new_v4(),
            email,
            username: Some(username),
            external_id: None,
            phone: None,
            status: Default::default(), // pending por defecto
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        self.repository.save(&user)?;

        // Publicar evento de dominio
        let _event = UserRegisteredEvent::new(user.user_id, user.email.clone());
        // Aquí podrías enviarlo a un bus de eventos

        Ok(user)
    }
}
