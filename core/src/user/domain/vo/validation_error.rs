use std::fmt;

use super::UserStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailErrorKind {
    Empty,
    Format,
    TooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhoneErrorKind {
    Empty,
    Missing,
    Format,
    TooShort,
    TooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsernameErrorKind {
    Empty,
    TooShort,
    TooLong,
    Characters,
    StartsWithChar,
    EndsWithChar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalIdErrorKind {
    Empty,
    TooLong,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserStatusErrorKind {
    Empty,
    Value,
    Transition {
        from: UserStatus,
        to: UserStatus,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoleErrorKind {
    Empty,
    Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocaleErrorKind {
    Empty,
    Format,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimezoneErrorKind {
    Empty,
    Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenderErrorKind {
    Empty,
    Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthTypeErrorKind {
    Empty,
    Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionTierErrorKind {
    Empty,
    Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionStatusErrorKind {
    Empty,
    Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsentTypeErrorKind {
    Empty,
    Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    EmailError {
        reason: EmailErrorKind,
    },
    PhoneError {
        reason: PhoneErrorKind,
    },
    UsernameError {
        reason: UsernameErrorKind,
    },
    ExternalIdError {
        reason: ExternalIdErrorKind,
    },
    UserStatusError {
        reason: UserStatusErrorKind,
    },
    RoleError {
        reason: RoleErrorKind,
    },
    LocaleError {
        reason: LocaleErrorKind,
    },
    TimezoneError {
        reason: TimezoneErrorKind,
    },
    GenderError {
        reason: GenderErrorKind,
    },
    AuthTypeError {
        reason: AuthTypeErrorKind,
    },
    SubscriptionTierError {
        reason: SubscriptionTierErrorKind,
    },
    SubscriptionStatusError {
        reason: SubscriptionStatusErrorKind,
    },
    ConsentTypeError {
        reason: ConsentTypeErrorKind,
    },
}

impl From<EmailErrorKind> for ValidationError {
    fn from(value: EmailErrorKind) -> Self {
        ValidationError::EmailError { reason: value }
    }
}

impl From<PhoneErrorKind> for ValidationError {
    fn from(value: PhoneErrorKind) -> Self {
        ValidationError::PhoneError { reason: value }
    }
}

impl From<UsernameErrorKind> for ValidationError {
    fn from(value: UsernameErrorKind) -> Self {
        ValidationError::UsernameError { reason: value }
    }
}

impl From<ExternalIdErrorKind> for ValidationError {
    fn from(value: ExternalIdErrorKind) -> Self {
        ValidationError::ExternalIdError { reason: value }
    }
}

impl From<UserStatusErrorKind> for ValidationError {
    fn from(value: UserStatusErrorKind) -> Self {
        ValidationError::UserStatusError { reason: value }
    }
}

impl From<RoleErrorKind> for ValidationError {
    fn from(value: RoleErrorKind) -> Self {
        ValidationError::RoleError { reason: value }
    }
}

impl From<LocaleErrorKind> for ValidationError {
    fn from(value: LocaleErrorKind) -> Self {
        ValidationError::LocaleError { reason: value }
    }
}

impl From<TimezoneErrorKind> for ValidationError {
    fn from(value: TimezoneErrorKind) -> Self {
        ValidationError::TimezoneError { reason: value }
    }
}

impl From<GenderErrorKind> for ValidationError {
    fn from(value: GenderErrorKind) -> Self {
        ValidationError::GenderError { reason: value }
    }
}

impl From<AuthTypeErrorKind> for ValidationError {
    fn from(value: AuthTypeErrorKind) -> Self {
        ValidationError::AuthTypeError { reason: value }
    }
}

impl From<SubscriptionTierErrorKind> for ValidationError {
    fn from(value: SubscriptionTierErrorKind) -> Self {
        ValidationError::SubscriptionTierError { reason: value }
    }
}

impl From<SubscriptionStatusErrorKind> for ValidationError {
    fn from(value: SubscriptionStatusErrorKind) -> Self {
        ValidationError::SubscriptionStatusError { reason: value }
    }
}

impl From<ConsentTypeErrorKind> for ValidationError {
    fn from(value: ConsentTypeErrorKind) -> Self {
        ValidationError::ConsentTypeError { reason: value }
    }
}

impl fmt::Display for EmailErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailErrorKind::Empty => write!(f, "El email es obligatorio."),
            EmailErrorKind::Format => write!(f, "El formato no es válido."),
            EmailErrorKind::TooLong => write!(f, "Excede la longitud máxima permitida (254 caracteres)."),
        }
    }
}

impl fmt::Display for PhoneErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhoneErrorKind::Empty => write!(f, "El número de teléfono es obligatorio."),
            PhoneErrorKind::Missing => write!(f, "Se requiere un número de teléfono."),
            PhoneErrorKind::Format => write!(f, "El número de teléfono no es válido."),
            PhoneErrorKind::TooShort => write!(f, "El número de teléfono es demasiado corto."),
            PhoneErrorKind::TooLong => write!(f, "El número de teléfono es demasiado largo."),
        }
    }
}

impl fmt::Display for UsernameErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsernameErrorKind::Empty => write!(f, "El username es obligatorio."),
            UsernameErrorKind::TooShort => write!(f, "El username es demasiado corto."),
            UsernameErrorKind::TooLong => write!(f, "El username es demasiado largo."),
            UsernameErrorKind::Characters => write!(f, "El username contiene caracteres no permitidos."),
            UsernameErrorKind::StartsWithChar => write!(f, "El username no puede comenzar con un carácter no permitido."),
            UsernameErrorKind::EndsWithChar => write!(f, "El username no puede terminar con un carácter no permitido."),
        }
    }
}

impl fmt::Display for ExternalIdErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExternalIdErrorKind::Empty => write!(f, "El identificador externo es obligatorio."),
            ExternalIdErrorKind::TooLong => write!(f, "El identificador externo es demasiado largo."),
        }
    }
}

impl fmt::Display for UserStatusErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserStatusErrorKind::Empty => write!(f, "El estado del usuario es obligatorio."),
            UserStatusErrorKind::Value => write!(f, "El estado del usuario proporcionado no es válido."),
            UserStatusErrorKind::Transition { from, to } => write!(f, "Transición de estado inválida de {:?} a {:?}.", from, to),
        }
    }
}

impl fmt::Display for RoleErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoleErrorKind::Empty => write!(f, "El rol es obligatorio."),
            RoleErrorKind::Value => write!(f, "El rol proporcionado no es válido o no está soportado."),
        }
    }
}

impl fmt::Display for LocaleErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocaleErrorKind::Empty => write!(f, "El locale es obligatorio."),
            LocaleErrorKind::Format => write!(f, "El locale proporcionado no es válido (ej: es-ES)."),
        }
    }
}

impl fmt::Display for TimezoneErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimezoneErrorKind::Empty => write!(f, "La zona horaria es obligatoria."),
            TimezoneErrorKind::Value => write!(f, "La zona horaria proporcionada no es válida."),
        }
    }
}

impl fmt::Display for GenderErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenderErrorKind::Empty => write!(f, "El género es obligatorio."),
            GenderErrorKind::Value => write!(f, "El género proporcionado no es válido o no esta soportado."),
        }
    }
}

impl fmt::Display for AuthTypeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthTypeErrorKind::Empty => write!(f, "El tipo de autenticación es obligatorio."),
            AuthTypeErrorKind::Value => write!(f, "El tipo de autenticación proporcionado no es válido o no esta soportado."),
        }
    }
}

impl fmt::Display for SubscriptionTierErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubscriptionTierErrorKind::Empty => write!(f, "El nivel de suscripción es obligatorio."),
            SubscriptionTierErrorKind::Value => write!(f, "El nivel de suscripción proporcionado no es válido o no esta soportado."),
        }
    }
}

impl fmt::Display for SubscriptionStatusErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubscriptionStatusErrorKind::Empty => write!(f, "El estado de suscripción es obligatorio."),
            SubscriptionStatusErrorKind::Value => write!(f, "El estado de suscripción proporcionado no es válido o no esta soportado."),
        }
    }
}

impl fmt::Display for ConsentTypeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsentTypeErrorKind::Empty => write!(f, "El tipo de consentimiento es obligatorio."),
            ConsentTypeErrorKind::Value => write!(f, "El tipo de consentimiento proporcionado no es válido o no esta soportado."),
        }
    }
}


impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmailError { reason } => {
                write!(f, "Error en el email: {}", reason)
            }
            ValidationError::PhoneError { reason } => {
                write!(f, "Error en el teléfono: {}", reason)
            }
            ValidationError::UsernameError { reason } => {
                write!(f, "Error en el username: {}", reason)
            }
            ValidationError::ExternalIdError { reason } => {
                write!(f, "Error en el identificador externo: {}", reason)
            }
            ValidationError::UserStatusError { reason } => {
                write!(f, "El estado del usuario proporcionado no es válido: {}", reason)
            }
            ValidationError::RoleError { reason } => {
                write!(f, "El rol proporcionado no es válido o no está soportado: {}", reason)
            }
            ValidationError::LocaleError { reason } => {
                write!(f, "El locale proporcionado no es válido (ej: es-ES): {}", reason)
            }
            ValidationError::TimezoneError { reason } => {
                write!(f, "La zona horaria proporcionada no es válida: {}", reason)
            }
            ValidationError::GenderError { reason } => {
                write!(f, "El género proporcionado no es válido o no esta soportado: {}", reason)
            }
            ValidationError::AuthTypeError { reason } => {
                write!(f, "El tipo de autenticación proporcionado no es válido o no esta soportado: {}", reason)
            }
            ValidationError::SubscriptionTierError { reason } => {
                write!(f, "El nivel de suscripción proporcionado no es válido o no esta soportado: {}", reason)
            }
            ValidationError::SubscriptionStatusError { reason } => {
                write!(f, "El estado de suscripción proporcionado no es válido o no esta soportado: {}", reason)
            }
            ValidationError::ConsentTypeError { reason } => {
                write!(f, "El tipo de consentimiento proporcionado no es válido o no esta soportado: {}", reason)
            }
        }
    }
}

impl std::error::Error for ValidationError {}
