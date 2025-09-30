use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::user::domain::vo::ExternalId;

pub struct UserExternalIdLinked {
    pub user_id: Uuid,
    pub external_id: ExternalId,
    pub occurred_at: DateTime<Utc>,
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
