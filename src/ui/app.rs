use incredible::*;
use incredible_elements::App;

use crate::state::State;
use crate::ui::{loader, markdown, scroll_window, theme};

pub(crate) fn build() -> App<State> {
    let app = App::default();
    app.title("Incredible")
        .exit_combination(Some(KeyCombination::new(Key::Escape, &[])))
        .showed(false);

    app.on_window(|el, _state, event| {
        if event.window == Window::Resize && event.loop_count == 0 {
            el.showed(true);
        }
        el.draw();
    });

    let scroll_window = scroll_window::build_scroll_window();

    let markdown = markdown::build_markdown();
    let loader = loader::build_loader();

    scroll_window.add(loader);
    scroll_window.add(markdown);

    app.add(scroll_window);

    theme::build_theme();

    app
}
