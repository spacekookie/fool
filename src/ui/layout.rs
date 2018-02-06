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
const HELP_FOOTER_SIZE: usize = 4;

macro_rules! check_y_bound {
    ($curr:expr, $max:expr) => {
        if $curr > $max - HELP_FOOTER_SIZE {
            eprintln!("Running break condition!");
            break;
        }
    };
}

macro_rules! check_y2_bound {
    ($curr:expr, $max:expr) => {
        ($curr < $max - HELP_FOOTER_SIZE)
    };
}


pub struct LineSnippet {
    line: String,
    bounds: usize,
}

impl LineSnippet {
    pub fn new(line: String, bounds: usize) -> LineSnippet {
        return LineSnippet { line, bounds };
    }
}

/// A layout represents items in a list on screen
pub struct Layout {
    text: Vec<LineSnippet>,
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
            text: Vec::new(),
        };

        me.update(res);
        return me;
    }

    pub fn update(&mut self, res: Vec2) {
        /* Subtract the boundries around the text area */
        let mut res = res;
        res.x -= 2;
        res.y -= 2;

        /* Update the dataset */
        self.buf.update();
        self.text.clear();

        let mut text = Vec::new();
        self.format_buffer(&mut text, res);
        self.text = text;
    }

    /// Format the buffer with only a read-only copy of the innards
    fn format_buffer(&self, text: &mut Vec<LineSnippet>, res: Vec2) {
        let mut y_pos = 0;

        Layout::add_line(text, res.x, format!("Remote:  {}", &self.buf.remote));
        Layout::add_line(text, res.x, format!("Local:   {}", &self.buf.local));
        Layout::add_line(text, res.x, format!("Head:    {}", &self.buf.head));

        /* Draw the Untracked block (if we have space) */
        if self.buf.has_untracked() && check_y2_bound!(y_pos, res.y) {
            Layout::add_line(text, res.x, "");
            y_pos = Layout::add_line(text, res.x, "Untracked files:");

            for f in &self.buf.untracked {
                check_y_bound!(y_pos, res.y);
                y_pos = Layout::add_line(text, res.x, format!("  {}", &f.0));
            }
        }

        /* Draw the Changed block */
        if self.buf.has_unstaged() && check_y2_bound!(y_pos, res.y) {
            Layout::add_line(text, res.x, "");
            y_pos = Layout::add_line(text, res.x, "Changed files:");

            for f in &self.buf.unstaged {
                check_y_bound!(y_pos, res.y);
                y_pos = Layout::add_line(text, res.x, format!("  {}", &f.0));
            }
        }

        /* Draw the Staged block */
        if self.buf.has_staged() && check_y2_bound!(y_pos, res.y) {
            Layout::add_line(text, res.x, "");
            y_pos = Layout::add_line(text, res.x, "Staged files:");

            for f in &self.buf.staged {
                check_y_bound!(y_pos, res.y);
                y_pos = Layout::add_line(text, res.x, format!("  {}", &f.0));
            }
        }

        text.push(LineSnippet::new(String::from(""), res.x));
        text.push(LineSnippet::new(String::from(HELP_FOOTER), res.x));
    }

    /// Adds a single line (with bounds) to the layout vector
    /// 
    /// Returns the new size of the vector that can be tracked externally
    fn add_line<S: Into<String>>(text_buffer: &mut Vec<LineSnippet>, b: usize, line: S) -> usize {
        let line: String = line.into();
        let length = line.len();
        let bounds = b - 4; // Subtract space for the " ..." at the end of the line
        text_buffer.push(LineSnippet::new(line, bounds));
        return text_buffer.len();
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut s = String::new();
        for string in &self.text {
            let length = string.line.len();
            let mut tmp = string.line.clone();
            if length + 1 >= string.bounds {
                let slice = &string.line[..string.bounds];
                tmp = format!("{} ...", slice);
            }

            s.push_str(&tmp);
            s.push_str("\n");
        }
        return write!(f, "{}", s);
    }
}
