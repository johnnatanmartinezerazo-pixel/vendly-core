use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::net::IpAddr;

/// Evento de dominio que indica que un usuario inició sesión exitosamente.
#[derive(Debug, Clone)]
pub struct UserLoggedIn {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub logged_in_at: DateTime<Utc>,
}

impl UserLoggedIn {
    pub fn new(
        user_id: Uuid,
        session_id: Uuid,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        logged_in_at: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            session_id,
            ip_address,
            user_agent,
            logged_in_at,
        }
    }
}
