fn main() {
    // Link the UserNotifications framework on macOS
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=UserNotifications");
}
