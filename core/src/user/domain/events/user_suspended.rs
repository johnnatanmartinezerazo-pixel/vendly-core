use crate::user::domain::vo::{
    UserId,
    UserStatus,
    OccurredAt,
};

pub struct UserSuspended {
    user_id: UserId,
    user_status: UserStatus,
    occurred_at: OccurredAt,
}

impl UserSuspended {
    pub fn new(user_id: UserId, user_status: UserStatus) -> Self {
        Self {
            user_id,
            user_status,
            occurred_at: OccurredAt::now(),
        }
    }
}
