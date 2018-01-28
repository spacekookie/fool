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
use cursive::views::{TextView, BoxView, Panel, ViewRef};
use state::{Buffer, ChangeType};

use std::fmt::{Write, Display, Formatter, Result};


const CURSOR_CHAR: char = '█';


pub struct Workspace {
    position: usize
}

impl Workspace {
    pub fn new() -> Workspace {
        return Workspace {
            position: 0,
        };
    }

    pub fn setup(&mut self, siv: &mut Cursive) {
        let mut text_view = TextView::new("<PLACEHOLDER>");
        text_view.set_scrollable(false);

        let view = BoxView::with_full_screen(
            Panel::new(text_view.with_id("workspace")),
        ).with_id("box");

        siv.add_fullscreen_layer(view);
    }

    /// Simple utility function which draws a single line
    /// 
    /// It will check that the line length doesn't overflow the workspace but
    /// will not do any other checking in regards to height. That is handled by
    /// the general draw call
    fn draw_line<S: Into<String>>(&self, l: S, buffer: &mut String, x: usize, y: &mut usize) {
        let line: String = l.into();
        let length = line.len();
        let mut text: String = line.clone();
        if length + 1 >= x {
            let slice = &line[..x /* pad */ - 2 /* ... */- 4];
            text = format!("{} ...", slice);
        }

        *y += 1;
        text.push_str("\n");
        write!(buffer, "{}", &text).ok();
    }

    pub fn draw(&self, state: &Buffer, siv: &mut Cursive) {
        let mut text = String::new();
        let mut ay = 0;
        let Vec2 { x, mut y } = siv.screen_size();
        y -= 2; // Frame correction

        self.draw_line(format!("Remote:    {}", &state.remote), &mut text, x, &mut ay);
        self.draw_line(format!("Local:     {}", &state.local), &mut text, x, &mut ay);
        self.draw_line(format!("Head:      {}", &state.head), &mut text, x, &mut ay);
        self.draw_line("\n", &mut text, x, &mut ay);

        // eprintln!("Drawn so far {}", &ay);

        /* First add all untracked files */
        write!(&mut text, "Untracked files: \n").ok();
        for f in &state.untracked {
            if ay == self.position {
                write!(&mut text, "{} {}\n", CURSOR_CHAR, &f.0).ok();
            } else {
                write!(&mut text, "  {}\n", &f.0).ok();
            }

            ay += 1;
        }

        /* Some space */
        write!(&mut text, "\n").ok();

        /* Then add all unstaged */
        write!(&mut text, "Changes: \n").ok();
        for f in &state.unstaged {
            if ay == self.position {
                write!(&mut text, "{} {}\t  {}\n", CURSOR_CHAR, &f.1, &f.0).ok();
            } else {
                write!(&mut text, "  {}\t  {}\n", &f.1, &f.0).ok();
            }

            ay += 1;
        }

        /* Some space */
        write!(&mut text, "\n").ok();

        /* Finally everything staged */
        write!(&mut text, "Staged Changes: \n").ok();
        for f in &state.staged {
            if ay == self.position {
                write!(&mut text, "{} {}\t  {}\n", CURSOR_CHAR, &f.1, &f.0).ok();
            } else {
                write!(&mut text, "  {}\t  {}\n", &f.1, &f.0).ok();
            }

            ay += 1;
        }

        /* Some more space */
        write!(&mut text, "\n\n").ok();

        /* Add small cheat sheet */
        // write!(&mut text, "{}", HELP_FOOTER).ok();

        /* Update siv */
        let mut tv: ViewRef<TextView> = siv.find_id("workspace").unwrap();
        (&mut *tv).set_content(text);
    }
}