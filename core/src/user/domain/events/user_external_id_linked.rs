use crate::user::domain::vo::{
    UserId,
    ExternalId,
    OccurredAt,
};

pub struct UserExternalIdLinked {
    user_id: UserId,
    external_id: ExternalId,
    occurred_at: OccurredAt,
}

impl UserExternalIdLinked {
    pub fn new(user_id: UserId, external_id: ExternalId) -> Self {
        Self {
            user_id,
            external_id,
            occurred_at: OccurredAt::now(),
        }
    }
}
