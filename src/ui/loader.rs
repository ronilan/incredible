use incredible_elements_extra::MarkdownLoader;

use crate::state::State;

pub fn build_loader() -> MarkdownLoader<State> {
    let loader = MarkdownLoader::<State>::new();
    loader
        .register_many(&[
            // Site and blog
            ("./index.md", include_str!("../content/index.md")),
            ("./blog/index.md", include_str!("../content/blog/index.md")),
            (
                "./blog/hello-world.md",
                include_str!("../content/blog/hello-world.md"),
            ),
            // Repo docs
            ("../README.md", include_str!("../../README.md")),
            (
                "./MARKDOWNS.md",
                include_str!("../../markdowns/MARKDOWNS.md"),
            ),
            (
                "./AI_POLICY.md",
                include_str!("../../markdowns/AI_POLICY.md"),
            ),
            (
                "./CONTRIBUTING.md",
                include_str!("../../markdowns/CONTRIBUTING.md"),
            ),
            (
                "./DEVELOPMENT.md",
                include_str!("../../markdowns/DEVELOPMENT.md"),
            ),
            (
                "./DEVELOPMENT_PREREQUISITES.md",
                include_str!("../../markdowns/DEVELOPMENT_PREREQUISITES.md"),
            ),
        ])
        .initial_path("./index.md");

    loader
}
