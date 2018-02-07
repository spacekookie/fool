//! The workspace layout engine
//!
//! A layout is created for a screen resolution and re-created for each
//! screen resolution. It holds the items of a buffer but can decide to
//! rearrange them in their order if necessary.

use std::fmt::{Display, Formatter, Result, Write};
use state::Buffer;
use cursive::vec::Vec2;

const CURSOR_CHAR: char = '█';
const HELP_FOOTER_1: &'static str = "# Cheat Sheet";
const HELP_FOOTER_2: &'static str = "#    s = stage, u = unstage, c = commit, P = push to upstream, Q = quit";
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

#[derive(Clone)]
pub struct LineSnippet {
    /// Signals if this is a list item
    pub item: bool,
    line: String,
    bounds: usize,
}

impl LineSnippet {
    pub fn new(line: String, item: bool, bounds: usize) -> LineSnippet {
        return LineSnippet { line, item, bounds };
    }

    /// Prepend a piece of text in front of an item
    pub fn prepend<S: Into<String>>(&mut self, text: S) {
        self.line = format!("{}{}", text.into(), self.line);
    }
}

impl Display for LineSnippet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let length = self.line.len();
        let mut line = self.line.clone();
        if length + 1 >= self.bounds {
            let slice = &self.line[..length];
            line = format!("{} ...", slice);
        }

        line.push_str("\n");
        return write!(f, "{}", line);
    }
}

/// A layout represents items in a list on screen
pub struct Layout {
    text: Vec<LineSnippet>,
    length: usize,
    buf: Buffer,
    start: usize,
    stop: usize,
}

impl Layout {
    pub fn new(buf: Buffer, res: Vec2) -> Layout {
        let mut me = Layout {
            buf,
            length: 0,
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

        let mut ctr = 0;
        for line in &self.text {
            if line.item {
                ctr += 1;
            }
        }
        self.length = ctr;
        eprintln!("Items: {}", self.length);
    }

    pub fn render(&self, pos: usize, size: usize) -> String {
        let mut text = self.text.clone();
        let mut s = String::new();
        let mut ctr = 0;

        eprintln!("Pos: {}   Size: {}", pos, size);

        for line in &mut text {
            
            // if (ctr == pos || (size > 1 && ctr < pos + size)) && line.item {
            /* Prepend a cursor if the line is selected */
            if line.item && ((size > 1 && ctr < pos + size) || ctr == pos) {
                line.prepend(format!("{} ", CURSOR_CHAR));
            } else if line.item {
                line.prepend("  ");
            }

            /* Increment the line counter if list item */
            if line.item {
                ctr += 1;
            }

            s.push_str(&format!("{}", line));
        }

        return s;
    }

    pub fn len(&self) -> usize {
        return self.length;
    }

    /// Format the buffer with only a read-only copy of the innards
    fn format_buffer(&self, text: &mut Vec<LineSnippet>, res: Vec2) {
        let mut y_pos = 0;

        Layout::add_line(text, res.x, format!("Remote:  {}", &self.buf.remote), false);
        Layout::add_line(text, res.x, format!("Local:   {}", &self.buf.local), false);
        Layout::add_line(text, res.x, format!("Head:    {}", &self.buf.head), false);

        /* Draw the Untracked block (if we have space) */
        if self.buf.has_untracked() && check_y2_bound!(y_pos, res.y) {
            Layout::add_line(text, res.x, "", false);
            y_pos = Layout::add_line(text, res.x, "Untracked files:", false);

            for f in &self.buf.untracked {
                check_y_bound!(y_pos, res.y);
                y_pos = Layout::add_line(text, res.x, format!("{}", &f.0), true);
            }
        }

        /* Draw the Changed block */
        if self.buf.has_unstaged() && check_y2_bound!(y_pos, res.y) {
            Layout::add_line(text, res.x, "", false);
            y_pos = Layout::add_line(text, res.x, "Changed files:", false);

            for f in &self.buf.unstaged {
                check_y_bound!(y_pos, res.y);
                y_pos = Layout::add_line(text, res.x, format!("{}", &f.0), true);
            }
        }

        /* Draw the Staged block */
        if self.buf.has_staged() && check_y2_bound!(y_pos, res.y) {
            Layout::add_line(text, res.x, "", false);
            y_pos = Layout::add_line(text, res.x, "Staged files:", false);

            for f in &self.buf.staged {
                check_y_bound!(y_pos, res.y);
                y_pos = Layout::add_line(text, res.x, format!("{}", &f.0), true);
            }
        }

        /* Fill the rest of the buffer*/
        if y_pos - 2 < res.y {
            for _ in 0..res.y - 2 - y_pos {
                Layout::add_line(text, res.x, "", false);
            }
        }

        /* Always include the help footer */
        Layout::add_line(text, res.x, HELP_FOOTER_1, false);
        Layout::add_line(text, res.x, HELP_FOOTER_2, false);
    }

    /// Adds a single line (with bounds) to the layout vector
    ///
    /// Returns the new size of the vector that can be tracked externally
    fn add_line<S: Into<String>>(
        text_buffer: &mut Vec<LineSnippet>,
        b: usize,
        line: S,
        item: bool,
    ) -> usize {
        let line: String = line.into();
        let bounds = b - 4; // Subtract space for the " ..." at the end of the line
        text_buffer.push(LineSnippet::new(line, item, bounds));
        return text_buffer.len();
    }
}
