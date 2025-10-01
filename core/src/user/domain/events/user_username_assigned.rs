use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::Username;
use super::UserEvent;

pub struct UserUsernameAssigned {
    pub user_id: Uuid,
    pub username: Username,
    pub occurred_at: DateTime<Utc>,
}

impl UserUsernameAssigned {
    pub fn new(user_id: Uuid, username: Username) -> Self {
        Self {
            user_id,
            username,
            occurred_at: Utc::now(),
        }
    }
}

impl UserEvent for UserUsernameAssigned {
    fn event_name(&self) -> &'static str {
        "UserUsernameAssigned"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
