use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::UserEvent;

pub struct UserPhoneAssigned {
    pub user_id: Uuid,
    pub phone: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserPhoneAssigned {
    pub fn new(user_id: Uuid, phone: String) -> Self {
        Self {
            user_id,
            phone,
            occurred_at: Utc::now(),
        }
    }
}

impl UserEvent for UserPhoneAssigned {
    fn event_name(&self) -> &'static str {
        "UserPhoneAssigned"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
