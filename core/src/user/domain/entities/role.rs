use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::user::domain::vo::{RoleName, ValidationError};

/// Representa un rol dentro del dominio.
/// Tiene identidad propia (role_id) y atributos relevantes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    pub role_id: Uuid,
    pub name: RoleName,             // VO: no permitimos nombres inválidos
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub permissions: Vec<String>,   // Si necesitas, lo puedes convertir en VO también
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
}

impl Role {
    /// Constructor seguro: valida los campos obligatorios
    pub fn new(
        role_id: Uuid,
        name_raw: &str,
        display_name: Option<String>,
        description: Option<String>,
        permissions: Vec<String>,
        is_system: bool,
        created_at: Option<DateTime<Utc>>,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            role_id,
            name: RoleName::try_from(name_raw)?, // VO se encarga de validar
            display_name,
            description,
            permissions,
            is_system,
            created_at: created_at.unwrap_or_else(Utc::now),
        })
    }

    /// Verifica si un rol tiene cierto permiso
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }

    /// Agrega un permiso (si no existe)
    pub fn add_permission(&mut self, permission: String) {
        if !self.has_permission(&permission) {
            self.permissions.push(permission);
        }
    }

    /// Elimina un permiso
    pub fn remove_permission(&mut self, permission: &str) {
        self.permissions.retain(|p| p != permission);
    }
}
