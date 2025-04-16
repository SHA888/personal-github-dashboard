// Models module for database entities and domain types

mod notification;

pub use notification::{
    Notification, NotificationFrequency, NotificationSettings, NotificationType,
};
