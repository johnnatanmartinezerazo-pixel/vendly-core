use crate::user::domain::vo::{
    UserId,
    Username,
    OccurredAt,
};

pub struct UserUsernameAssigned {
    user_id: UserId,
    username: Username,
    occurred_at: OccurredAt,
}

impl UserUsernameAssigned {
    pub fn new(user_id: UserId, username: Username) -> Self {
        Self {
            user_id,
            username,
            occurred_at: OccurredAt::now(),
        }
    }
}
