use uuid::Uuid;
use std::convert::TryFrom;

use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl TryFrom<&str> for UserId {
    type Error = UserDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err((CategoryError::Id, TypeError::Empty).into());
        }
        match Uuid::parse_str(value) {
            Ok(uuid) => Ok(UserId(uuid)),
            Err(_) => Err((CategoryError::Id, TypeError::Format { format: "uuid".into() }).into()),
        }
    }
}
