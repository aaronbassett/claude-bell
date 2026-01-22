//! macOS notification sending and response handling

pub mod macos;
pub mod response;

pub use macos::{send_notification, NotificationConfig};
pub use response::{NotificationResponse, ResponseHandler};
