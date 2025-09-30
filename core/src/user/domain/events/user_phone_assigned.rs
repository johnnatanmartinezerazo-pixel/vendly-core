use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct UserPhoneAssigned {
    pub user_id: Uuid,
    pub phone: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserPhoneAssigned {
    pub fn new(user_id: Uuid, phone: String) -> Self {
        Self {
            user_id,
            phone,
            occurred_at: Utc::now(),
        }
    }
}
