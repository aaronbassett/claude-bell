//! macOS notification sending using UserNotifications framework
//!
//! This module provides manual bindings to the UserNotifications framework
//! using objc2 for type-safe Objective-C interop.

use crate::cli::args::Cli;
use crate::error::{AppError, ExitCode};
use crate::notification::response::{NotificationResponse, ResponseHandler};
use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::{AnyClass, AnyObject};
use objc2::{class, msg_send, msg_send_id};
use objc2_foundation::{NSArray, NSDictionary, NSError, NSString, NSURL};
use std::path::Path;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Manual declaration of UNUserNotificationCenter
#[repr(C)]
pub struct UNUserNotificationCenter {
    _priv: [u8; 0],
}

unsafe impl objc2::RefEncode for UNUserNotificationCenter {
    const ENCODING_REF: objc2::Encoding = objc2::Encoding::Object;
}

unsafe impl objc2::Message for UNUserNotificationCenter {}

impl UNUserNotificationCenter {
    fn class() -> &'static AnyClass {
        class!(UNUserNotificationCenter)
    }

    /// Get the shared notification center instance
    pub fn current() -> Option<Retained<Self>> {
        unsafe {
            msg_send_id![
                Self::class(),
                currentNotificationCenter
            ]
        }
    }

    /// Request notification permissions
    ///
    /// This makes a proper system call to request permissions.
    /// The completion handler is optional for simplified usage.
    pub fn request_authorization(&self, options: u64) -> Result<bool, AppError> {
        unsafe {
            // Call requestAuthorizationWithOptions with NULL completion handler
            // The system will still request permissions, we just won't wait for the callback
            let _: () = msg_send![
                self,
                requestAuthorizationWithOptions: options
                completionHandler: ptr::null::<AnyObject>()
            ];
        }

        // Return true since we've made the request
        // In production, you'd wait for the callback, but for our use case this is sufficient
        Ok(true)
    }

    /// Add a notification request (simplified version)
    ///
    /// Note: This is a simplified implementation that doesn't wait for completion.
    /// A full implementation would use blocks to handle the async callback.
    pub fn add_request(&self, request: &UNNotificationRequest) -> Result<(), AppError> {
        unsafe {
            // Call the method without a completion handler for now
            // The notification will be posted, we just won't know immediately if it failed
            let _: () = msg_send![
                self,
                addNotificationRequest: request
                withCompletionHandler: ptr::null::<AnyObject>()
            ];

            Ok(())
        }
    }

    /// Set the notification center delegate
    pub fn set_delegate(&self, delegate: &AnyObject) {
        unsafe {
            let _: () = msg_send![self, setDelegate: delegate];
        }
    }
}

/// Manual declaration of UNMutableNotificationContent
#[repr(C)]
pub struct UNMutableNotificationContent {
    _priv: [u8; 0],
}

unsafe impl objc2::RefEncode for UNMutableNotificationContent {
    const ENCODING_REF: objc2::Encoding = objc2::Encoding::Object;
}

unsafe impl objc2::Message for UNMutableNotificationContent {}

impl UNMutableNotificationContent {
    fn class() -> &'static AnyClass {
        class!(UNMutableNotificationContent)
    }

    /// Create a new mutable notification content
    pub fn new() -> Retained<Self> {
        unsafe { msg_send_id![Self::class(), new] }
    }

    /// Set the notification title
    pub fn set_title(&self, title: &str) {
        unsafe {
            let title = NSString::from_str(title);
            let _: () = msg_send![self, setTitle: &*title];
        }
    }

    /// Set the notification subtitle
    pub fn set_subtitle(&self, subtitle: &str) {
        unsafe {
            let subtitle = NSString::from_str(subtitle);
            let _: () = msg_send![self, setSubtitle: &*subtitle];
        }
    }

    /// Set the notification body
    pub fn set_body(&self, body: &str) {
        unsafe {
            let body = NSString::from_str(body);
            let _: () = msg_send![self, setBody: &*body];
        }
    }

    /// Set the notification sound
    pub fn set_sound(&self, sound_name: &str) {
        unsafe {
            let sound_name_ns = NSString::from_str(sound_name);
            let sound: Option<Retained<AnyObject>> = msg_send_id![
                class!(UNNotificationSound),
                soundNamed: &*sound_name_ns
            ];
            if let Some(sound) = sound {
                let _: () = msg_send![self, setSound: &*sound];
            }
        }
    }

    /// Set default sound
    pub fn set_default_sound(&self) {
        unsafe {
            let sound: Retained<AnyObject> = msg_send_id![class!(UNNotificationSound), defaultSound];
            let _: () = msg_send![self, setSound: &*sound];
        }
    }

    /// Set user info dictionary
    pub fn set_user_info(&self, user_info: &NSDictionary<NSString, AnyObject>) {
        unsafe {
            let _: () = msg_send![self, setUserInfo: user_info];
        }
    }

    /// Set attachments array
    pub fn set_attachments(&self, attachments: &NSArray<AnyObject>) {
        unsafe {
            let _: () = msg_send![self, setAttachments: attachments];
        }
    }

    /// Set category identifier for actions
    pub fn set_category_identifier(&self, identifier: &str) {
        unsafe {
            let identifier = NSString::from_str(identifier);
            let _: () = msg_send![self, setCategoryIdentifier: &*identifier];
        }
    }

    /// Set notification icon
    /// Note: UserNotifications doesn't support custom icons directly.
    /// Icons are typically handled through app bundle or notification service extensions.
    /// This method attempts to set an icon attachment if the path exists.
    pub fn set_icon(&self, icon_path: &str) -> Result<(), AppError> {
        // Try to create an attachment for the icon
        let path_obj = Path::new(icon_path);
        if !path_obj.exists() {
            return Ok(()); // Silently ignore non-existent icons
        }

        unsafe {
            let url = NSURL::fileURLWithPath(&NSString::from_str(icon_path));

            // Try to create an icon attachment
            // Note: macOS may display this as a thumbnail rather than an app icon
            let identifier = NSString::from_str("icon");
            let mut error_ptr: *mut NSError = ptr::null_mut();
            let attachment: Option<Retained<UNNotificationAttachment>> = msg_send_id![
                class!(UNNotificationAttachment),
                attachmentWithIdentifier: &*identifier
                URL: &*url
                options: ptr::null::<NSDictionary<NSString, AnyObject>>()
                error: &mut error_ptr
            ];

            if let Some(attachment) = attachment {
                // Get existing attachments or create new array
                let existing_attachments: Option<Retained<NSArray<AnyObject>>> = msg_send_id![self, attachments];
                let mut attachments_vec: Vec<Retained<AnyObject>> = Vec::new();

                if let Some(existing) = existing_attachments {
                    for i in 0..existing.len() {
                        if let Some(obj) = existing.get(i) {
                            // Create a new retained reference
                            let retained: Retained<AnyObject> = Retained::retain(obj as *const AnyObject as *mut AnyObject).unwrap();
                            attachments_vec.push(retained);
                        }
                    }
                }

                // Add icon attachment
                attachments_vec.push(Retained::cast(attachment));
                let attachments_array = NSArray::from_vec(attachments_vec);
                self.set_attachments(&attachments_array);
            }
        }

        Ok(())
    }
}

/// Manual declaration of UNNotificationRequest
#[repr(C)]
pub struct UNNotificationRequest {
    _priv: [u8; 0],
}

unsafe impl objc2::RefEncode for UNNotificationRequest {
    const ENCODING_REF: objc2::Encoding = objc2::Encoding::Object;
}

unsafe impl objc2::Message for UNNotificationRequest {}

impl UNNotificationRequest {
    fn class() -> &'static AnyClass {
        class!(UNNotificationRequest)
    }

    /// Create a notification request with identifier, content, and trigger
    pub fn with_identifier(
        identifier: &str,
        content: &UNMutableNotificationContent,
        trigger: Option<&AnyObject>,
    ) -> Retained<Self> {
        unsafe {
            let identifier = NSString::from_str(identifier);
            let trigger_ptr = trigger.map_or(ptr::null(), |t| t as *const AnyObject);
            msg_send_id![
                Self::class(),
                requestWithIdentifier: &*identifier
                content: content
                trigger: trigger_ptr
            ]
        }
    }
}

/// Manual declaration of UNNotificationAction
#[repr(C)]
pub struct UNNotificationAction {
    _priv: [u8; 0],
}

unsafe impl objc2::RefEncode for UNNotificationAction {
    const ENCODING_REF: objc2::Encoding = objc2::Encoding::Object;
}

unsafe impl objc2::Message for UNNotificationAction {}

impl UNNotificationAction {
    fn class() -> &'static AnyClass {
        class!(UNNotificationAction)
    }

    /// Create an action with identifier and title
    pub fn with_identifier(identifier: &str, title: &str, options: u64) -> Retained<Self> {
        unsafe {
            let identifier = NSString::from_str(identifier);
            let title = NSString::from_str(title);
            msg_send_id![
                Self::class(),
                actionWithIdentifier: &*identifier
                title: &*title
                options: options
            ]
        }
    }
}

/// Manual declaration of UNTextInputNotificationAction
#[repr(C)]
pub struct UNTextInputNotificationAction {
    _priv: [u8; 0],
}

unsafe impl objc2::RefEncode for UNTextInputNotificationAction {
    const ENCODING_REF: objc2::Encoding = objc2::Encoding::Object;
}

unsafe impl objc2::Message for UNTextInputNotificationAction {}

impl UNTextInputNotificationAction {
    fn class() -> &'static AnyClass {
        class!(UNTextInputNotificationAction)
    }

    /// Create a text input action
    pub fn with_identifier(
        identifier: &str,
        title: &str,
        options: u64,
        button_title: &str,
        placeholder: &str,
    ) -> Retained<Self> {
        unsafe {
            let identifier = NSString::from_str(identifier);
            let title = NSString::from_str(title);
            let button_title = NSString::from_str(button_title);
            let placeholder = NSString::from_str(placeholder);
            msg_send_id![
                Self::class(),
                actionWithIdentifier: &*identifier
                title: &*title
                options: options
                textInputButtonTitle: &*button_title
                textInputPlaceholder: &*placeholder
            ]
        }
    }
}

/// Manual declaration of UNNotificationCategory
#[repr(C)]
pub struct UNNotificationCategory {
    _priv: [u8; 0],
}

unsafe impl objc2::RefEncode for UNNotificationCategory {
    const ENCODING_REF: objc2::Encoding = objc2::Encoding::Object;
}

unsafe impl objc2::Message for UNNotificationCategory {}

impl UNNotificationCategory {
    fn class() -> &'static AnyClass {
        class!(UNNotificationCategory)
    }

    /// Create a category with identifier and actions
    pub fn with_identifier(
        identifier: &str,
        actions: &NSArray<AnyObject>,
        intent_identifiers: &NSArray<NSString>,
        options: u64,
    ) -> Retained<Self> {
        unsafe {
            let identifier = NSString::from_str(identifier);
            msg_send_id![
                Self::class(),
                categoryWithIdentifier: &*identifier
                actions: actions
                intentIdentifiers: intent_identifiers
                options: options
            ]
        }
    }
}

/// Manual declaration of UNNotificationAttachment
#[repr(C)]
pub struct UNNotificationAttachment {
    _priv: [u8; 0],
}

unsafe impl objc2::RefEncode for UNNotificationAttachment {
    const ENCODING_REF: objc2::Encoding = objc2::Encoding::Object;
}

unsafe impl objc2::Message for UNNotificationAttachment {}

impl UNNotificationAttachment {
    fn class() -> &'static AnyClass {
        class!(UNNotificationAttachment)
    }

    /// Create an attachment from a URL
    pub fn with_identifier(identifier: &str, url: &NSURL) -> Result<Retained<Self>, AppError> {
        unsafe {
            let identifier = NSString::from_str(identifier);
            let mut error_ptr: *mut NSError = ptr::null_mut();
            let attachment: Option<Retained<Self>> = msg_send_id![
                Self::class(),
                attachmentWithIdentifier: &*identifier
                URL: url
                options: ptr::null::<NSDictionary<NSString, AnyObject>>()
                error: &mut error_ptr
            ];

            if let Some(attachment) = attachment {
                Ok(attachment)
            } else {
                Err(AppError::NotificationError(
                    "Failed to create attachment".to_string(),
                ))
            }
        }
    }
}

// Authorization option constants
const UN_AUTHORIZATION_OPTION_BADGE: u64 = 1 << 0;
const UN_AUTHORIZATION_OPTION_SOUND: u64 = 1 << 1;
const UN_AUTHORIZATION_OPTION_ALERT: u64 = 1 << 2;

// Action option constants
const UN_NOTIFICATION_ACTION_OPTION_FOREGROUND: u64 = 1 << 0;

// Category option constants
const UN_NOTIFICATION_CATEGORY_OPTION_CUSTOM_DISMISS_ACTION: u64 = 1 << 0;

/// Shared state for notification responses
struct NotificationState {
    response: Option<NotificationResponse>,
    completed: bool,
}

/// Global state for notification response handling
static NOTIFICATION_STATE: Mutex<Option<Arc<Mutex<NotificationState>>>> = Mutex::new(None);

/// Initialize notification state
fn init_notification_state() -> Arc<Mutex<NotificationState>> {
    let state = Arc::new(Mutex::new(NotificationState {
        response: None,
        completed: false,
    }));

    let mut global = NOTIFICATION_STATE.lock().unwrap();
    *global = Some(state.clone());
    state
}

/// Get current notification state
#[allow(dead_code)]
fn get_notification_state() -> Option<Arc<Mutex<NotificationState>>> {
    let global = NOTIFICATION_STATE.lock().unwrap();
    global.clone()
}

/// Clear notification state
fn clear_notification_state() {
    let mut global = NOTIFICATION_STATE.lock().unwrap();
    *global = None;
}

/// Notification configuration built from CLI arguments
pub struct NotificationConfig {
    pub title: String,
    pub subtitle: Option<String>,
    pub message: Option<String>,
    pub image: Option<String>,
    pub icon: Option<String>,
    pub sound: Option<String>,
    pub actions: Vec<String>,
    pub reply: Option<String>,
    pub url: Option<String>,
    pub persistent: bool,
    pub timeout: Option<Duration>,
    pub default_value: Option<String>,
    pub on_dismiss: Option<String>,
    pub on_timeout: Option<String>,
}

impl NotificationConfig {
    /// Build configuration from CLI arguments
    pub fn from_cli(cli: &Cli) -> Result<Self, AppError> {
        // Title is required for notifications
        let title = cli
            .title
            .clone()
            .ok_or_else(|| AppError::InvalidArgument("Title is required".to_string()))?;

        // Parse timeout if provided
        let timeout = if let Some(ref timeout_str) = cli.timeout {
            Some(parse_duration(timeout_str)?)
        } else {
            None
        };

        // Determine persistence
        let mut persistent = cli.persistent;
        if cli.not_persistent {
            persistent = false;
        } else if cli.actions.is_some() || cli.reply.is_some() {
            // Implicit persistence for interactive notifications
            persistent = true;
        }

        Ok(Self {
            title,
            subtitle: cli.subtitle.clone(),
            message: cli.message.clone(),
            image: cli.image.clone(),
            icon: cli.icon.clone(),
            sound: cli.sound.clone(),
            actions: cli.actions.clone().unwrap_or_default(),
            reply: cli.reply.clone(),
            url: cli.url.clone(),
            persistent,
            timeout,
            default_value: cli.default.clone(),
            on_dismiss: cli.on_dismiss.clone(),
            on_timeout: cli.on_timeout.clone(),
        })
    }

    /// Check if this is an interactive notification
    pub fn is_interactive(&self) -> bool {
        !self.actions.is_empty() || self.reply.is_some()
    }
}

/// Parse duration string (e.g., "30s", "5m", "1h")
fn parse_duration(s: &str) -> Result<Duration, AppError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(AppError::InvalidArgument("Empty duration".to_string()));
    }

    let (number_part, unit_part) = s.split_at(s.len() - 1);
    let value: u64 = number_part
        .parse()
        .map_err(|_| AppError::InvalidArgument(format!("Invalid duration: {}", s)))?;

    let seconds = match unit_part {
        "s" => value,
        "m" => value * 60,
        "h" => value * 3600,
        _ => {
            return Err(AppError::InvalidArgument(format!(
                "Invalid duration unit: {}. Use s, m, or h",
                unit_part
            )))
        }
    };

    Ok(Duration::from_secs(seconds))
}

/// Send a notification with the given configuration
pub fn send_notification(config: NotificationConfig) -> Result<ExitCode, AppError> {
    autoreleasepool(|_| {
        let center = UNUserNotificationCenter::current().ok_or_else(|| {
            AppError::SystemError(
                "Cannot access notification center. This may occur if the app is not properly bundled. \
                 For development, try building with proper macOS app bundle structure."
                    .to_string(),
            )
        })?;

        // Request authorization
        let auth_options =
            UN_AUTHORIZATION_OPTION_ALERT | UN_AUTHORIZATION_OPTION_SOUND | UN_AUTHORIZATION_OPTION_BADGE;
        let _authorized = center.request_authorization(auth_options)?;

        // Create notification content
        let content = UNMutableNotificationContent::new();
        content.set_title(&config.title);

        if let Some(ref subtitle) = config.subtitle {
            content.set_subtitle(subtitle);
        }

        if let Some(ref message) = config.message {
            content.set_body(message);
        }

        // Set sound
        if let Some(ref sound) = config.sound {
            if sound == "default" || sound.is_empty() {
                content.set_default_sound();
            } else {
                content.set_sound(sound);
            }
        }

        // Handle icon (if provided)
        if let Some(ref icon_path) = config.icon {
            // Set icon - this will add it as an attachment if possible
            content.set_icon(icon_path)?;
        }

        // Handle attachments (image)
        if let Some(ref image_path) = config.image {
            if let Some(attachment) = create_attachment(image_path)? {
                unsafe {
                    // Cast the attachment to AnyObject for the NSArray
                    let attachment_obj: Retained<AnyObject> = Retained::cast(attachment);

                    // Get existing attachments (may include icon) and append image
                    let existing_attachments: Option<Retained<NSArray<AnyObject>>> = msg_send_id![&*content, attachments];
                    let mut attachments_vec: Vec<Retained<AnyObject>> = Vec::new();

                    if let Some(existing) = existing_attachments {
                        for i in 0..existing.len() {
                            if let Some(obj) = existing.get(i) {
                                // Create a new retained reference
                                let retained: Retained<AnyObject> = Retained::retain(obj as *const AnyObject as *mut AnyObject).unwrap();
                                attachments_vec.push(retained);
                            }
                        }
                    }

                    attachments_vec.push(attachment_obj);
                    let attachments = NSArray::from_vec(attachments_vec);
                    content.set_attachments(&attachments);
                }
            }
        }

        // Create response handler
        let response_handler = ResponseHandler::new(
            config.default_value.clone(),
            config.on_dismiss.clone(),
            config.on_timeout.clone(),
            config.actions.clone(),
        );

        // Handle actions and reply
        if config.is_interactive() {
            setup_interactive_notification(&content, &config)?;

            // Initialize state for response tracking
            let state = init_notification_state();

            // Set up delegate to receive responses
            // Note: In a full implementation, we'd create a proper Objective-C delegate class
            // For now, we'll use a simplified approach with polling

            // Create and add notification request
            let identifier = NSString::from_str("claude-bell-notification");
            let request = UNNotificationRequest::with_identifier(
                identifier.to_string().as_str(),
                &content,
                None,
            );

            center.add_request(&request)?;

            // Wait for response with timeout
            let timeout = config.timeout.unwrap_or(Duration::from_secs(300)); // Default 5 minutes
            let result = wait_for_response(state, timeout, &config)?;

            clear_notification_state();

            // Print output to stdout
            let (output, exit_code) = response_handler.process_response(&result);
            if !output.is_empty() {
                println!("{}", output);
            }

            return Ok(exit_code);
        }

        // For fire-and-forget notifications, just add request and return
        let identifier = NSString::from_str("claude-bell-notification");
        let request = UNNotificationRequest::with_identifier(
            identifier.to_string().as_str(),
            &content,
            None,
        );

        center.add_request(&request)?;

        // For non-interactive notifications, print default value if specified
        if let Some(ref default_val) = config.default_value {
            println!("{}", default_val);
        }

        Ok(ExitCode::Success)
    })
}

/// Create an attachment from a file path or URL
fn create_attachment(
    path: &str,
) -> Result<Option<Retained<UNNotificationAttachment>>, AppError> {
    // Check if it's a URL
    if path.starts_with("http://") || path.starts_with("https://") {
        // For URLs, we'd need to download first - skip for now
        return Ok(None);
    }

    // Check if file exists
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        return Ok(None);
    }

    unsafe {
        let url = NSURL::fileURLWithPath(&NSString::from_str(path));
        let attachment = UNNotificationAttachment::with_identifier("image", &url)?;
        Ok(Some(attachment))
    }
}

/// Wait for user response to interactive notification
///
/// This function polls for a response from the notification system.
/// In a full implementation, this would use a proper delegate with callbacks.
/// For Phase 10.1, we use a simplified polling approach with checking notification center.
fn wait_for_response(
    state: Arc<Mutex<NotificationState>>,
    timeout: Duration,
    _config: &NotificationConfig,
) -> Result<NotificationResponse, AppError> {
    let start = Instant::now();
    let poll_interval = Duration::from_millis(100);

    // Note: This is a simplified implementation.
    // A full implementation would use UNUserNotificationCenterDelegate callbacks.
    // For now, we'll wait for the timeout and check if the notification was dismissed.

    loop {
        // Check if we've timed out
        if start.elapsed() >= timeout {
            return Ok(NotificationResponse::Timeout);
        }

        // Check state for response
        {
            let state_lock = state.lock().unwrap();
            if state_lock.completed {
                if let Some(ref response) = state_lock.response {
                    return Ok(response.clone());
                }
            }
        }

        // In a real implementation, we'd process events from the notification center delegate
        // For now, we'll use a simplified approach: check with the notification center
        // to see if the notification is still delivered

        // Sleep before next poll
        std::thread::sleep(poll_interval);

        // Check if notification was dismissed by checking delivered notifications
        if let Some(center) = UNUserNotificationCenter::current() {
            unsafe {
                // Get delivered notifications
                let delivered: Option<Retained<NSArray<AnyObject>>> = msg_send_id![
                    &*center,
                    deliveredNotifications
                ];

                if let Some(notifications) = delivered {
                    // If our notification is not in the delivered list, it was dismissed
                    let mut found = false;
                    for i in 0..notifications.len() {
                        if let Some(notification) = notifications.get(i) {
                            let request: Option<Retained<AnyObject>> = msg_send_id![
                                notification,
                                request
                            ];
                            if let Some(req) = request {
                                let identifier: Option<Retained<NSString>> = msg_send_id![
                                    &*req,
                                    identifier
                                ];
                                if let Some(id) = identifier {
                                    if id.to_string() == "claude-bell-notification" {
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    // If notification was removed from delivered list, it was dismissed
                    // Wait a bit longer to see if we get an action callback
                    if !found && start.elapsed() > Duration::from_secs(1) {
                        return Ok(NotificationResponse::Dismissed);
                    }
                }
            }
        }
    }
}

/// Setup interactive notification with actions and/or reply
fn setup_interactive_notification(
    content: &UNMutableNotificationContent,
    config: &NotificationConfig,
) -> Result<(), AppError> {
    let center = UNUserNotificationCenter::current()
        .ok_or_else(|| AppError::SystemError("Cannot access notification center".to_string()))?;

    unsafe {
        let mut actions: Vec<Retained<AnyObject>> = Vec::new();

        // Add reply action if specified
        if let Some(ref reply_placeholder) = config.reply {
            let reply_action = UNTextInputNotificationAction::with_identifier(
                "reply",
                "Reply",
                UN_NOTIFICATION_ACTION_OPTION_FOREGROUND,
                "Send",
                reply_placeholder,
            );
            actions.push(Retained::cast(reply_action));
        }

        // Add button actions
        for (idx, action_title) in config.actions.iter().enumerate() {
            let identifier = format!("action_{}", idx);
            let action = UNNotificationAction::with_identifier(
                &identifier,
                action_title,
                UN_NOTIFICATION_ACTION_OPTION_FOREGROUND,
            );
            actions.push(Retained::cast(action));
        }

        if !actions.is_empty() {
            let actions_array = NSArray::from_vec(actions);
            let intent_identifiers = NSArray::new();

            let category = UNNotificationCategory::with_identifier(
                "CLAUDE_BELL_CATEGORY",
                &actions_array,
                &intent_identifiers,
                UN_NOTIFICATION_CATEGORY_OPTION_CUSTOM_DISMISS_ACTION,
            );

            let category_obj: Retained<AnyObject> = Retained::cast(category);
            let categories_set = NSArray::from_vec(vec![category_obj]);
            let _: () = msg_send![&*center, setNotificationCategories: &*categories_set];

            content.set_category_identifier("CLAUDE_BELL_CATEGORY");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("30s").unwrap(), Duration::from_secs(30));
        assert_eq!(parse_duration("5m").unwrap(), Duration::from_secs(300));
        assert_eq!(parse_duration("2h").unwrap(), Duration::from_secs(7200));

        assert!(parse_duration("").is_err());
        assert!(parse_duration("30x").is_err());
        assert!(parse_duration("abc").is_err());
    }

    #[test]
    fn test_notification_config_interactive() {
        let mut cli = Cli {
            command: None,
            title: Some("Test".to_string()),
            subtitle: None,
            message: None,
            image: None,
            icon: None,
            sound: None,
            actions: Some(vec!["OK".to_string()]),
            reply: None,
            url: None,
            persistent: false,
            not_persistent: false,
            timeout: None,
            default: None,
            on_dismiss: None,
            on_timeout: None,
            batch: false,
            json: None,
            pretty: false,
            quiet: false,
            silent: false,
            log_level: "warn".to_string(),
            template: None,
            var: None,
        };

        let config = NotificationConfig::from_cli(&cli).unwrap();
        assert!(config.is_interactive());
        assert!(config.persistent); // Should be implicit for interactive

        cli.actions = None;
        cli.reply = Some("Type here".to_string());
        let config = NotificationConfig::from_cli(&cli).unwrap();
        assert!(config.is_interactive());

        cli.reply = None;
        let config = NotificationConfig::from_cli(&cli).unwrap();
        assert!(!config.is_interactive());
    }

    #[test]
    fn test_notification_config_from_cli() {
        let cli = Cli {
            command: None,
            title: Some("Test Title".to_string()),
            subtitle: Some("Test Subtitle".to_string()),
            message: Some("Test Message".to_string()),
            image: None,
            icon: None,
            sound: Some("default".to_string()),
            actions: None,
            reply: None,
            url: None,
            persistent: true,
            not_persistent: false,
            timeout: Some("30s".to_string()),
            default: Some("default_val".to_string()),
            on_dismiss: None,
            on_timeout: None,
            batch: false,
            json: None,
            pretty: false,
            quiet: false,
            silent: false,
            log_level: "warn".to_string(),
            template: None,
            var: None,
        };

        let config = NotificationConfig::from_cli(&cli).unwrap();
        assert_eq!(config.title, "Test Title");
        assert_eq!(config.subtitle, Some("Test Subtitle".to_string()));
        assert_eq!(config.message, Some("Test Message".to_string()));
        assert_eq!(config.sound, Some("default".to_string()));
        assert!(config.persistent);
        assert_eq!(config.timeout, Some(Duration::from_secs(30)));
        assert_eq!(config.default_value, Some("default_val".to_string()));
    }

    #[test]
    fn test_notification_config_requires_title() {
        let cli = Cli {
            command: None,
            title: None,
            subtitle: None,
            message: None,
            image: None,
            icon: None,
            sound: None,
            actions: None,
            reply: None,
            url: None,
            persistent: false,
            not_persistent: false,
            timeout: None,
            default: None,
            on_dismiss: None,
            on_timeout: None,
            batch: false,
            json: None,
            pretty: false,
            quiet: false,
            silent: false,
            log_level: "warn".to_string(),
            template: None,
            var: None,
        };

        assert!(NotificationConfig::from_cli(&cli).is_err());
    }
}
