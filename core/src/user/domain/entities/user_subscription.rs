use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::user::domain::vo::{SubscriptionTier, SubscriptionStatus, ValidationError};

/// Representa una suscripción de usuario en el sistema.
///
/// - Cada usuario puede tener una o varias suscripciones a distintos planes.
/// - Controla nivel (`tier`), estado (`status`), renovación y fechas.
/// - Está alineada con la tabla `user_subscriptions`.
#[derive(Debug, Clone, PartialEq)]
pub struct UserSubscription {
    pub subscription_id: Uuid,
    pub user_id: Uuid,
    pub tier: SubscriptionTier,
    pub status: SubscriptionStatus,
    pub starts_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub auto_renew: bool,
    pub payment_method: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserSubscription {
    /// Crea una nueva suscripción validando fechas y estado.
    pub fn new(
        subscription_id: Uuid,
        user_id: Uuid,
        tier: SubscriptionTier,
        status: SubscriptionStatus,
        starts_at: DateTime<Utc>,
        expires_at: Option<DateTime<Utc>>,
        auto_renew: bool,
        payment_method: Option<String>,
    ) -> Result<Self, ValidationError> {
        // Validar que expires_at sea posterior a starts_at si existe
        if let Some(exp) = expires_at {
            if exp <= starts_at {
                return Err(ValidationError::InvalidSubscriptionStatus);
            }
        }

        Ok(Self {
            subscription_id,
            user_id,
            tier,
            status,
            starts_at,
            expires_at,
            auto_renew,
            payment_method,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Verifica si la suscripción está actualmente activa.
    pub fn is_active(&self) -> bool {
        if self.status != SubscriptionStatus::Active {
            return false;
        }
        if let Some(exp) = self.expires_at {
            return exp > Utc::now();
        }
        true
    }

    /// Marca la suscripción como cancelada.
    pub fn cancel(&mut self) {
        self.status = SubscriptionStatus::Canceled;
        self.auto_renew = false;
        self.updated_at = Utc::now();
    }

    /// Renueva la suscripción extendiendo la fecha de expiración.
    pub fn renew(&mut self, new_expires_at: DateTime<Utc>) -> Result<(), ValidationError> {
        if new_expires_at <= Utc::now() {
            return Err(ValidationError::InvalidSubscriptionStatus);
        }
        self.expires_at = Some(new_expires_at);
        self.status = SubscriptionStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Pausa temporalmente la suscripción (ejemplo: problemas de pago).
    pub fn inactive(&mut self) {
        self.status = SubscriptionStatus::Inactive;
        self.updated_at = Utc::now();
    }
}
