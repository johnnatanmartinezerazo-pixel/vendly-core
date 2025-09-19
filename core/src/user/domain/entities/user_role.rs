use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::ValidationError;

/// Relación entre un usuario y un rol dentro del sistema.
///
/// - Un `UserRole` indica que un usuario tiene asignado un rol específico.
/// - Incluye metadatos como quién lo otorgó, cuándo expira y si sigue activo.
#[derive(Debug, Clone, PartialEq)]
pub struct UserRole {
    pub user_role_id: Uuid,
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub granted_by: Option<Uuid>,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

impl UserRole {
    /// Crea una nueva asignación de rol a usuario.
    pub fn new(
        user_role_id: Uuid,
        user_id: Uuid,
        role_id: Uuid,
        granted_by: Option<Uuid>,
        expires_at: Option<DateTime<Utc>>,
        granted_at: Option<DateTime<Utc>>,
        is_active: Option<bool>,
    ) -> Result<Self, ValidationError> {
        // Validación: expiración no puede estar en el pasado
        if let Some(exp) = expires_at {
            if exp < Utc::now() {
                return Err(ValidationError::InvalidUserStatus); // O define un error más específico
            }
        }

        Ok(Self {
            user_role_id,
            user_id,
            role_id,
            granted_by,
            granted_at: granted_at.unwrap_or_else(Utc::now),
            expires_at,
            is_active: is_active.unwrap_or(true),
        })
    }

    /// Revoca el rol (lo marca como inactivo).
    pub fn revoke(&mut self) {
        self.is_active = false;
    }

    /// Reactiva el rol (si estaba revocado).
    pub fn reactivate(&mut self) {
        self.is_active = true;
    }

    /// Verifica si el rol sigue vigente (no expirado y activo).
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }
        if let Some(exp) = self.expires_at {
            return exp > Utc::now();
        }
        true
    }
}
