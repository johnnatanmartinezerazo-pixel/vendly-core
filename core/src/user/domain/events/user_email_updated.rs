use crate::user::domain::vo::{
    UserId,
    Email,
    OccurredAt,
};

pub struct UserEmailUpdated {
    user_id: UserId,
    old_email: Email,
    new_email: Email,
    occurred_at: OccurredAt,
}

impl UserEmailUpdated {
    pub fn new(user_id: UserId, old_email: Email, new_email: Email) -> Self {
        Self {
            user_id,
            old_email,
            new_email,
            occurred_at: OccurredAt::now(),
        }
    }
}
