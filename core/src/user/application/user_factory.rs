use uuid::Uuid;
use std::convert::TryFrom;
use crate::user::{self, domain::{
    Email, ExternalId, Phone, User, UserStatus, ValidationError
}};

pub struct UserFactory;

impl UserFactory {
    pub fn create(
    id: Uuid,
    email_raw: &str,
    username_raw: Option<&str>,
    phone_raw: Option<&str>,
    external_id_raw: Option<&str>,
    status_raw: Option<&str>,
    ) -> Result<User, ValidationError> {
        let email = Email::try_from(email_raw)?;

        let username = match username_raw {
            Some(v) => Some(user::domain::Username::try_from(v)?),
            None => None,
        };

        let phone = match phone_raw {
            Some(v) => Some(Phone::try_from(v)?),
            None => None,
        };

        let external_id = match external_id_raw {
            Some(v) => Some(ExternalId::try_from(v)?),
            None => None,
        };

        let status = match status_raw {
            Some(v) => Some(UserStatus::try_from(v)?),
            None => None,
        };

        Ok(User::new(id, email, username, phone, status.unwrap_or(UserStatus::Pending), external_id))
    }
}
