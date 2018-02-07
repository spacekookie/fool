//! The text render workspace built on top of cursive
//!
//! It handles a few things differently like overriding input handling to
//! manually scroll and manual text highlighting for certain words
//!
//! The workspace is initialised with a Buffer object which represents the current state
//! of a git repository
//!
//! ```notest
//! let b = workspace::new(git_workspace);
//! ```

use cursive::Cursive;
use cursive::traits::*;
use cursive::vec::Vec2;
use cursive::views::{BoxView, Panel, TextView, ViewRef};

use super::input::Command;
use super::layout::Layout;

#[derive(Debug)]
enum Grow {
    Up,
    Down,
    None,
}

pub struct Workspace {
    layout: Layout,
    position: usize,
    select_size: usize,
    growth: Grow,
    dirty: bool,
}

impl Workspace {
    pub fn new(layout: Layout) -> Workspace {
        return Workspace {
            layout: layout,
            position: 0,
            growth: Grow::None,
            select_size: 1,
            dirty: true,
        };
    }

    pub fn setup(&mut self, siv: &mut Cursive) {
        let mut text_view = TextView::new("<PLACEHOLDER>");
        text_view.set_scrollable(false);

        let view =
            BoxView::with_full_screen(Panel::new(text_view.with_id("workspace"))).with_id("box");

        siv.add_fullscreen_layer(view);
    }

    pub fn draw(&mut self, siv: &mut Cursive) {
        self.dirty = match self.dirty {
            true => {
                self.layout.update(siv.screen_size());
                let mut tv: ViewRef<TextView> = siv.find_id("workspace").unwrap();
                (&mut *tv).set_content(self.layout.render(self.position, self.select_size).as_ref());
                false
            }
            _ => false,
        };
    }

    pub fn cmd(&mut self, cmd: Command) {
        eprintln!("Getting event: {:?}", cmd);

        match cmd {
            /* A simple move up. Breaks multi-line select */
            Command::MoveUp => if self.position > 0 {
                self.position -= 1;
            },

            /* A simple move down. Breaks multi-line select */
            Command::MoveDown => if self.position < self.layout.len() - 1 {
                self.position += 1;
            },

            /* Either start or continue down growd. Shrinks up growth */
            Command::SelectDown => match self.growth {
                Grow::Down | Grow::None => {
                    self.growth = Grow::Down;
                    self.select_size += 1;
                }
                Grow::Up => self.select_size -= 1,
            },

            /* Either start or continue up growd. Shrinks down growth */
            Command::SelectUp => match self.growth {
                Grow::Up | Grow::None => {
                    self.growth = Grow::Up;
                    self.select_size += 1;
                    self.position -= 1; // Shift select point up
                }
                Grow::Down => {
                    self.select_size -= 1;
                    self.position += 1; // Shift select point down
                }
            },
        }

        self.dirty = true;
    }
}
