use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::user::domain::entities::role::Role;
use crate::user::domain::repositories::role_repository::RoleRepository;
use crate::user::domain::vo::validation_error::ValidationError;

// Importa las entidades generadas por SeaORM (ejemplo: sea-orm-cli generate entity)
use crate::user::infrastructure::persistence::orm::entities::roles;

pub struct RoleRepositorySeaOrm {
    pub db: DatabaseConnection,
}

impl RoleRepositorySeaOrm {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RoleRepository for RoleRepositorySeaOrm {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>, ValidationError> {
        let role = roles::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|_| ValidationError::InvalidRole)?;

        Ok(role.map(|model| Role {
            id: model.role_id,
            name: model.name,
            display_name: model.display_name,
            description: model.description,
            permissions: model.permissions.unwrap_or_default(),
            is_system: model.is_system,
            created_at: model.created_at,
        }))
    }

    async fn save(&self, role: &Role) -> Result<(), ValidationError> {
        let active_model = roles::ActiveModel {
            role_id: Set(role.id),
            name: Set(role.name.clone()),
            display_name: Set(role.display_name.clone()),
            description: Set(role.description.clone()),
            permissions: Set(Some(role.permissions.clone())),
            is_system: Set(role.is_system),
            created_at: Set(role.created_at),
        };

        active_model
            .insert(&self.db)
            .await
            .map_err(|_| ValidationError::InvalidRole)?;

        Ok(())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Role>, ValidationError> {
        let role = roles::Entity::find()
            .filter(roles::Column::Name.eq(name))
            .one(&self.db)
            .await
            .map_err(|_| ValidationError::InvalidRole)?;

        Ok(role.map(|model| Role {
            id: model.role_id,
            name: model.name,
            display_name: model.display_name,
            description: model.description,
            permissions: model.permissions.unwrap_or_default(),
            is_system: model.is_system,
            created_at: model.created_at,
        }))
    }
}
