#[cfg(all(
    not(target_arch = "wasm32"),
    any(not(target_os = "macos"), not(feature = "macos-native"))
))]
mod app;
#[cfg(all(
    not(target_arch = "wasm32"),
    any(not(target_os = "macos"), not(feature = "macos-native"))
))]
mod platform;
#[cfg(all(
    not(target_arch = "wasm32"),
    any(not(target_os = "macos"), not(feature = "macos-native"))
))]
mod runtime;

#[cfg(all(
    not(target_arch = "wasm32"),
    any(not(target_os = "macos"), not(feature = "macos-native"))
))]
fn main() {
    runtime::run();
}

#[cfg(all(
    not(target_arch = "wasm32"),
    target_os = "macos",
    feature = "macos-native"
))]
fn main() {
    eprintln!(
        "The 'incredible' binary is for Terminal. Use 'incredible_macos' for the macOS version, or build without the 'macos-native' feature."
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {}
