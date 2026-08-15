use crate::ui::{logo, moose_clicker};
use incredible::*;
use incredible_elements_extra::Markdown;

use crate::state::State;

pub fn build_markdown() -> Markdown<State> {
    let markdown: Markdown<State> = Markdown::default();
    markdown
        .wrap_at(80)
        .copy_combination(Some(KeyCombination::new(Key::Char('c'), &[KeyMod::Ctrl])))
        .select_all_combination(Some(KeyCombination::new(Key::Char('e'), &[KeyMod::Ctrl])));

    let moose_clicker = moose_clicker::build_moose_clicker();
    let logo = logo::build_logo();

    markdown.add_embed(moose_clicker).add_embed(logo);

    markdown
}
