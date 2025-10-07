use crate::user::domain::vo::{
    UserId,
    UserStatus,
    OccurredAt,
};

pub struct UserActivated {
    user_id: UserId,
    user_status: UserStatus,
    occurred_at: OccurredAt,
}

impl UserActivated {
    pub fn new(user_id: UserId, user_status: UserStatus) -> Self {
        Self {
            user_id,
            user_status,
            occurred_at: OccurredAt::now(),
        }
    }
}
