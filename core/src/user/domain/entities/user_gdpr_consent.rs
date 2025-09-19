use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::net::IpAddr;

use crate::user::domain::vo::{ConsentType, ValidationError};

/// Representa un consentimiento GDPR otorgado por un usuario.
/// Ejemplo: marketing, analytics, términos y condiciones, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct UserGdprConsent {
    pub consent_id: Uuid,
    pub user_id: Uuid,
    pub consent_type: ConsentType, // VO que valida tipos de consentimiento válidos
    pub consent_given: bool,
    pub consent_details: Option<serde_json::Value>, // Detalles adicionales en JSON
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl UserGdprConsent {
    /// Crea un consentimiento validado
    pub fn new(
        consent_id: Uuid,
        user_id: Uuid,
        consent_type: ConsentType,
        consent_given: bool,
        consent_details: Option<serde_json::Value>,
        ip_address: Option<IpAddr>,
        user_agent: Option<&str>,
        expires_at: Option<DateTime<Utc>>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Self, ValidationError> {
        // Regla: Un consentimiento debe tener siempre tipo válido (lo valida el VO).
        // Regla: Si `consent_given` es false, aún así puede registrarse (rechazo explícito).
        Ok(Self {
            consent_id,
            user_id,
            consent_type,
            consent_given,
            consent_details,
            ip_address,
            user_agent: user_agent.map(|ua| ua.to_string()),
            expires_at,
            created_at: created_at.unwrap_or_else(Utc::now),
        })
    }

    /// Verifica si el consentimiento sigue siendo válido (no expirado)
    pub fn is_valid(&self) -> bool {
        match self.expires_at {
            Some(exp) => Utc::now() < exp,
            None => true, // sin expiración => válido mientras no se revoque
        }
    }

    /// Revoca explícitamente el consentimiento
    pub fn revoke(&mut self) {
        self.consent_given = false;
    }
}
