use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct UserEmailUpdated {
    pub user_id: Uuid,
    pub old_email: String,
    pub new_email: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserEmailUpdated {
    pub fn new(user_id: Uuid, old_email: String, new_email: String) -> Self {
        Self {
            user_id,
            old_email,
            new_email,
            occurred_at: Utc::now(),
        }
    }
}
