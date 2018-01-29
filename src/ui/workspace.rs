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

use std::sync::{Arc, Mutex};
use std::fmt::Write;

use super::input::Command;
use super::layout::Layout;
use state::{Buffer, ChangeType};


const CURSOR_CHAR: char = '█';
const HELP_FOOTER: &'static str = "# Cheat Sheet
#    s = stage, u = unstage, c = commit, P = push to upstream, Q = quit";

pub struct Workspace {
    layout: Layout,
    position: usize,
    items: usize,
}

impl Workspace {
    pub fn new(bf: Arc<Mutex<Buffer>>) -> Workspace {
        return Workspace {
            layout: Layout::new(bf, (0, 0)),
            position: 0,
            items: 0,
        };
    }

    pub fn setup(&mut self, siv: &mut Cursive) {
        let mut text_view = TextView::new("<PLACEHOLDER>");
        text_view.set_scrollable(false);

        let view =
            BoxView::with_full_screen(Panel::new(text_view.with_id("workspace"))).with_id("box");

        siv.add_fullscreen_layer(view);
    }

    pub fn cmd(&mut self, cmd: Command) {
        use self::Command::*;
        match cmd {
            Up => if self.position > 0 {
                self.position -= 1;
            },
            Down => if self.position < self.items - 1 {
                self.position += 1;
            },
        }
    }

    /// Get the current position of the cursor
    pub fn get_position(&self) -> usize {
        return self.position;
    }

    pub fn draw(&mut self, state: &Buffer, siv: &mut Cursive) {
        let mut text = String::new();
        let (mut ay, mut item) = (0, 0);
        let Vec2 { x, mut y } = siv.screen_size();
        y -= 4; // Frame correction + space for help

        /* Draw the header to the top of the screen */
        Workspace::draw_line(format!("Remote: {}", &state.remote), &mut text, x, &mut ay);
        Workspace::draw_line(format!("Local:  {}", &state.local), &mut text, x, &mut ay);
        Workspace::draw_line(format!("Head:   {}", &state.head), &mut text, x, &mut ay);

        /* Check if we have untracked files */
        if state.has_untracked() && ay < y - 2 {
            Workspace::draw_line("", &mut text, x, &mut ay);
            Workspace::draw_line("Untracked files:", &mut text, x, &mut ay);

            for f in &state.untracked {
                if ay > y - 2 {
                    /* Break rendering if screen is full */
                    break;
                }

                if item == self.position {
                    Workspace::draw_line(
                        format!("{} {}", CURSOR_CHAR, &f.0),
                        &mut text,
                        x,
                        &mut ay,
                    );
                } else {
                    Workspace::draw_line(format!("  {}", &f.0), &mut text, x, &mut ay);
                }

                item += 1;
            }
        }


        /* Check if we have untracked files */
        if state.has_unstaged() && ay < y - 2 {
            Workspace::draw_line("", &mut text, x, &mut ay);
            Workspace::draw_line("Changes:", &mut text, x, &mut ay);

            for f in &state.unstaged {
                if ay > y - 2 {
                    /* Break rendering if screen is full */
                    break;
                }

                if item == self.position {
                    Workspace::draw_line(
                        format!("{} {}", CURSOR_CHAR, &f.0),
                        &mut text,
                        x,
                        &mut ay,
                    );
                } else {
                    Workspace::draw_line(format!("  {}", &f.0), &mut text, x, &mut ay);
                }

                item += 1;
            }
        }


        /* Check if we have untracked files */
        if state.has_staged() && ay < y - 2 {
            Workspace::draw_line("", &mut text, x, &mut ay);
            Workspace::draw_line("Staged files:", &mut text, x, &mut ay);

            for f in &state.staged {
                if ay > y - 2 {
                    /* Break rendering if screen is full */
                    break;
                }

                if item == self.position {
                    Workspace::draw_line(
                        format!("{} {}", CURSOR_CHAR, &f.0),
                        &mut text,
                        x,
                        &mut ay,
                    );
                } else {
                    Workspace::draw_line(format!("  {}", &f.0), &mut text, x, &mut ay);
                }

                item += 1;
            }
        }

        /* Fill the rest of the buffer*/
        if ay < y {
            for _ in 0..y - ay {
                Workspace::draw_line("", &mut text, x, &mut ay);
            }
        }

        Workspace::draw_line(HELP_FOOTER, &mut text, x, &mut ay);

        /* Update siv */
        let mut tv: ViewRef<TextView> = siv.find_id("workspace").unwrap();
        (&mut *tv).set_content(text);

        /* Store the number of items we rendered */
        self.items = item;
    }

    /// Simple utility function which draws a single line
    ///
    /// It will check that the line length doesn't overflow the workspace but
    /// will not do any other checking in regards to height. That is handled by
    /// the general draw call
    fn draw_line<S: Into<String>>(l: S, buffer: &mut String, x: usize, y: &mut usize) {
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
}
