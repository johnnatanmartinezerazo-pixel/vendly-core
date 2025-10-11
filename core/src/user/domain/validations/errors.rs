use std::fmt;

use crate::user::domain::vo::UserStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CategoryError {
    Id,
    ExternalId,
    Username,
    Email,
    Phone,
    Status,
    Role,
    Locale,
    Timezone,
    Gender,
    AuthType,
    SubscriptionTier,
    SubscriptionStatus,
    ConsentType,
    OccurredAt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    Empty,
    Missing,
    NotSupported,
    Format { format: String,},
    TooShort { short: u16,},
    TooLong { long: u32,},
    Unchanged { value: String,},
    Characters { value: String,},
    StartsWithChar { start: String,},
    EndsWithChar { end: String,},
    AlreadyVerified,
    InvalidStatus { status: UserStatus,},
    Transition { from: UserStatus, to: UserStatus, },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct  UserDomainError {
    category: CategoryError,
    detail: TypeError,
}

impl From<(CategoryError, TypeError)> for UserDomainError {
    fn from((category, detail): (CategoryError, TypeError)) -> Self {
        Self { category, detail }
    }
}

impl fmt::Display for UserDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {:?} -> {:?}", self.category, self.detail)
    }
}

impl std::error::Error for UserDomainError {}
