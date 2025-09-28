use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

use crate::user::domain::vo::{
    Gender,
    Locale,
    Timezone,
    ValidationError,
};

/// Perfil del usuario: datos adicionales y de presentación.
///
/// - Esta entity está relacionada 1:1 con `User`
/// - Maneja datos opcionales, ya que no todos los usuarios tienen perfil completo.
#[derive(Debug, Clone, PartialEq)]
pub struct UserProfile {
    pub profile_id: Uuid,
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub locale: Locale,
    pub timezone: Timezone,
    pub created_at: DateTime<Utc>,
}

impl UserProfile {
    /// Crea un nuevo perfil de usuario validado.
    pub fn new(
        profile_id: Uuid,
        user_id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        display_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        birth_date: Option<NaiveDate>,
        gender: Option<Gender>,
        locale: Option<Locale>,
        timezone: Option<Timezone>,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Self, ValidationError> {
        // Validación mínima de display_name
        if let Some(name) = &display_name {
            if name.trim().is_empty() {
                return Err(ValidationError::InvalidUsername); // puedes hacer un `InvalidDisplayName`
            }
            if name.len() < 6 || name.len() > 30 {
                return Err(ValidationError::InvalidUsername);
            }
        }

        Ok(Self {
            profile_id,
            user_id,
            first_name,
            last_name,
            display_name,
            avatar_url,
            bio,
            birth_date,
            gender,
            locale: locale.unwrap_or(Locale::default()),
            timezone: timezone.unwrap_or(Timezone::default()),
            created_at: created_at.unwrap_or_else(Utc::now),
        })
    }

    /// Actualiza el nombre para mostrar.
    pub fn update_display_name(&mut self, name: String) -> Result<(), ValidationError> {
        if name.trim().is_empty(){
            return Err(ValidationError::InvalidUsername);
        } else if name.len() < 6 || name.len() > 30 {
            return Err(ValidationError::InvalidUsername);
        }
        self.display_name = Some(name);
        Ok(())
    }

    /// Actualiza el avatar del perfil.
    pub fn update_avatar(&mut self, url: String) {
        self.avatar_url = Some(url);
    }

    /// Actualiza la biografía.
    pub fn update_bio(&mut self, bio: String) {
        self.bio = Some(bio);
    }

    /// Actualiza la localización y zona horaria.
    pub fn update_locale_timezone(&mut self, locale: Locale, timezone: Timezone) {
        self.locale = locale;
        self.timezone = timezone;
    }
}
