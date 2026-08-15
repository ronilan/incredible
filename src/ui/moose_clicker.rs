use incredible::*;
use incredible_elements::Text;

use crate::state::State;

pub fn build_moose_clicker() -> Text<State> {
    let moose_clicker: Text<State> = Text::default();
    moose_clicker
        .handle("moose_clicker")
        .text(&format!("🫎: 0"))
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.clicks += 1;
            }
        })
        .on_state(|el, state| {
            el.text(&format!("🫎: {}", state.clicks));
            el.draw();
        });

    moose_clicker
}
