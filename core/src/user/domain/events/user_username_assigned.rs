use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::user::domain::vo::Username;

pub struct UserUsernameAssigned {
    pub user_id: Uuid,
    pub username: Username,
    pub occurred_at: DateTime<Utc>,
}

impl UserUsernameAssigned {
    pub fn new(user_id: Uuid, username: Username) -> Self {
        Self {
            user_id,
            username,
            occurred_at: Utc::now(),
        }
    }
}
