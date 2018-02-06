//! The workspace layout engine
//!
//! A layout is created for a screen resolution and re-created for each
//! screen resolution. It holds the items of a buffer but can decide to
//! rearrange them in their order if necessary.

use std::fmt::{Display, Formatter, Result, Write};
use state::buffer::Buffer;
use cursive::vec::Vec2;

const CURSOR_CHAR: char = '█';
const HELP_FOOTER: &'static str = "# Cheat Sheet
#    s = stage, u = unstage, c = commit, P = push to upstream, Q = quit";

/// A layout represents items in a list on screen
pub struct Layout {
    text: String,
    buf: Buffer,
    start: usize,
    stop: usize,
}

impl Layout {
    pub fn new(buf: Buffer, res: Vec2) -> Layout {
        let mut me = Layout {
            buf,
            start: 0,
            stop: res.y,
            text: String::new(),
        };

        me.update(res);
        return me;
    }

    pub fn update(&mut self, res: Vec2) {
        
        /* Subtract the boundries around the text area */
        let mut res = res;
        let mut y_pos = 0;
        res.x -= 2;
        res.y -= 2;

        /* Update the dataset */
        self.buf.update();

        /* Then do formatting */
        self.draw_line((res.x, &mut y_pos), format!("Remote:\torigin @ https://github.com/spacekookie/fool"));
    }

    /// Draws a single line of text into the text buffer
    /// 
    /// Does boundry checking on how long the line can be as
    /// well as if it should add padding
    fn draw_line<S: Into<String>>(&mut self, res: (usize, &mut usize), line: S) {
        let line: String = line.into();
        let length = line.len();
        let mut text: String = line.clone();
        if length + 1 >= res.0 {
            let slice = &line[..res.0 /* padding */ - 2]; // /* pad */ - 2 /* ... */ - 4
            text = format!("{} ...", slice);
        }

        *res.1 += 1;
        text.push_str("\n");
        write!(self.text, "{}", &text).ok();
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{}", self.text);
    }
}
