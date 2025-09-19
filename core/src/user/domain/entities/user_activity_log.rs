use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

use crate::user::domain::vo::{ValidationError};

/// Representa un log de actividad de usuario.
/// Tiene identidad propia (log_id) y siempre se relaciona con un `user_id`.
#[derive(Debug, Clone, PartialEq)]
pub struct UserActivityLog {
    pub log_id: Uuid,
    pub user_id: Uuid,
    pub action_type: String,       // Podría ser VO si quieres restringir acciones permitidas
    pub action_details: Option<JsonValue>, // JSONB en DB
    pub ip_address: Option<String>,        // Podrías usar un VO tipo `IpAddress`
    pub user_agent: Option<String>,
    pub success: bool,
    pub error_details: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

impl UserActivityLog {
    /// Constructor seguro: solo asegura que `action_type` no esté vacío.
    pub fn new(
        log_id: Uuid,
        user_id: Uuid,
        action_type: &str,
        action_details: Option<JsonValue>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        success: bool,
        error_details: Option<JsonValue>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Self, ValidationError> {
        if action_type.trim().is_empty() {
            return Err(ValidationError::InvalidConsentType);
            // ⚠️ Cambia este error por uno específico si quieres (ej. `InvalidActionType`)
        }

        Ok(Self {
            log_id,
            user_id,
            action_type: action_type.to_string(),
            action_details,
            ip_address,
            user_agent,
            success,
            error_details,
            created_at: created_at.unwrap_or_else(Utc::now),
        })
    }

    /// Indica si el log representa un fallo
    pub fn is_failure(&self) -> bool {
        !self.success
    }
}
