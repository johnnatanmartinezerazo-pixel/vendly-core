use crate::user::domain::{
    entities::{user::User, user_auth_method::UserAuthMethod, user_password::UserPassword},
    vo::{AuthType, ValidationError},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use std::convert::TryFrom;

/// Servicio de dominio encargado de autenticar usuarios.
/// Aquí no se manejan tokens JWT ni sesiones (eso iría en application/infrastructure),
/// solo la lógica pura de autenticación.
pub struct AuthenticationService;

impl AuthenticationService {
    /// Autenticación mediante contraseña.
    pub fn authenticate_with_password(
        user: &User,
        password: &str,
        stored_password: &UserPassword,
    ) -> Result<(), ValidationError> {
        let parsed_hash = PasswordHash::new(stored_password.password_hash.as_str())
            .map_err(|_| ValidationError::InvalidPassword)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| ValidationError::InvalidPassword)
    }

    /// Autenticación mediante proveedor externo (ej: Google, Microsoft).
    pub fn authenticate_with_provider(
        user: &User,
        auth_methods: &[UserAuthMethod],
        auth_type: AuthType,
        provider_user_id: &str,
    ) -> Result<(), ValidationError> {
        // Filtrar solo los métodos activos de ese tipo
        let method = auth_methods.iter().find(|m| m.auth_type == auth_type);

        match method {
            Some(m) if m.provider_user_id.as_deref() == Some(provider_user_id) => Ok(()),
            _ => Err(ValidationError::InvalidAuthType),
        }
    }

    /// Verifica si el usuario tiene un método de autenticación válido.
    pub fn supports_auth_type(user: &User, methods: &[UserAuthMethod], auth_type: AuthType) -> bool {
        methods.iter().any(|m| m.auth_type == auth_type)
    }
}
