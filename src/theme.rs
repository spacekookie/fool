//! The theme module which makes fool all nice and cozy

// pub const CURSIVE_THEME: &'static str = "shadow = false
// # borders = \"outset\"

// [colors]
// 	background = \"#333333\"
// 	shadow     = \"#333333\"
// 	view       = \"#CCC\"
// 	primary   = \"#3333FF\"
// 	secondary = \"#333\"
// 	tertiary  = \"#2D2D2D\"
// 	title_primary   = \"BLUE\"
// 	title_secondary = \"#FFF\"
// 	highlight          = \"#FFF\"
// 	highlight_inactive = \"#FFF\"";


use cursive::theme::{load_default, Color, Palette, Theme};
pub fn setup() -> Theme {
    let mut t = load_default();

    // /* But then add some 💩 */
    let pal = Palette {
        background: Color::from_256colors(236),             // 
        shadow: Color::from_256colors(236),                 // Dark background #303030
        view: Color::from_256colors(236),                   //
        primary: Color::from_256colors(253),                // Make the text white
        secondary: Color::from_256colors(240),              // Edit fields are slightly less far
        tertiary: Color::from_256colors(15),
        title_primary: Color::from_256colors(15),
        title_secondary: Color::from_256colors(15),
        highlight: Color::from_256colors(15),
        highlight_inactive: Color::from_256colors(15),
    };

    t.colors = pal;
    return t;
}
