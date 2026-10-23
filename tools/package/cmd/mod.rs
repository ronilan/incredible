pub mod build;
pub mod build_all;
pub mod bundle;
pub mod clean;
pub mod publish;
pub mod release;
pub mod web;

use std::io;
use std::process::Command;

/// Builds and runs the sibling `static_site` binary.
///
/// Site content lives in `static_site/` and compiles as its own bin, so there
/// is no repo-root copy (that path is the source directory). Invoking it via
/// `cargo run` matches the other targets: the build happens as part of
/// packaging, and the generated pages can never be stale.
pub fn run_static_site() -> io::Result<()> {
    let status = Command::new("cargo")
        .args(["run", "--release", "--quiet", "--bin", "static_site"])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "static_site exited with {}",
            status.code().unwrap_or(-1)
        )))
    }
}
