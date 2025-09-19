use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Evento de dominio que indica que un usuario verificó su correo electrónico.
#[derive(Debug, Clone)]
pub struct UserEmailVerified {
    pub user_id: Uuid,
    pub email: String,
    pub verified_at: DateTime<Utc>,
}

impl UserEmailVerified {
    pub fn new(user_id: Uuid, email: String, verified_at: DateTime<Utc>) -> Self {
        Self {
            user_id,
            email,
            verified_at,
        }
    }
}
