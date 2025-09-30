use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserSuspended {
    pub user_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

impl UserSuspended {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            occurred_at: Utc::now(),
        }
    }
}
