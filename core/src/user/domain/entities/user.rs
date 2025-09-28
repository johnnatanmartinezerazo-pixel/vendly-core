use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::user::domain::vo::{Email, Phone, Username, ExternalId, UserStatus, ValidationError};

pub struct User {
    user_id: Uuid,
    external_id: Option<ExternalId>,
    username: Option<Username>,
    email: Email,
    email_verified: bool,
    phone: Option<Phone>,
    phone_verified: bool,
    status: UserStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn register(email: Email) -> Self {
        let now = Utc::now();
        Self {
            user_id: Uuid::new_v4(),
            external_id: None,
            username: None,
            email,
            email_verified: false,
            phone: None,
            phone_verified: false,
            status: UserStatus::Pending,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
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

    pub fn status(&self) -> &UserStatus {
        &self.status
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

    /// Actualiza el email del usuario y marca como no verificado.
    pub fn update_email(&mut self, email: Email) {
        self.email = email;
        self.email_verified = false; // requiere nueva verificación
        self.status = UserStatus::Pending;
        self.updated_at = Utc::now();
    }

    /// Marca el email como verificado.
    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.updated_at = Utc::now();
    }

    /// Marca el teléfono como verificado.
    pub fn verify_phone(&mut self) -> Result<(), ValidationError> {
        match self.phone {
            Some(_) => {
                self.phone_verified = true;
                self.updated_at = Utc::now();
                Ok(())
            }
            None => Err(ValidationError::MissingPhone),
        }
    }


    /// Cambia el estado del usuario.
    pub fn activate(&mut self) -> Result<(), ValidationError> {
        match self.status {
            UserStatus::Pending | UserStatus::Suspended => {
                self.status = UserStatus::Active;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(ValidationError::InvalidStatusTransition {
                from: self.status.clone(),
                to: UserStatus::Active,
            }),
        }
    }

    pub fn suspend(&mut self) -> Result<(), ValidationError> {
        match self.status {
            UserStatus::Active => {
                self.status = UserStatus::Suspended;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(ValidationError::InvalidStatusTransition {
                from: self.status.clone(),
                to: UserStatus::Suspended,
            }),
        }
    }

    pub fn delete(&mut self) -> Result<(), ValidationError> {
        match self.status {
            UserStatus::Active | UserStatus::Pending | UserStatus::Suspended => {
                self.status = UserStatus::Deleted;
                self.deleted_at = Some(Utc::now());
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(ValidationError::InvalidStatusTransition {
                from: self.status.clone(),
                to: UserStatus::Deleted,
            }),
        }
    }

    /// Asigna un username
    pub fn assign_username(&mut self, username: Username) {
        self.username = Some(username);
        self.updated_at = Utc::now();
    }

    /// Asigna un phone
    pub fn assign_phone(&mut self, phone: Phone) {
        self.phone = Some(phone);
        self.phone_verified = false; // requiere nueva verificación
        self.updated_at = Utc::now();
    }

    /// Asigna un external_id
    pub fn link_external_id(&mut self, external_id: ExternalId) {
        self.external_id = Some(external_id);
        self.updated_at = Utc::now();
    }

}
