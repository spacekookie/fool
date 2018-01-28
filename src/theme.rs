//! The theme module which makes fool all nice and cozy

use cursive::theme::{self, load_default, Color, Theme};
use cursive::theme::PaletteColor::*;

#[deprecated]
pub fn setup() -> Theme {
    let mut t = load_default();

    /* This is the dark colour palette */
    let mut p = theme::default_palette();
    p[Background] = Color::from_256colors(236);
    p[Shadow] = Color::from_256colors(236);
    p[View] = Color::from_256colors(236);
    p[Primary] = Color::from_256colors(253);
    p[Secondary] = Color::from_256colors(242);
    p[Tertiary] = Color::from_256colors(15);
    p[TitlePrimary] = Color::from_256colors(141);
    p[TitleSecondary] = Color::from_256colors(141);
    p[Highlight] = Color::from_256colors(15);
    p[HighlightInactive] = Color::from_256colors(15);

    t.palette = p;
    return t;
}
