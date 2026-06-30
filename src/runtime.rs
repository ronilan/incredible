use crate::{app, platform};

pub fn run() -> incredible::tui::DeferredValue<app::State> {
    platform::init();

    let app = app::build();

    if let Some(w) = app.options.min_width {
        platform::set_content_columns(w);
    }

    if let Some(h) = app.options.min_height {
        platform::set_content_rows(h);
    }

    app.run(app::State::default())
}
