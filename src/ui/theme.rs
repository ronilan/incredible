use incredible::*;
use incredible_elements::{
    HorizontalLineKind, HorizontalLineStyle, VerticalLineKind, VerticalLineStyle,
};
use incredible_helpers_effects::*;

pub fn build_theme() {
    theme_rule::<Style>("Markdown", |s| {
        s.hovered.pointer.set(Some(PointerShape::Text));
    });

    theme_rule::<Style>("MarkdownHeading", |s| {
        s.base.decor.color.set(Some(Color::from(13)));
    });

    theme_rule::<Style>("Link", |s| {
        s.base.decor.underline.set(Some(UnderlineKind::Dotted));
        s.hovered.decor.color.set(Some(Color::from(11)));
        s.hovered.pointer.set(Some(PointerShape::Pointer));
    });

    theme_rule::<Style>("MarkdownRelativeLink", |s| {
        s.base.decor.underline.set(Some(UnderlineKind::Single));
        s.hovered.decor.color.set(Some(Color::from(12)));
        s.hovered.pointer.set(Some(PointerShape::Pointer));
    });

    theme_rule::<Style>("ScrollArea::VerticalLine", |s| {
        s.base.decor.faint.set(Some(true));
        s.dragged.decor.faint.set(Some(false));
        s.hovered.pointer.set(Some(PointerShape::Grab));
        s.dragged.pointer.set(Some(PointerShape::Grabbing))
    });

    theme_rule::<VerticalLineStyle>("ScrollArea::VerticalLine", |s| {
        s.base.kind.set(Some(VerticalLineKind::Thin));
        s.hovered.kind.set(Some(VerticalLineKind::Thick));
    });

    theme_rule::<Style>("ScrollArea::HorizontalLine", |s| {
        s.base.decor.faint.set(Some(true));
        s.dragged.decor.faint.set(Some(false));
        // TODO: for drag need not reset whn off element (special case)
        s.hovered.pointer.set(Some(PointerShape::Grab));
        s.dragged.pointer.set(Some(PointerShape::Grabbing))
    });

    theme_rule::<HorizontalLineStyle>("ScrollArea::HorizontalLine", |s| {
        s.base.kind.set(Some(HorizontalLineKind::Thin));
        s.hovered.kind.set(Some(HorizontalLineKind::Thick));
    });

    theme_rule::<Style>("ScrollArea Markdown HorizontalLine", |s| {
        s.base.decor.faint.set(Some(true));
    });

    theme_rule::<HorizontalLineStyle>("ScrollArea Markdown HorizontalLine", |s| {
        s.base.kind.set(Some(HorizontalLineKind::Dotted));
    });

    transform_rule("GradientLabel", |flattened, progress| {
        gradient_color(
            &[
                Color::ansi(4),
                Color::ansi(5),
                Color::ansi(6),
                Color::ansi(4),
            ],
            GradientDirection::Horizontal,
            flattened,
            progress,
        )
    });
}
