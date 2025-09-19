use uuid::Uuid;

use crate::user::domain::{
    entities::user::User,
    vo::ValidationError,
};

/// Contrato de repositorio para la entidad/agregado User.
/// Define cómo interactuar con la persistencia sin exponer detalles de la base de datos.
pub trait UserRepository {
    /// Busca un usuario por su UUID.
    fn get_by_id(&self, id: Uuid) -> Result<Option<User>, ValidationError>;

    /// Busca un usuario por su email.
    fn get_by_email(&self, email: &str) -> Result<Option<User>, ValidationError>;

    /// Busca un usuario por su nombre de usuario.
    fn get_by_username(&self, username: &str) -> Result<Option<User>, ValidationError>;

    /// Verifica si un email ya existe (para reglas de unicidad).
    fn exists_by_email(&self, email: &str) -> Result<bool, ValidationError>;

    /// Verifica si un username ya existe.
    fn exists_by_username(&self, username: &str) -> Result<bool, ValidationError>;

    /// Guarda (crea o actualiza) un usuario.
    fn save(&mut self, user: &User) -> Result<(), ValidationError>;

    /// Elimina lógicamente un usuario (soft delete).
    fn soft_delete(&mut self, id: Uuid) -> Result<(), ValidationError>;
}
