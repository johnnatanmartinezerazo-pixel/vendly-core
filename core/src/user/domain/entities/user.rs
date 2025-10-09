use crate::user::domain::vo::{
    UserId,
    ExternalId,
    Username,
    Email,
    Phone,
    UserStatus,
    OccurredAt,
};
use crate::user::domain::validations::{
    UserDomainError,
    CategoryError,
    TypeError,
};
use crate::user::domain::events::{
    UserDomainEvent,
    UserRegistered,
    UserEmailUpdated,
    UserEmailVerified,
    UserPhoneAssigned,
    UserPhoneVerified,
    UserActivated,
    UserSuspended,
    UserDeleted,
    UserUsernameAssigned,
    UserExternalIdLinked,
};

pub struct User {
    id: UserId,
    external_id: Option<ExternalId>,
    username: Option<Username>,
    email: Email,
    email_verified: bool,
    phone: Option<Phone>,
    phone_verified: bool,
    status: UserStatus,
    created_at: OccurredAt,
    updated_at: OccurredAt,
    deleted_at: Option<OccurredAt>,
    pending_events: Vec<Box<UserDomainEvent>>,
}

impl User {
    pub fn register(email: Email) -> User {
        let now = OccurredAt::now();
        let mut user: User = Self {
            id: UserId::new(),
            external_id: None,
            username: None,
            email,
            email_verified: false,
            phone: None,
            phone_verified: false,
            status: UserStatus::Pending,
            created_at: now.clone(),
            updated_at: now.clone(),
            deleted_at: None,
            pending_events: Vec::new(),
        };

        let event = UserRegistered::new(user.id.clone(), user.email.clone());

        user.record_event(UserDomainEvent::Registered(event));

        user
    }

    fn record_event(&mut self, event: UserDomainEvent) {
        self.pending_events.push(Box::new(event));
    }

    pub fn take_events(&mut self) -> Vec<Box<UserDomainEvent>> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn phone(&self) -> Option<&Phone> {
        self.phone.as_ref()
    }

    pub fn phone_verified(&self) -> bool {
        self.phone_verified
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    pub fn username(&self) -> Option<&Username> {
        self.username.as_ref()
    }

    pub fn external_id(&self) -> Option<&ExternalId> {
        self.external_id.as_ref()
    }

    pub fn created_at(&self) -> &OccurredAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &OccurredAt {
        &self.updated_at
    }

    pub fn deleted_at(&self) -> Option<&OccurredAt> {
        self.deleted_at.as_ref()
    }

    pub fn link_external_id(&mut self, external_id: ExternalId) -> Result<(), UserDomainError> {
        self.updated_at = OccurredAt::now();

        let event = UserExternalIdLinked::new(self.id.clone(), external_id);
        self.record_event(UserDomainEvent::ExternalIdLinkend(event));

        Ok(())
    }

    pub fn update_email(&mut self, new_email: Email) -> Result<(), UserDomainError> {
        let old_email = self.email.clone();

        if old_email == new_email {
            return Err((CategoryError::Email,TypeError::Unchanged {value: format!("new email '{}' is equal to old email '{}'", new_email, old_email),},).into());
        }

        self.email = new_email;
        self.email_verified = false;
        self.status = UserStatus::Pending;
        self.updated_at = OccurredAt::now();

        let event = UserEmailUpdated::new(self.id.clone(), old_email, self.email.clone());
        self.record_event(UserDomainEvent::EmailUpdated(event));

        Ok(())
    }


    pub fn verify_email(&mut self) -> Result<(), UserDomainError> {
        if self.email_verified {
            return Err((CategoryError::Email, TypeError::AlreadyVerified).into());
        }

        if matches!(self.status, UserStatus::Deleted | UserStatus::Suspended) {
            return Err((CategoryError::Email, TypeError::InvalidStatus { status: self.status().clone() }).into());
        }

        self.email_verified = true;
        self.updated_at = OccurredAt::now();

        let event = UserEmailVerified::new(self.id.clone(), self.email.clone());
        self.record_event(UserDomainEvent::EmailVerified(event));

        Ok(())
    }

    pub fn verify_phone(&mut self) -> Result<(), UserDomainError> {
        if self.phone_verified {
            return Err((CategoryError::Phone, TypeError::AlreadyVerified).into());
        }

        match self.phone.clone() {
            Some(phone) => {
                self.phone_verified = true;
                self.updated_at = OccurredAt::now();

                let event = UserPhoneVerified::new(self.id.clone(), phone);
                self.record_event(UserDomainEvent::PhoneVerified(event));

                Ok(())
            }
            None => Err((CategoryError::Phone, TypeError::Missing).into())
        }
    }

    pub fn activate(&mut self) -> Result<(), UserDomainError> {
        match self.status {
            UserStatus::Pending | UserStatus::Suspended => {
                self.status = UserStatus::Active;
                self.updated_at = OccurredAt::now();

                let event = UserActivated::new(self.id.clone(), self.status.clone());
                self.record_event(UserDomainEvent::Activated(event));

                Ok(())
            }
            _ => Err((CategoryError::Status, TypeError::Transition { from: self.status.clone(), to: UserStatus::Active }).into()),
        }
    }

    pub fn suspend(&mut self) -> Result<(), UserDomainError> {
        match self.status {
            UserStatus::Active => {
                self.status = UserStatus::Suspended;
                self.updated_at = OccurredAt::now();

                let event = UserSuspended::new(self.id.clone(), self.status.clone());
                self.record_event(UserDomainEvent::Suspended(event));

                Ok(())
            }
            _ => Err((CategoryError::Status, TypeError::Transition { from: self.status.clone(), to: UserStatus::Suspended}).into()),
        }
    }

    pub fn delete(&mut self) -> Result<(), UserDomainError> {
        match self.status {
            UserStatus::Active | UserStatus::Pending | UserStatus::Suspended => {
                self.status = UserStatus::Deleted;
                self.deleted_at = Some(OccurredAt::now());
                self.updated_at = OccurredAt::now();

                let event = UserDeleted::new(self.id.clone(), self.status.clone());
                self.record_event(UserDomainEvent::Deleted(event));

                Ok(())
            }
            _ => Err((CategoryError::Status, TypeError::Transition { from: self.status.clone(), to: UserStatus::Deleted}).into()),
        }
    }

    pub fn assign_username(&mut self, username: Username) -> Result<(), UserDomainError> {
        self.updated_at = OccurredAt::now();

        let event = UserUsernameAssigned::new(self.id.clone(), username);
        self.record_event(UserDomainEvent::UsernameAssigned(event));

        Ok(())
    }

    pub fn assign_phone(&mut self, phone: Phone) -> Result<(), UserDomainError> {
        self.phone_verified = false;
        self.updated_at = OccurredAt::now();

        let event = UserPhoneAssigned::new(self.id.clone(), phone);
        self.record_event(UserDomainEvent::PhoneAssigned(event));

        Ok(())
    }
}
