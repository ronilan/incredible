#[cfg(all(target_os = "macos", feature = "macos-native"))]
mod app;
#[cfg(all(target_os = "macos", feature = "macos-native"))]
mod platform;
#[cfg(all(target_os = "macos", feature = "macos-native"))]
mod runtime;

#[cfg(all(target_os = "macos", feature = "macos-native"))]
fn main() {
    incredible_window_macos::set_window_title(option_env!("APP_NAME").unwrap_or("Incredible"));
    platform::init();
    incredible_window_macos::run_app(runtime::run);
}

#[cfg(any(not(target_os = "macos"), not(feature = "macos-native")))]
fn main() {
    eprintln!(
        "This binary is only available for macOS targets with the 'macos-native' feature enabled."
    );
}
