use incredible::*;
use incredible_elements::{App, AppOptions};
use incredible_elements_extra::Markdown;
use incredible_elements_text_fonts::{BlockCharsStr, BlockKind};
use incredible_helpers_effects::*;
use incredible_helpers_layout::*;

#[derive(Clone, PartialEq, Default)]
pub struct State {}

const MARKDOWN: &str = include_str!("../README.md");

pub fn build() -> App<State> {
    let app = App::new(AppOptions {
        min_width: Some(60),
        ..Default::default()
    });
    app.exit_combination(Some(KeyCombination::new(Key::Escape, &[KeyMod::Alt])))
        .showed(false);
    app.on_window(|el, _state, event| {
        if event.window == Window::Resize {
            el.elements_down(1)
                .elements_flow_down(1)
                .elements_to_center()
                .showed(true);
        }
        el.draw();
    });

    fn label_gradient_decorate(el: &BlockCharsStr<State>) {
        // BlockCharsStr composes sub elements.
        // Create a look that is fully composited and decorated, ready for further processing.
        let flattened = flatten_composite(el, decorate, label_gradient_decorate);

        // Use animation progress as gradient offset if animation has ticked,
        // otherwise fall back to the last known progress, then 0.0.
        let progress = if let Some(anim) = el.get_animation() {
            if let Some(p) = anim.progress {
                p
            } else if let Some(p) = anim.last_progress {
                p
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Define gradient and create look.
        let stops = [[0, 95, 175], [175, 95, 175], [0, 175, 175], [0, 95, 175]];
        let look = gradient_color(&stops, GradientDirection::Horizontal, &flattened, progress);
        el.look(look);

        // Resolve and store the current decoration based on element style and status.
        el.decoration()
            .active_decor
            .replace(el.decoration().style.resolve(el.status()));
    }

    let label = BlockCharsStr::default();
    label
        .text("Incredible")
        .kind(BlockKind::Shadow)
        .animation(Some(Animation::new(2000.0, 8.0, 1.0)))
        .draw_override(Some(DrawOverride {
            auto_render: Some(AutoRenderOptions::default()),
            flatten_override: false,
        }));
    label
        .on_mouse(|el, _state, event| {
            if let Mouse::Down = event.mouse {
                if let Some(mut anim) = el.get_animation() {
                    anim.start_time = None;
                    el.animation(Some(anim));
                }
            }
        })
        .on_loop(|el, _state, _event| {
            if let Some(anim) = el.get_animation() {
                if let Some(_) = anim.progress {
                    el.draw();
                }
            }
        });
    label.renderer.decorate.set(label_gradient_decorate);

    let markdown = Markdown::default();
    markdown
        .markdown(MARKDOWN)
        .wrap_at(50)
        .copy_combination(Some(KeyCombination::new(Key::Char('c'), &[KeyMod::Ctrl])))
        .select_all_combination(Some(KeyCombination::new(Key::Char('e'), &[KeyMod::Ctrl])));

    app.add(label).add(markdown);
    app
}
