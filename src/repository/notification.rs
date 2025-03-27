use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::model::notification::Notification;

lazy_static! {
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(Vec::new());
}

pub struct NotificationRepository;

impl NotificationRepository {
    pub fn add(notification: Notification) -> Notification {
        NOTIFICATIONS.write().unwrap().push(notification.clone());
        notification
    }

    pub fn list_all_as_string() -> Vec<String> {
        NOTIFICATIONS.read().unwrap().iter().map(|f| {
            format!("{}", f.clone())
        }).collect()
    }
}
