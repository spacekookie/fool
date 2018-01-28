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
use cursive::views::{TextView, BoxView, Panel};
use state::{Buffer, ChangeType};

pub struct Workspace {

}

impl Workspace {
    pub fn new() -> Workspace {
        return Workspace {};
    }

    pub fn setup(&mut self, siv: &mut Cursive) {
        let mut text_view = TextView::new("<PLACEHOLDER>");
        text_view.set_scrollable(false);
        let size = siv.screen_size();

        let view = BoxView::with_fixed_size(
            (size.x - 2, size.y - 2),
            Panel::new(text_view.with_id("text_area")),
        );

        siv.add_layer(view);
    }

    pub fn set_size(&mut self, size: Vec2) {
    }

    pub fn update(&mut self, state: &Buffer) {

    }
}