use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::{ValidationError};

/// Tipos válidos de MFA.
/// Si prefieres, esto puede ser un VO en `vo/mfa_type.rs`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MfaType {
    Totp,     // Google Authenticator, Authy, etc.
    Sms,
    Email,
    WebAuthn,
}

impl MfaType {
    pub fn from_str(value: &str) -> Result<Self, ValidationError> {
        match value.to_lowercase().as_str() {
            "totp" => Ok(MfaType::Totp),
            "sms" => Ok(MfaType::Sms),
            "email" => Ok(MfaType::Email),
            "webauthn" => Ok(MfaType::WebAuthn),
            _ => Err(ValidationError::InvalidAuthType), // o crear un ValidationError específico para MFA
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MfaType::Totp => "totp",
            MfaType::Sms => "sms",
            MfaType::Email => "email",
            MfaType::WebAuthn => "webauthn",
        }
    }
}

/// Representa un método MFA configurado para un usuario.
#[derive(Debug, Clone, PartialEq)]
pub struct UserMfa {
    pub mfa_id: Uuid,
    pub user_id: Uuid,
    pub mfa_type: MfaType,
    pub secret_encrypted: Option<String>,
    pub backup_codes_encrypted: Option<Vec<String>>,
    pub recovery_codes_used: i32,
    pub is_enabled: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

impl UserMfa {
    /// Crea una nueva configuración MFA validada
    pub fn new(
        mfa_id: Uuid,
        user_id: Uuid,
        mfa_type: MfaType,
        secret_encrypted: Option<String>,
        backup_codes_encrypted: Option<Vec<String>>,
        recovery_codes_used: Option<i32>,
        is_enabled: bool,
        is_verified: bool,
        created_at: Option<DateTime<Utc>>,
        last_used_at: Option<DateTime<Utc>>,
    ) -> Result<Self, ValidationError> {
        // Ejemplo de regla de negocio:
        // TOTP requiere secret_encrypted, SMS/Email no necesariamente
        if matches!(mfa_type, MfaType::Totp) && secret_encrypted.is_none() {
            return Err(ValidationError::InvalidAuthType);
        }

        Ok(Self {
            mfa_id,
            user_id,
            mfa_type,
            secret_encrypted,
            backup_codes_encrypted,
            recovery_codes_used: recovery_codes_used.unwrap_or(0),
            is_enabled,
            is_verified,
            created_at: created_at.unwrap_or_else(Utc::now),
            last_used_at,
        })
    }

    /// Marca MFA como verificado
    pub fn verify(&mut self) {
        self.is_verified = true;
    }

    /// Deshabilita MFA
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    /// Actualiza el uso de recovery codes
    pub fn use_recovery_code(&mut self) {
        self.recovery_codes_used += 1;
    }
}
