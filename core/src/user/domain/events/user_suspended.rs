use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::UserEvent;

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

impl UserEvent for UserSuspended {
    fn event_name(&self) -> &'static str {
        "UserSuspended"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
