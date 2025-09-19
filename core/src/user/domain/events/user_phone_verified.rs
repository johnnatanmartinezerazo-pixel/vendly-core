use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Evento de dominio que indica que un usuario verificó su número de teléfono.
#[derive(Debug, Clone)]
pub struct UserPhoneVerified {
    pub user_id: Uuid,
    pub phone: String,
    pub verified_at: DateTime<Utc>,
}

impl UserPhoneVerified {
    pub fn new(user_id: Uuid, phone: String, verified_at: DateTime<Utc>) -> Self {
        Self {
            user_id,
            phone,
            verified_at,
        }
    }
}
