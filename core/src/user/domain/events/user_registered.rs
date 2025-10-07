use crate::user::domain::vo::{
    UserId,
    Email,
    OccurredAt,
};

pub struct UserRegistered {
    user_id: UserId,
    email: Email,
    occurred_at: OccurredAt
}

impl UserRegistered {
    pub fn new(user_id: UserId, email: Email) -> Self {
        Self {
          user_id,
          email,
          occurred_at: OccurredAt::now(),
        }
    }
}
