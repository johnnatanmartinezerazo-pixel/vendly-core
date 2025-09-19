use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

use crate::user::domain::vo::ValidationError;

/// Representa una sesión de usuario en el sistema.
///
/// - Cada sesión tiene un `refresh_token` y puede expirar.
/// - Contiene información del dispositivo y del cliente.
/// - Se utiliza para controlar autenticación y expiración de accesos.
#[derive(Debug, Clone, PartialEq)]
pub struct UserSession {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: Option<String>,
    pub access_token_version: i32,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_info: Option<JsonValue>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_activity_at: Option<DateTime<Utc>>,
}

impl UserSession {
    /// Crea una nueva sesión de usuario.
    pub fn new(
        session_id: Uuid,
        user_id: Uuid,
        refresh_token: Option<String>,
        expires_at: DateTime<Utc>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        device_info: Option<JsonValue>,
    ) -> Result<Self, ValidationError> {
        if expires_at <= Utc::now() {
            return Err(ValidationError::InvalidUserStatus); // Puedes definir un error más específico tipo InvalidSession
        }

        Ok(Self {
            session_id,
            user_id,
            refresh_token,
            access_token_version: 1,
            expires_at,
            ip_address,
            user_agent,
            device_info,
            is_active: true,
            created_at: Utc::now(),
            last_activity_at: None,
        })
    }

    /// Marca la sesión como inactiva (logout).
    pub fn terminate(&mut self) {
        self.is_active = false;
    }

    /// Verifica si la sesión sigue siendo válida.
    pub fn is_valid(&self) -> bool {
        self.is_active && self.expires_at > Utc::now()
    }

    /// Actualiza la última actividad de la sesión.
    pub fn touch(&mut self) {
        self.last_activity_at = Some(Utc::now());
    }

    /// Invalida todos los tokens de acceso incrementando la versión.
    pub fn invalidate_tokens(&mut self) {
        self.access_token_version += 1;
    }
}
