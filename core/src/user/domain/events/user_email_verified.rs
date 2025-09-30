use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct UserEmailVerified {
    pub user_id: Uuid,
    pub email: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserEmailVerified {
    pub fn new(user_id: Uuid, email: String) -> Self {
        Self {
            user_id,
            email,
            occurred_at: Utc::now(),
        }
    }
}
