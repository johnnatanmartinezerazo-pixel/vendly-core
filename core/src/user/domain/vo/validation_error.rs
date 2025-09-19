use std::fmt;

/// Errores de validación para todos los Value Objects del dominio User.
/// Cada variante representa una regla de dominio incumplida.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    // Identidad y credenciales
    InvalidEmail,
    InvalidPhone,
    InvalidUsernameLength,
    InvalidUsernameFormat,

    // Identificador externo (Google, Microsoft, etc.)
    InvalidExternalIdEmpty,
    InvalidExternalIdLength,
    InvalidExternalIdFormat,

    // Roles y estado
    InvalidRole,
    InvalidUserStatus,

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
        let msg = match self {
            ValidationError::InvalidEmail => "El email proporcionado no es válido o supera los 320 caracteres.",
            ValidationError::InvalidPhone => "El número de teléfono no es válido. Debe comenzar con '+' y contener solo dígitos, máximo 20 caracteres.",
            ValidationError::InvalidUsernameLength => "El nombre de usuario no es válido. Debe tener entre 6 y 30 caracteres.",
            ValidationError::InvalidUsernameFormat => "El nombre de usuario no es válido. Tiene un Formato inválido",
            ValidationError::InvalidExternalIdEmpty => "El identificador externo no puede estar vacío.",
            ValidationError::InvalidExternalIdLength => "El identificador externo excede la longitud máxima permitida (255 caracteres).",
            ValidationError::InvalidExternalIdFormat => "El identificador externo contiene caracteres inválidos o de control.",
            ValidationError::InvalidRole => "El rol proporcionado no es válido.",
            ValidationError::InvalidUserStatus => "El estado de usuario proporcionado no es válido.",
            ValidationError::InvalidLocale => "El locale proporcionado no es válido (ej: es-ES).",
            ValidationError::InvalidTimezone => "La zona horaria proporcionada no es válida.",
            ValidationError::InvalidGender => "El género proporcionado no es válido o no esta soportado.",
            ValidationError::InvalidAuthType => "El tipo de autenticación proporcionado no es válido o no esta soportado.",
            ValidationError::InvalidSubscriptionTier => "El nivel de suscripción proporcionado no es válido o no esta soportado.",
            ValidationError::InvalidSubscriptionStatus => "El estado de suscripción proporcionado no es válido o no esta soportado.",
            ValidationError::InvalidConsentType => "El tipo de consentimiento proporcionado no es válido o no esta soportado.",
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for ValidationError {}
