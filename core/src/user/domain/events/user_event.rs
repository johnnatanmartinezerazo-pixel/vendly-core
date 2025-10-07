use crate::user::domain::events::{
    UserActivated,
    UserDeleted,
    UserEmailUpdated,
    UserEmailVerified,
    UserExternalIdLinked,
    UserPhoneAssigned,
    UserPhoneVerified,
    UserRegistered,
    UserSuspended,
    UserUsernameAssigned,
};

use crate::user::domain::vo::OccurredAt;

pub enum UserDomainEvent {
    Activated(UserActivated),
    Deleted(UserDeleted),
    EmailUpdated(UserEmailUpdated),
    EmailVerified(UserEmailVerified),
    ExternalIdLinkend(UserExternalIdLinked),
    PhoneAssigned(UserPhoneAssigned),
    PhoneVerified(UserPhoneVerified),
    Registered(UserRegistered),
    Suspended(UserSuspended),
    UsernameAssigned(UserUsernameAssigned)
}

impl UserDomainEvent {
    pub fn event_name(&self) -> &'static str {
        match self {
            Self::Activated(_) => "user_activated",
            Self::Deleted(_) => "user_deleted",
            Self::EmailUpdated(_) => "user_email_updated",
            Self::EmailVerified(_) => "user_email_verified",
            Self::ExternalIdLinkend(_) => "user_external_id_linkend",
            Self::PhoneAssigned(_) => "user_phone_assigned",
            Self::PhoneVerified(_) => "user_phone_verified",
            Self::Registered(_) => "user_registered",
            Self::Suspended(_) => "user_suspended",
            Self::UsernameAssigned(_) => "username_assigned",
        }
    }

    pub fn occurred_at(&self) -> OccurredAt {
        match self {
            Self::Activated(_) => OccurredAt::now(),
            Self::Deleted(_) => OccurredAt::now(),
            Self::EmailUpdated(_) => OccurredAt::now(),
            Self::EmailVerified(_) => OccurredAt::now(),
            Self::ExternalIdLinkend(_) => OccurredAt::now(),
            Self::PhoneAssigned(_) => OccurredAt::now(),
            Self::PhoneVerified(_) => OccurredAt::now(),
            Self::Registered(_) => OccurredAt::now(),
            Self::Suspended(_) => OccurredAt::now(),
            Self::UsernameAssigned(_) => OccurredAt::now(),
        }
    }
}
