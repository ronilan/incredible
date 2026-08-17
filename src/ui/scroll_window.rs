use incredible::*;
use incredible_elements::{FrameKind, ScrollArea};
use incredible_elements_extra::Markdown;
use incredible_helpers_layout::*;

use crate::state::State;

pub fn build_scroll_window() -> ScrollArea<State> {
    let scroll_window: ScrollArea<State> = ScrollArea::default();
    scroll_window
        .width(Platform::columns())
        .height(Platform::rows())
        .kind(Some(FrameKind::Blank))
        .focused(true);

    scroll_window.on_window(|el, _state, event| {
        if event.window == Window::Resize {
            el.width(Platform::columns()).height(Platform::rows());

            if let Some(md) = el.elements.cot::<Markdown<State>>().first() {
                md.wrap_at(el.get_width().saturating_sub(4).min(80));
            }

            el.elements_snap_top()
                .elements_to_center_x()
                .elements_flow_down(1);
        }
        el.draw();
    });

    scroll_window
}
