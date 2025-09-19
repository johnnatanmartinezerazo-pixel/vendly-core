use chrono::{DateTime, Duration, Utc};

use crate::user::domain::{
    entities::user_subscription::UserSubscription,
    vo::{SubscriptionStatus, SubscriptionTier, ValidationError},
};

/// Servicio de dominio para la gestión de suscripciones de usuario.
pub struct SubscriptionService;

impl SubscriptionService {
    /// Crea una nueva suscripción con estado activo.
    pub fn create_subscription(
        tier: SubscriptionTier,
        duration_days: i64,
    ) -> Result<UserSubscription, ValidationError> {
        let now = Utc::now();
        let expires_at = now + Duration::days(duration_days);

        Ok(UserSubscription {
            subscription_id: uuid::Uuid::new_v4(),
            user_id: uuid::Uuid::nil(), // se asigna en la capa de aplicación
            tier,
            status: SubscriptionStatus::Active,
            starts_at: now,
            expires_at: Some(expires_at),
            auto_renew: true,
            payment_method: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Renueva una suscripción si está activa o expirada con `auto_renew = true`.
    pub fn renew_subscription(
        subscription: &mut UserSubscription,
        duration_days: i64,
    ) -> Result<(), ValidationError> {
        if !subscription.auto_renew {
            return Err(ValidationError::InvalidSubscriptionStatus);
        }

        let now = Utc::now();
        let new_expiration = now + Duration::days(duration_days);

        subscription.status = SubscriptionStatus::Active;
        subscription.starts_at = now;
        subscription.expires_at = Some(new_expiration);
        subscription.updated_at = now;

        Ok(())
    }

    /// Cancela una suscripción.
    pub fn cancel_subscription(subscription: &mut UserSubscription) {
        subscription.status = SubscriptionStatus::Canceled;
        subscription.auto_renew = false;
        subscription.updated_at = Utc::now();
    }

    /// Verifica si una suscripción está activa y vigente.
    pub fn is_active(subscription: &UserSubscription) -> bool {
        match subscription.status {
            SubscriptionStatus::Active => {
                if let Some(expiration) = subscription.expires_at {
                    expiration > Utc::now()
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    /// Cambia el nivel de suscripción (ej: Free → Pro).
    pub fn change_tier(
        subscription: &mut UserSubscription,
        new_tier: SubscriptionTier,
    ) -> Result<(), ValidationError> {
        if subscription.status != SubscriptionStatus::Active {
            return Err(ValidationError::InvalidSubscriptionStatus);
        }

        subscription.tier = new_tier;
        subscription.updated_at = Utc::now();
        Ok(())
    }
}
