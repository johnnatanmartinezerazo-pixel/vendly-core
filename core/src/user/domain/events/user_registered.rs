use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::UserEvent;

pub struct UserRegistered {
    pub user_id: Uuid,
    pub email: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserRegistered {
    pub fn new(user_id: Uuid, email: String) -> Self {
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
