use std::fmt;

use super::UserStatus;

/// Errores de validación para todos los Value Objects del dominio User.
/// Cada variante representa una regla de dominio incumplida.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    // Email
    InvalidEmailFormat,
    InvalidEmailTooLong,

    // Phone
    InvalidPhone,
    MissingPhone,

    // Username
    InvalidUsername,

    // Identificador externo (Google, Microsoft, etc.)
    InvalidExternalId,

    // Estado
    InvalidStatusTransition {
        from: UserStatus,
        to: UserStatus,
    },
    InvalidUserStatus,

    // Roles
    InvalidRole,

    // Perfil de usuario
    InvalidLocale,
    InvalidTimezone,
    InvalidGender,

    // Autenticación
    InvalidAuthType,

    // Suscripciones
    InvalidSubscriptionTier,
    InvalidSubscriptionStatus,

    // GDPR / Consentimientos
    InvalidConsentType,

}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidEmailFormat => {
                write!(f, "El formato del email proporcionado no es válido.")
            }
            ValidationError::InvalidEmailTooLong => {
                write!(f, "El email proporcionado excede la longitud máxima permitida (254 caracteres).")
            }
            ValidationError::InvalidPhone => {
                write!(f, "El número de teléfono no es válido. Debe comenzar con '+' y contener solo dígitos, con una longitud mínima de 7 y máxima de 15 dígitos.")
            }
            ValidationError::MissingPhone => {
                write!(f, "Se requiere un número de teléfono.")
            }
            ValidationError::InvalidUsername => {
                write!(f, "El nombre de usuario no es válido. Debe tener entre 6 y 30 caracteres, comenzar con una letra y solo puede contener letras, números, puntos (.), guiones (-) o guiones bajos (_). No puede terminar en símbolo ni contener símbolos consecutivos.")
            }
            ValidationError::InvalidExternalId => {
                write!(f, "El identificador externo excede la longitud máxima permitida (255 caracteres).")
            }
            ValidationError::InvalidStatusTransition { from, to } => {
                write!(f, "La transición de estado no es válida. No se puede cambiar de {:?} a {:?}", from, to)
            }
            ValidationError::InvalidUserStatus => {
                write!(f, "El estado del usuario proporcionado no es válido.")
            }
            ValidationError::InvalidRole => {
                write!(f, "El rol proporcionado no es válido o no está soportado.")
            }
            ValidationError::InvalidLocale => {
                write!(f, "El locale proporcionado no es válido (ej: es-ES).")
            }
            ValidationError::InvalidTimezone => {
                write!(f, "La zona horaria proporcionada no es válida.")
            }
            ValidationError::InvalidGender => {
                write!(f, "El género proporcionado no es válido o no esta soportado.")
            }
            ValidationError::InvalidAuthType => {
                write!(f, "El tipo de autenticación proporcionado no es válido o no esta soportado.")
            }
            ValidationError::InvalidSubscriptionTier => {
                write!(f, "El nivel de suscripción proporcionado no es válido o no esta soportado.")
            }
            ValidationError::InvalidSubscriptionStatus => {
                write!(f, "El estado de suscripción proporcionado no es válido o no esta soportado.")
            }
            ValidationError::InvalidConsentType => {
                write!(f, "El tipo de consentimiento proporcionado no es válido o no esta soportado.")
            }
        }
    }
}


impl std::error::Error for ValidationError {}
