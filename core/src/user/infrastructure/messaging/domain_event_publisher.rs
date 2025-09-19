use async_trait::async_trait;
use std::sync::Arc;
use crate::user::domain::events::*; // importa todos los eventos de user/domain/events

/// Trait genérico para publicar eventos de dominio.
/// Esto permite que el dominio sea independiente de la infraestructura.
#[async_trait]
pub trait DomainEventPublisher: Send + Sync {
    async fn publish(&self, event: DomainEvent) -> anyhow::Result<()>;
}

/// Enum que envuelve los eventos del dominio de `user`.
/// Permite que un único publicador maneje múltiples tipos de eventos.
#[derive(Debug, Clone)]
pub enum DomainEvent {
    UserRegistered(UserRegistered),
    UserEmailVerified(UserEmailVerified),
    UserPhoneVerified(UserPhoneVerified),
    UserLoggedIn(UserLoggedIn),
    SubscriptionRenewed(SubscriptionRenewed),
    // Aquí puedes seguir agregando los eventos que vayas creando
}

/// Implementación de ejemplo en memoria.
/// Útil para tests o ambientes locales.
pub struct InMemoryDomainEventPublisher {
    pub events: Arc<tokio::sync::Mutex<Vec<DomainEvent>>>,
}

impl InMemoryDomainEventPublisher {
    pub fn new() -> Self {
        Self {
            events: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl DomainEventPublisher for InMemoryDomainEventPublisher {
    async fn publish(&self, event: DomainEvent) -> anyhow::Result<()> {
        let mut events = self.events.lock().await;
        events.push(event);
        Ok(())
    }
}
