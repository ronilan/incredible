use crate::{platform, state, ui::app};

pub fn run() -> incredible::tui::DeferredValue<state::State> {
    platform::init();

    app::build().run(state::State::default())
}
