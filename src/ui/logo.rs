use incredible::*;
use incredible_elements_text_fonts::{BlockCharsStr, BlockKind, BlockSize};
use incredible_helpers_effects::*;

use crate::state::State;

pub fn build_logo(width: usize) -> BlockCharsStr<State> {
    let logo: BlockCharsStr<State> = BlockCharsStr::default();
    let size = if width >= 80 {
        BlockSize::Big
    } else {
        BlockSize::Small
    };
    let kind = if width >= 80 {
        BlockKind::Shadow
    } else {
        BlockKind::Plain
    };

    logo.handle("logo")
        .text("Incredible")
        .kind(kind)
        .size(size)
        .style_handle("GradientLabel")
        .animation(Some(Animation::new(2000.0, 8.0, 1.0)));

    fn logo_effects(el: &BlockCharsStr<State>) {
        decorate_rules::<State, BlockCharsStr<State>>(el, logo_effects);
    }

    logo.on_mouse(|el, _state, event| {
        if let Mouse::Down = event.mouse {
            if let Some(mut anim) = el.get_animation() {
                anim.start_time = None;
                el.animation(Some(anim));
            }
        }
    });

    // Binds the rules-based decorator AND the internal auto-redraw loop
    effect(&logo, logo_effects);

    logo
}
