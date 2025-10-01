use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::Email;
use super::UserEvent;

pub struct UserEmailUpdated {
    user_id: Uuid,
    old_email: Email,
    new_email: Email,
    occurred_at: DateTime<Utc>,
}

impl UserEmailUpdated {
    pub fn new(user_id: Uuid, old_email: Email, new_email: Email) -> Self {
        Self {
            user_id,
            old_email,
            new_email,
            occurred_at: Utc::now(),
        }
    }
}

impl UserEvent for UserEmailUpdated {
    fn event_name(&self) -> &'static str {
        "UserEmailUpdated"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
