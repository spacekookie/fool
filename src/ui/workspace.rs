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

/// Represents an ever changing block of selection
struct SelectBlock {
    pub mode: Grow,
    pub pos: usize,
    pub size: usize,
}

impl SelectBlock {
    pub fn new() -> SelectBlock {
        return SelectBlock {
            mode: Grow::None,
            pos: 0,
            size: 0,
        };
    }
}

pub struct Workspace {
    select: SelectBlock,
    pub layout: Layout,
    pub dirty: bool,
}

impl Workspace {
    pub fn new(layout: Layout) -> Workspace {
        return Workspace {
            layout: layout,
            select: SelectBlock::new(),
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
                (&mut *tv).set_content(
                    self.layout
                        .render(self.select.pos, self.select.size)
                        .as_ref(),
                );
                false
            }
            _ => false,
        };
    }

    pub fn cmd(&mut self, cmd: Command) {
        match cmd {
            Command::SelectUp => match self.select.mode {
                Grow::None | Grow::Up => {
                    self.select.mode = Grow::Up;
                    if self.select.pos > 0 {
                        self.select.pos -= 1;
                    }
                    self.select.size += 1;
                }
                Grow::Down => {
                    self.select.size -= 1;
                    if self.select.size == 1 {
                        self.select.mode = Grow::None;
                    }
                }
            },
            Command::SelectDown => match self.select.mode {
                Grow::None | Grow::Down => {
                    self.select.mode = Grow::Down;
                    if self.select.pos + self.select.size < self.layout.len() {
                        self.select.size += 1;
                    }
                }
                Grow::Up => {
                    self.select.size -= 1;
                    if self.select.size == 1 {
                        self.select.mode = Grow::None;
                    }
                }
            },
            Command::MoveUp => {
                self.select.mode = Grow::None;
                self.select.size = 1;
                if self.select.pos > 0 {
                    self.select.pos -= 1;
                }
            }
            Command::MoveDown => {
                self.select.mode = Grow::None;
                self.select.size = 1;
                if self.select.pos < self.layout.len() - 1 {
                    self.select.pos += 1;
                }
            }
            Command::Dirty => { /* 🤷 */ }
        }

        self.dirty = true;
        // eprintln!(
        //     "Pos: {}, Size: {}, State: {:?}",
        //     self.select.pos,
        //     self.select.size,
        //     self.select.mode
        // );
    }
}
