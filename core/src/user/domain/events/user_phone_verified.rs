use crate::user::domain::vo::{
    UserId,
    Phone,
    OccurredAt,
};

pub struct UserPhoneVerified {
    user_id: UserId,
    phone: Phone,
    occurred_at: OccurredAt,
}

impl UserPhoneVerified {
    pub fn new(user_id: UserId, phone: Phone) -> Self {
        Self {
            user_id,
            phone,
            occurred_at: OccurredAt::now(),
        }
    }
}
