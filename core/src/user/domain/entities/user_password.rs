use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::ValidationError;

/// Representa las credenciales de un usuario.
/// En un sistema real, el hash debe generarse usando librerías seguras
/// (ej. Argon2, bcrypt, scrypt), nunca guardar la contraseña en texto plano.
#[derive(Debug, Clone, PartialEq)]
pub struct UserPassword {
    pub password_id: Uuid,
    pub user_id: Uuid,
    pub password_hash: String,
    pub password_salt: Option<String>,
    pub reset_token: Option<String>,
    pub reset_token_expires: Option<DateTime<Utc>>,
    pub failed_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserPassword {
    /// Crea un nuevo `UserPassword` validado.
    pub fn new(
        password_id: Uuid,
        user_id: Uuid,
        password_hash: String,
        password_salt: Option<String>,
        reset_token: Option<String>,
        reset_token_expires: Option<DateTime<Utc>>,
        failed_attempts: Option<i32>,
        locked_until: Option<DateTime<Utc>>,
        created_at: Option<DateTime<Utc>>,
        updated_at: Option<DateTime<Utc>>,
    ) -> Result<Self, ValidationError> {
        // Validación de seguridad: el hash no puede estar vacío
        if password_hash.trim().is_empty() {
            return Err(ValidationError::InvalidAuthType); // si quieres, crea un `InvalidPassword`
        }

        Ok(Self {
            password_id,
            user_id,
            password_hash,
            password_salt,
            reset_token,
            reset_token_expires,
            failed_attempts: failed_attempts.unwrap_or(0),
            locked_until,
            created_at: created_at.unwrap_or_else(Utc::now),
            updated_at: updated_at.unwrap_or_else(Utc::now),
        })
    }

    /// Incrementa contador de intentos fallidos.
    pub fn register_failed_attempt(&mut self) {
        self.failed_attempts += 1;
    }

    /// Resetea intentos fallidos después de un login exitoso.
    pub fn reset_failed_attempts(&mut self) {
        self.failed_attempts = 0;
    }

    /// Marca la contraseña como bloqueada hasta una fecha dada.
    pub fn lock_until(&mut self, until: DateTime<Utc>) {
        self.locked_until = Some(until);
    }

    /// Genera un token de reseteo de contraseña.
    pub fn set_reset_token(&mut self, token: String, expires_at: DateTime<Utc>) {
        self.reset_token = Some(token);
        self.reset_token_expires = Some(expires_at);
    }

    /// Limpia el token de reseteo después de usarlo.
    pub fn clear_reset_token(&mut self) {
        self.reset_token = None;
        self.reset_token_expires = None;
    }

    /// Actualiza el hash de la contraseña.
    pub fn update_password(&mut self, new_hash: String, new_salt: Option<String>) -> Result<(), ValidationError> {
        if new_hash.trim().is_empty() {
            return Err(ValidationError::InvalidAuthType);
        }

        self.password_hash = new_hash;
        self.password_salt = new_salt;
        self.updated_at = Utc::now();

        Ok(())
    }
}
