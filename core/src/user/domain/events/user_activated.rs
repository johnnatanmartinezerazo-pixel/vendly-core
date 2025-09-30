use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct UserActivated {
    pub user_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

impl UserActivated {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            occurred_at: Utc::now(),
        }
    }
}
