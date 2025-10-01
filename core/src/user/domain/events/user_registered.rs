use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::Email;
use super::UserEvent;

pub struct UserRegistered {
    user_id: Uuid,
    email: Email,
    occurred_at: DateTime<Utc>,
}

impl UserRegistered {
    pub fn new(user_id: Uuid, email: Email) -> Self {
        Self {
          user_id,
          email,
          occurred_at: Utc::now(),
        }
    }
}

impl UserEvent for UserRegistered {
    fn event_name(&self) -> &'static str {
        "UserRegistered"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
