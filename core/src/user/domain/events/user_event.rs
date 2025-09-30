use chrono::{DateTime, Utc};

pub trait UserEvent: Send + Sync {
    fn event_name(&self) -> &'static str;
    fn occurred_at(&self) -> DateTime<Utc>;
}
