use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::UserEvent;

pub struct UserDeleted {
    pub user_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

impl UserDeleted {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            occurred_at: Utc::now(),
        }
    }
}

impl UserEvent for UserDeleted {
    fn event_name(&self) -> &'static str {
        "UserDeleted"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
