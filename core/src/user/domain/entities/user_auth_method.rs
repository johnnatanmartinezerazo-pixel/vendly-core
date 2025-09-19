use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::{AuthType, ValidationError};

/// Representa un método de autenticación asociado a un usuario.
/// Ejemplo: password, google, microsoft, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct UserAuthMethod {
    pub auth_method_id: Uuid,
    pub user_id: Uuid,
    pub auth_type: AuthType,           // VO ya definido
    pub provider: Option<String>,      // ej: "google", "facebook"
    pub provider_user_id: Option<String>, // ID externo del proveedor
    pub is_primary: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

impl UserAuthMethod {
    /// Constructor seguro para crear un nuevo método de autenticación
    pub fn new(
        auth_method_id: Uuid,
        user_id: Uuid,
        auth_type: AuthType,
        provider: Option<&str>,
        provider_user_id: Option<&str>,
        is_primary: bool,
        is_verified: bool,
        created_at: Option<DateTime<Utc>>,
        last_used_at: Option<DateTime<Utc>>,
    ) -> Result<Self, ValidationError> {
        // Reglas de negocio simples:
        // 1. Si el auth_type es externo (ej: Google), debe haber provider y provider_user_id
        if matches!(auth_type, AuthType::Google | AuthType::Microsoft | AuthType::Oidc | AuthType::Saml) {
            if provider.is_none() || provider_user_id.is_none() {
                return Err(ValidationError::InvalidExternalIdEmpty);
        }
}


        Ok(Self {
            auth_method_id,
            user_id,
            auth_type,
            provider: provider.map(|p| p.to_string()),
            provider_user_id: provider_user_id.map(|id| id.to_string()),
            is_primary,
            is_verified,
            created_at: created_at.unwrap_or_else(Utc::now),
            last_used_at,
        })
    }

    /// Marcar este método como usado recientemente
    pub fn mark_as_used(&mut self) {
        self.last_used_at = Some(Utc::now());
    }

    /// Indica si este método es externo (ej: Google, Facebook)
    pub fn is_external(&self) -> bool {
        self.provider.is_some()
    }
}
