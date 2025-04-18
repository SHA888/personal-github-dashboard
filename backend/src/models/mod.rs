// Models module for database entities and domain types

pub mod notification;

pub use notification::{
    Notification, NotificationFrequency, NotificationSettings, NotificationType,
};
