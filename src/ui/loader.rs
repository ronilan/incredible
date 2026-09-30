use incredible_elements_extra::{MarkdownLoader, is_supported_image_extension};

use crate::state::State;

// Dispatches an embedded file to the right store based on extension:
// images go to the ImageCache via register_image, everything else is UTF-8
// text registered as a markdown document. Using include_bytes! for all
// entries lets one macro handle both kinds with a single path list.
fn register_embedded(loader: &MarkdownLoader<State>, path: &str, bytes: &[u8]) {
    if is_supported_image_extension(path) {
        loader.register_image(path, bytes);
    } else {
        let text = std::str::from_utf8(bytes).expect("embedded text must be UTF-8");
        loader.register(path, text);
    }
}

// Registers every embedded document/image as an app-relative virtual path
// ("./blog/post.md"). The on-disk file it embeds is the same path relative
// to src/ui, i.e. "../../" + path.
//
// include_bytes! can only take a compile-time path literal (never a runtime
// loop variable), so instead of a for-loop this macro expands a flat list of
// virtual paths into per-file register_embedded calls, each deriving its
// include path.
macro_rules! markdown_load {
    ($loader:expr, $($path:literal),* $(,)?) => {
        $(
            register_embedded(&$loader, $path, include_bytes!(concat!("../../", $path)));
        )*
    };
}

pub fn build_loader() -> MarkdownLoader<State> {
    let loader = MarkdownLoader::<State>::new();
    loader.initial_path("./wasm.md").register_title("./wasm.md", "Incredible. A Modern Rust TUI Framework");

    markdown_load!(
        loader,
        // Site and blog
        "./wasm.md",
        "./blog/index.md",
        "./blog/listen-to-the-redditors.md",
        "./blog/hello-world.md",
        "./blog/tui-games-in-80x24.md",
        "./blog/listen-to-the-redditors.md",
        "./blog/images/tui-games-in-80x24.png",
        // Repo docs
        "./README.md",
        "./markdowns/index.md",
        "./markdowns/AI_POLICY.md",
        "./markdowns/CONTRIBUTING.md",
        "./markdowns/DEVELOPMENT.md",
        "./markdowns/DEVELOPMENT_PREREQUISITES.md",
    );

    loader
}
