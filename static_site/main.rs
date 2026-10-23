mod cargo;
mod render;
mod shell;

use std::process;

fn main() {
    if let Err(e) = render::build() {
        eprintln!("static_site: {}", e);
        process::exit(1);
    }
}
