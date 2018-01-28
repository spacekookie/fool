//! A UI container which represents the entire rendering tree for fool
//!
//!

use cursive::Cursive;
use cursive::theme::Theme;

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
    ws: Arc<Mutex<Workspace>>,
    buffer: Arc<Mutex<Buffer>>,
}

impl Ui {

    /// Initialise the UI with a theme
    pub fn new(t: FoolTheme, state: Buffer) -> Ui {

        /* Initialise Cursive */
        let mut siv = Cursive::new();
        siv.set_theme(match t {
            FoolTheme::Dark => theme::dark(),
            FoolTheme::Light => theme::light(),
            FoolTheme::Custom(theme) => theme,
        });
        
        /* Initialise Workspace initially */
        let mut ws = Workspace::new();
        ws.setup(&mut siv);
        ws.draw(&state, &mut siv);

        return Ui {
            siv: siv,
            ws: Arc::new(Mutex::new(ws)),
            buffer: Arc::new(Mutex::new(state)),
        };
    }

    /// Triggers the main rendering 
    pub fn run(&mut self) {
        self.siv.run();
    }

}
