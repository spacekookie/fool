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

use super::input::Input;

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

        let mut me = Ui {
            siv: siv,
            ws: Arc::new(Mutex::new(ws)),
            buffer: Arc::new(Mutex::new(state)),
        };

        let ws = Arc::clone(&me.ws);
        let b = Arc::clone(&me.buffer);
        me.siv.add_global_callback(WindowResize, move |s: &mut Cursive| {
            let buffer = b.lock().unwrap();
            ws.lock().unwrap().draw(&buffer, s);
        });

        /* Register all key callbacks */
        me.register_all_keys();

        return me;
    }

    /// Triggers the main rendering 
    pub fn run(&mut self) {
        self.siv.run();
    }

    /// Get the current size of the screen
    /// 
    /// **Note** You probably want to add your code to the ScreenResize callback! 
    pub fn get_screen_size(&self) -> Vec2 {
        return self.siv.screen_size();
    }

    /// To add a new key-map, add it to this list
    /// 
    /// Maybe at some point, this could take a nicer list of key-bindings, provided
    /// from the user config
    fn register_all_keys(&mut self) {
        Input::register_quit(&mut self.siv);
        Input::register_push(&mut self.siv, Arc::clone(&self.buffer), Arc::clone(&self.ws));
        Input::register_move_up(&mut self.siv, Arc::clone(&self.buffer), Arc::clone(&self.ws));
        Input::register_move_down(&mut self.siv, Arc::clone(&self.buffer), Arc::clone(&self.ws));
        
        Input::register_stage(&mut self.siv, Arc::clone(&self.buffer), Arc::clone(&self.ws));
        Input::register_unstage(&mut self.siv, Arc::clone(&self.buffer), Arc::clone(&self.ws));
        Input::register_commit(&mut self.siv, Arc::clone(&self.buffer), Arc::clone(&self.ws));
    }
}


