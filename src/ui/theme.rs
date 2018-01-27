//! Contain default themes for fool
//! 
//! They exist in two flavour: light and dark. If no fool config is present
//! this (statically included) code can be used to generate those files 
//! without any demand for weird packages

use cursive::theme::{self, Color, Theme, Palette};
use cursive::theme::PaletteColor::*;

fn load_and_apply(p: Palette) -> Theme {
    let mut t = theme::load_default();
    t.palette = p;
    return t;
}

/// Creates and loads the dark colour theme
pub fn dark() -> Theme {
    let mut p = theme::default_palette();
    p[Background] = Color::from_256colors(236);
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

    return load_and_apply(p);
}

/// Creates and loads the light colour theme
pub fn light() -> Theme {
    let mut p = theme::default_palette();
    p[Background] = Color::from_256colors(236);
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

    return load_and_apply(p);
}
