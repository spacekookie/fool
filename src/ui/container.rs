//! A UI container which represents the entire rendering tree for fool
//!
//!

use cursive::Cursive;
use cursive::theme::Theme;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::*;

use std::sync::{Arc, Mutex};

use state::buffer::Buffer;
use ui::theme;
use ui::workspace::*;


pub enum FoolTheme {
    Dark,
    Light,
    Custom(Theme),
}

/// Represents the entire UI tree built for fool
pub struct UI {
    siv: Cursive,
    ws: Workspace,
}

impl UI {

    /// Initialise the UI with a theme
    pub fn new(t: FoolTheme, state: Arc<Mutex<Buffer>>) -> UI {
        let mut me = UI {
            siv: Cursive::new(),
            ws: Workspace::new(),
        };

        me.siv.set_theme(match t {
            FoolTheme::Dark => theme::dark(),
            FoolTheme::Light => theme::light(),
            FoolTheme::Custom(theme) => theme,
        });

        return me;
    }
}
