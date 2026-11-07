#[cfg(all(target_os = "macos", feature = "macos-native"))]
mod platform;
#[cfg(all(target_os = "macos", feature = "macos-native"))]
mod runtime;
#[cfg(all(target_os = "macos", feature = "macos-native"))]
mod state;
#[cfg(all(target_os = "macos", feature = "macos-native"))]
mod ui;

#[cfg(all(target_os = "macos", feature = "macos-native"))]
fn main() {
    platform::init();
    incredible_window_macos::run_app(runtime::run);
}

#[cfg(any(not(target_os = "macos"), not(feature = "macos-native")))]
fn main() {
    eprintln!(
        "This binary is only available for macOS targets with the 'macos-native' feature enabled."
    );
}
