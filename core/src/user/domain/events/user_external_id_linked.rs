use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::ExternalId;
use super::UserEvent;

pub struct UserExternalIdLinked {
    user_id: Uuid,
    external_id: ExternalId,
    occurred_at: DateTime<Utc>,
}

impl UserExternalIdLinked {
    pub fn new(user_id: Uuid, external_id: ExternalId) -> Self {
        Self {
            user_id,
            external_id,
            occurred_at: Utc::now(),
        }
    }
}

impl UserEvent for UserExternalIdLinked {
    fn event_name(&self) -> &'static str {
        "UserExternalIdLinked"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }
}
