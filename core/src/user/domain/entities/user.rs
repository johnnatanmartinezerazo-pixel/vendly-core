use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::{
    Email,
    ExternalId,
    Phone,
    UserStatus,
    Username,
};
use crate::user::domain::vo::{
    ValidationError,
    PhoneErrorKind,
    UserStatusErrorKind,
};
use crate::user::domain::events::{
    UserEvent,
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
    user_id: Uuid,
    external_id: Option<ExternalId>,
    username: Option<Username>,
    email: Email,
    email_verified: bool,
    phone: Option<Phone>,
    phone_verified: bool,
    user_status: UserStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
    pending_events: Vec<Box<dyn UserEvent>>,
}

impl User {
    pub fn register(email: Email) -> User {
        let now = Utc::now();
        let mut user: User = Self {
            user_id: Uuid::new_v4(),
            external_id: None,
            username: None,
            email,
            email_verified: false,
            phone: None,
            phone_verified: false,
            user_status: UserStatus::Pending,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            pending_events: Vec::new(),
        };

        user.pending_events.push(Box::new(UserRegistered::new(user.user_id, user.email.clone())));
        user
    }

    pub fn take_events(&mut self) -> Vec<Box<dyn UserEvent>> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn id(&self) -> &Uuid {
        &self.user_id
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

    pub fn user_status(&self) -> &UserStatus {
        &self.user_status
    }

    pub fn username(&self) -> Option<&Username> {
        self.username.as_ref()
    }

    pub fn external_id(&self) -> Option<&ExternalId> {
        self.external_id.as_ref()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.deleted_at.as_ref()
    }

    pub fn update_email(&mut self, email: Email) {
        let old_email = self.email.clone();
        self.email = email;
        self.email_verified = false;
        self.user_status = UserStatus::Pending;
        self.updated_at = Utc::now();
        self.pending_events.push(Box::new(UserEmailUpdated::new(
            self.user_id.clone(),
            old_email,
            self.email.clone(),
        )));
    }

    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.updated_at = Utc::now();
        self.pending_events.push(Box::new(UserEmailVerified::new(
            self.user_id.clone(),
            self.email.clone(),
        )));
    }

    pub fn verify_phone(&mut self) -> Result<(), ValidationError> {
        match self.phone {
            Some(_) => {
                self.phone_verified = true;
                self.updated_at = Utc::now();
                self.pending_events.push(Box::new(UserPhoneVerified::new(
                    self.user_id.clone(),
                    self.phone.clone().unwrap(),
                )));
                Ok(())
            }
            None => Err(PhoneErrorKind::Missing.into()),
        }
    }

    pub fn activate(&mut self) -> Result<(), ValidationError> {
        match self.user_status {
            UserStatus::Pending | UserStatus::Suspended => {
                self.user_status = UserStatus::Active;
                self.updated_at = Utc::now();
                self.pending_events.push(Box::new(UserActivated::new(
                    self.user_id.clone(),
                    self.user_status.clone(),
                )));
                Ok(())
            }
            _ => Err(UserStatusErrorKind::Transition {
                from: self.user_status.clone(),
                to: UserStatus::Active,
            }.into()),
        }
    }

    pub fn suspend(&mut self) -> Result<(), ValidationError> {
        match self.user_status {
            UserStatus::Active => {
                self.user_status = UserStatus::Suspended;
                self.updated_at = Utc::now();
                self.pending_events.push(Box::new(UserSuspended::new(
                    self.user_id.clone(),
                    self.user_status.clone(),
                )));
                Ok(())
            }
            _ => Err(UserStatusErrorKind::Transition {
                from: self.user_status.clone(),
                to: UserStatus::Suspended,
            }.into()),
        }
    }

    pub fn delete(&mut self) -> Result<(), ValidationError> {
        match self.user_status {
            UserStatus::Active | UserStatus::Pending | UserStatus::Suspended => {
                self.user_status = UserStatus::Deleted;
                self.deleted_at = Some(Utc::now());
                self.updated_at = Utc::now();
                self.pending_events.push(Box::new(UserDeleted::new(
                  self.user_id.clone(),
                  self.user_status.clone(),
                )));
                Ok(())
            }
            _ => Err(UserStatusErrorKind::Transition {
                from: self.user_status.clone(),
                to: UserStatus::Deleted,
            }.into()),
        }
    }

    pub fn assign_username(&mut self, username: Username) -> Result<(), ValidationError> {
        self.username = Some(username);
        self.updated_at = Utc::now();
        self.pending_events.push(Box::new(UserUsernameAssigned::new(
            self.user_id.clone(),
            self.username.clone().unwrap(),
        )));
        Ok(())
    }

    pub fn assign_phone(&mut self, phone: Phone) -> Result<(), ValidationError> {
        self.phone = Some(phone);
        self.phone_verified = false;
        self.updated_at = Utc::now();
        self.pending_events.push(Box::new(UserPhoneAssigned::new(
            self.user_id.clone(),
            self.phone.clone().unwrap(),
        )));
        Ok(())
    }

    pub fn link_external_id(&mut self, external_id: ExternalId) -> Result<(), ValidationError> {
        self.external_id = Some(external_id);
        self.updated_at = Utc::now();

        self.pending_events.push(Box::new(UserExternalIdLinked::new(
            self.user_id.clone(),
            self.external_id.clone().unwrap(),
        )));

        Ok(())
    }

}
