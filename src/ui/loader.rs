use incredible_elements_extra::MarkdownLoader;

use crate::state::State;

// Registers every embedded markdown document as a (path, content) pair. The
// loader key is the app-relative virtual path ("./blog/post.md"); the on-disk
// file it embeds is the same path relative to src/ui, i.e. "../../" + path.
//
// include_str! can only take a compile-time path literal (never a runtime loop
// variable), so instead of a for-loop this macro expands a flat list of virtual
// paths into chained .register() calls, each deriving its include path.
macro_rules! markdown_load {
    ($loader:expr, $($path:literal),* $(,)?) => {
        $(
            $loader.register($path, include_str!(concat!("../../", $path)));
        )*
    };
}

pub fn build_loader() -> MarkdownLoader<State> {
    let loader = MarkdownLoader::<State>::new();
    loader.initial_path("./index.md");

    markdown_load!(
        loader,
        // Site and blog
        "./index.md",
        "./blog/index.md",
        "./blog/hello-world.md",
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
