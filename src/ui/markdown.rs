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

    markdown.on_window(|el, _state, event| {
        if event.window == Window::Resize {
            let wrap_at = Platform::columns().saturating_sub(4).min(80);
            // TODO: responsive logo. this is effective. encapsulation wil be more elegant
            el.remove_embed("logo").add_embed(logo::build_logo(wrap_at));
            el.wrap_at(wrap_at);
        }
        el.draw();
    });

    let moose_clicker = moose_clicker::build_moose_clicker();
    let logo = logo::build_logo(80);

    markdown.add_embed(moose_clicker).add_embed(logo);

    markdown
}
