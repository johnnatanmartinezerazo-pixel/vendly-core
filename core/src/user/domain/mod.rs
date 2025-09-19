// Declaración de submódulos internos
pub mod aggregates;
pub mod entities;
pub mod events;
pub mod repositories;
pub mod vo;

// Reexportamos lo que debe ser consumido por Application Layer
pub use aggregates::*;
pub use entities::*;
pub use events::*;
pub use repositories::*;
pub use vo::*;
