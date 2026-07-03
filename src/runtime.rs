use crate::{app, platform};

pub fn run() -> incredible::tui::DeferredValue<app::State> {
    platform::init();

    app::build().run(app::State::default())
}
