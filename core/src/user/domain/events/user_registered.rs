use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Evento de dominio que indica que un usuario fue registrado en el sistema.
#[derive(Debug, Clone)]
pub struct UserRegistered {
    pub user_id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl UserRegistered {
    pub fn new(user_id: Uuid, email: String, created_at: DateTime<Utc>) -> Self {
        Self {
            user_id,
            email,
            created_at,
        }
    }
}
