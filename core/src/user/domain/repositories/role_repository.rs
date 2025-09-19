use uuid::Uuid;

use crate::user::domain::{
    entities::role::Role,
    vo::ValidationError,
};

/// Contrato de repositorio para Roles.
/// Encapsula el acceso a la persistencia sin exponer detalles de la DB.
pub trait RoleRepository {
    /// Obtiene un rol por su ID.
    fn get_by_id(&self, id: Uuid) -> Result<Option<Role>, ValidationError>;

    /// Obtiene un rol por su nombre (ej: "admin").
    fn get_by_name(&self, name: &str) -> Result<Option<Role>, ValidationError>;

    /// Lista todos los roles.
    fn list_all(&self) -> Result<Vec<Role>, ValidationError>;

    /// Guarda (crea/actualiza) un rol.
    fn save(&mut self, role: &Role) -> Result<(), ValidationError>;

    /// Elimina un rol por su ID.
    fn delete(&mut self, id: Uuid) -> Result<(), ValidationError>;
}
