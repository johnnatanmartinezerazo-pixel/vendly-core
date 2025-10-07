use crate::user::domain::vo::{
    UserId,
    Phone,
    OccurredAt,
};

pub struct UserPhoneAssigned {
    user_id: UserId,
    phone: Phone,
    occurred_at: OccurredAt,
}

impl UserPhoneAssigned {
    pub fn new(user_id: UserId, phone: Phone) -> Self {
        Self {
            user_id,
            phone,
            occurred_at: OccurredAt::now(),
        }
    }
}
