use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::UserEvent;

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

impl UserEvent for UserEmailVerified {
    fn event_name(&self) -> &'static str {
        "UserEmailVerified"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
