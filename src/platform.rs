#[cfg(target_arch = "wasm32")]
use std::sync::OnceLock;

#[cfg(target_arch = "wasm32")]
static CONTENT_COLUMNS: OnceLock<usize> = OnceLock::new();

#[cfg(target_arch = "wasm32")]
static CONTENT_ROWS: OnceLock<usize> = OnceLock::new();

pub fn init() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
pub fn set_content_columns(value: usize) {
    let _ = CONTENT_COLUMNS.set(value);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_content_columns(_value: usize) {}

#[cfg(target_arch = "wasm32")]
pub fn content_columns() -> Option<usize> {
    CONTENT_COLUMNS.get().copied()
}

#[cfg(target_arch = "wasm32")]
pub fn set_content_rows(value: usize) {
    let _ = CONTENT_ROWS.set(value);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn set_content_rows(_value: usize) {}

#[cfg(target_arch = "wasm32")]
pub fn content_rows() -> Option<usize> {
    CONTENT_ROWS.get().copied()
}


