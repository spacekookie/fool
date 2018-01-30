//! The workspace layout engine
//!
//! A layout is created for a screen resolution and re-created for each
//! screen resolution. It holds the items of a buffer but can decide to
//! rearrange them in their order if necessary.

use std::fmt::{Display, Formatter, Result};
use state::buffer::Buffer;
use cursive::vec::Vec2;


/// A layout represents items in a list on screen
pub struct Layout<'a> {
    buf: &'a mut Buffer,
    start: usize,
    stop: usize,
}

impl<'a> Layout<'a> {
    pub fn new(buf: &'a mut Buffer, res: Vec2) -> Layout {
        let mut me = Layout {
            buf,
            start: 0,
            stop: res.y,
        };

        me.update(res);
        return me;
    }

    pub fn update(&mut self, res: Vec2) {
        /* Update the dataset */
        self.buf.update();

        /* Then do formatting */
    }
}

impl<'a> Display for Layout<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut text = String::new();
        return write!(f, "{}", text);
    }
}
