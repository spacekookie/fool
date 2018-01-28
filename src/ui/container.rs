//! A UI container which represents the entire rendering tree for fool
//!
//!

use cursive::Cursive;
use cursive::vec::Vec2;
use cursive::theme::Theme;
use cursive::event::Event::WindowResize;

// use cursive::traits::*;
// use cursive::views::*;

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
pub struct Ui {
    siv: Cursive,
    ws: Workspace,
}

impl Ui {

    /// Initialise the UI with a theme
    pub fn new(t: FoolTheme, state: Arc<Mutex<Buffer>>) -> Ui {
        let mut me = Ui {
            siv: Cursive::new(),
            ws: Workspace::new(),
        };

        me.siv.set_theme(match t {
            FoolTheme::Dark => theme::dark(),
            FoolTheme::Light => theme::light(),
            FoolTheme::Custom(theme) => theme,
        });

        me.siv.add_global_callback(WindowResize, |_| {
            
        });

        return me;
    }

    /// Get the current size of the screen
    /// 
    /// **Note** You probably want to add your code to the ScreenResize callback! 
    pub fn get_screen_size(&self) -> Vec2 {
        return self.siv.screen_size();
    }
}
