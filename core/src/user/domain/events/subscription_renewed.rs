use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Evento de dominio que indica que la suscripción de un usuario fue renovada.
#[derive(Debug, Clone)]
pub struct SubscriptionRenewed {
    pub user_id: Uuid,
    pub subscription_id: Uuid,
    pub renewed_at: DateTime<Utc>,
    pub new_expiration_date: DateTime<Utc>,
}

impl SubscriptionRenewed {
    pub fn new(
        user_id: Uuid,
        subscription_id: Uuid,
        renewed_at: DateTime<Utc>,
        new_expiration_date: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            subscription_id,
            renewed_at,
            new_expiration_date,
        }
    }
}
