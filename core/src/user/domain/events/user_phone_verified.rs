use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::UserEvent;

pub struct UserPhoneVerified {
    pub user_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

impl UserPhoneVerified {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            occurred_at: Utc::now(),
        }
    }
}

impl UserEvent for UserPhoneVerified {
    fn event_name(&self) -> &'static str {
        "UserPhoneVerified"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
