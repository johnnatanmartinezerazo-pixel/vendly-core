use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::UserStatus;
use super::UserEvent;

pub struct UserDeleted {
    user_id: Uuid,
    user_status: UserStatus,
    occurred_at: DateTime<Utc>,
}

impl UserDeleted {
    pub fn new(user_id: Uuid, user_status: UserStatus) -> Self {
        Self {
            user_id,
            user_status,
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
