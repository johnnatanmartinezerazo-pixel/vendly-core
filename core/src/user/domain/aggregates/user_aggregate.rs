use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::entities::{
    user::User,
    user_profile::UserProfile,
    user_auth_method::UserAuthMethod,
    user_password::UserPassword,
    user_mfa::UserMfa,
    user_session::UserSession,
    user_role::UserRole,
    user_subscription::UserSubscription,
    user_gdpr_consent::UserGdprConsent,
    user_activity_log::UserActivityLog,
};

/// Aggregate root que representa un Usuario completo con sus datos relacionados.
/// Todas las operaciones deben realizarse a través de este objeto para mantener
/// la consistencia en el dominio.
#[derive(Debug)]
pub struct UserAggregate {
    pub user: User,
    pub profile: Option<UserProfile>,
    pub auth_methods: Vec<UserAuthMethod>,
    pub password: Option<UserPassword>,
    pub mfa: Vec<UserMfa>,
    pub sessions: Vec<UserSession>,
    pub roles: Vec<UserRole>,
    pub subscriptions: Vec<UserSubscription>,
    pub gdpr_consents: Vec<UserGdprConsent>,
    pub activity_logs: Vec<UserActivityLog>,
}

impl UserAggregate {
    /// Crear un nuevo agregado con la entidad raíz (`User`) y colecciones vacías.
    pub fn new(user: User) -> Self {
        Self {
            user,
            profile: None,
            auth_methods: Vec::new(),
            password: None,
            mfa: Vec::new(),
            sessions: Vec::new(),
            roles: Vec::new(),
            subscriptions: Vec::new(),
            gdpr_consents: Vec::new(),
            activity_logs: Vec::new(),
        }
    }

    /// Devuelve el ID único del usuario.
    pub fn id(&self) -> Uuid {
        self.user.user_id
    }

    // --------------------------
    // Métodos de negocio
    // --------------------------

    /// Agregar un método de autenticación.
    pub fn add_auth_method(&mut self, method: UserAuthMethod) {
        self.auth_methods.push(method);
    }

    /// Asignar un perfil al usuario.
    pub fn set_profile(&mut self, profile: UserProfile) {
        self.profile = Some(profile);
    }

    /// Registrar un rol.
    pub fn add_role(&mut self, role: UserRole) {
        self.roles.push(role);
    }

    /// Agregar un consentimiento GDPR.
    pub fn add_gdpr_consent(&mut self, consent: UserGdprConsent) {
        self.gdpr_consents.push(consent);
    }

    /// Registrar actividad de usuario.
    pub fn log_activity(&mut self, log: UserActivityLog) {
        self.activity_logs.push(log);
    }
}
