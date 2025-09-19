use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::user::domain::vo::{
    Email, Phone, ExternalId, Username, UserStatus
};

/// Entity que representa un Usuario dentro del dominio.
/// Está compuesta por Value Objects que garantizan consistencia.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct User {
    pub user_id: Uuid,                  // Identidad principal
    pub external_id: Option<ExternalId>,// Puede existir si integra sistemas externos
    pub username: Option<Username>,     // Opcional en tu modelo, pero VO para validación
    pub email: Email,                   // VO obligatorio
    pub email_verified: bool,           // Estado de negocio
    pub phone: Option<Phone>,           // VO opcional
    pub phone_verified: bool,           // Estado de negocio
    pub status: UserStatus,             // VO que controla el ciclo de vida
    pub created_at: DateTime<Utc>,      // Trazabilidad
    pub updated_at: DateTime<Utc>,      // Trazabilidad
    pub deleted_at: Option<DateTime<Utc>>, // Soft delete o auditoría
}

#[allow(dead_code)]
impl User {
    /// Crea un nuevo usuario con estado inicial `Pending` y verificaciones en `false`.
    pub fn new(
        user_id: Uuid,
        email: Email,
        external_id: Option<ExternalId>,
        username: Option<Username>,
        phone: Option<Phone>,
    ) -> Self {
        Self {
            user_id,
            external_id,
            username,
            email,
            email_verified: false,
            phone,
            phone_verified: false,
            status: UserStatus::Pending, // valor por defecto
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }

    /// Marca el email como verificado.
    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.updated_at = Utc::now();
    }

    /// Marca el teléfono como verificado.
    pub fn verify_phone(&mut self) {
        if self.phone.is_some() {
            self.phone_verified = true;
            self.updated_at = Utc::now();
        }
    }

    /// Cambia el estado del usuario.
    pub fn activate(&mut self) {
        self.status = UserStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn suspend(&mut self) {
        self.status = UserStatus::Suspended;
        self.updated_at = Utc::now();
    }

    pub fn deleted(&mut self) {
        self.status = UserStatus::Deleted;
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Asigna un username si no existe.
    pub fn set_username(&mut self, username: Username) {
        self.username = Some(username);
    }

    /// Asigna un phone si no existe.
    pub fn set_phone(&mut self, phone: Phone) {
        self.phone = Some(phone);
        self.phone_verified = false; // requiere nueva verificación
    }

    /// Asigna un external_id (ej: Google, Microsoft).
    pub fn set_external_id(&mut self, external_id: ExternalId) {
        self.external_id = Some(external_id);
        self.updated_at = Utc::now();
    }
}
