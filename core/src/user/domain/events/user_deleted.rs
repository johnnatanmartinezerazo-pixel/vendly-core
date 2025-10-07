use crate::user::domain::vo::{
    UserId,
    UserStatus,
    OccurredAt,
};

pub struct UserDeleted {
    user_id: UserId,
    user_status: UserStatus,
    occurred_at: OccurredAt,
}

impl UserDeleted {
    pub fn new(user_id: UserId, user_status: UserStatus) -> Self {
        Self {
            user_id,
            user_status,
            occurred_at: OccurredAt::now(),
        }
    }
}
