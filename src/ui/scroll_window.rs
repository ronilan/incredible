use incredible::*;
use incredible_elements::ScrollArea;
use incredible_helpers_layout::*;

use crate::state::State;

pub fn build_scroll_window() -> ScrollArea<State> {
    let scroll_window: ScrollArea<State> = ScrollArea::scrollbars_unframed();
    scroll_window.clip_padding(ClipPadding::new(1, 1, 1, 0));

    scroll_window.on_window(|el, _state, event| {
        if event.window == Window::Resize {
            el.width(Platform::columns()).height(Platform::rows());

            el.elements_snap_top()
                .elements_to_center_x()
                .elements_flow_down(1);
        }
        el.draw();
    });

    scroll_window
}
