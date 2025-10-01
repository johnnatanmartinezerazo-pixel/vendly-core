use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::Phone;
use super::UserEvent;

pub struct UserPhoneVerified {
    user_id: Uuid,
    phone: Phone,
    occurred_at: DateTime<Utc>,
}

impl UserPhoneVerified {
    pub fn new(user_id: Uuid, phone: Phone) -> Self {
        Self {
            user_id,
            phone,
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
