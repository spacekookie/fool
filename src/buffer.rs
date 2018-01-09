//! A buffer that holds a git context and then renders it
//!
//! Is aware of cursor position and updates the rendered text according
//! to the user actions taken
//!
//! The rendering considers the size of the terminal so that the container
//! view from cursive doesn't have to worry about scrolling.
//!
//! When rendering the view, the current cursor position is considered and
//! a footer with cheat sheet commands is added (can be turned off)

use std::fmt::{Write, Display, Formatter, Result};
const CURSOR_CHAR: char = '█';
// const HELP_FOOTER: &'static str = "# Cheat Sheet
// #    s = stage file/section, S = stage all unstaged files
// #    c = commit, C = commit -a (add unstaged)
// #    P = push to upstream";
const HELP_FOOTER: &'static str = "# Cheat Sheet
#    s = stage, u = unstage, c = commit, P = push to upstream, Q = quit";


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ChangeType {
    // An error type
    None,

    // A new, unstaged file
    Untracked,

    // A new, staged file
    Added,

    // Either tracked or staged
    Modified,

    // Either tracked or staged
    Deleted,
}

impl Display for ChangeType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return match self {
            &ChangeType::Added => write!(f, "Added"),
            &ChangeType::Modified => write!(f, "Modified"),
            &ChangeType::Deleted => write!(f, "Deleted"),
            &ChangeType::Untracked => write!(f, "Untracked"),
            _ => write!(f, "NONE <Invalid>"),
        };
    }
}


pub struct Buffer {
    /// The selected position in the buffer
    pub position: u64,

    /// Any file in the repo that is untracked
    untracked: Vec<(String, ChangeType)>,

    /// Staged files for a commit
    staged: Vec<(String, ChangeType)>,

    /// All changes with the type that is being applied
    unstaged: Vec<(String, ChangeType)>,

    /// Data about stuff
    remote: String,
    local: String,
    head: String,

    /// Height
    height: usize,
}


impl Buffer {
    pub fn new() -> Buffer {
        return Buffer {
            position: 0,
            untracked: Vec::new(),
            staged: Vec::new(),
            unstaged: Vec::new(),
            remote: String::new(),
            local: String::new(),
            head: String::new(),
            height: 0,
        };
    }

    /// Add a file as untracked
    ///
    /// If it is currently staged it will be removed from staged
    /// If it is not staged, a new file is entered into the scope
    pub fn add_untracked(&mut self, file: String) {

        /* If the file was staged before */
        let (c, ctr) = contains(&self.staged, &file);
        if c {
            self.staged.remove(ctr);
        }

        self.untracked.push((file, ChangeType::Added));
    }

    /// Stage a file for a certain type of action
    ///
    /// If it was previously untracked or unstaged, it will
    /// be removed from those sets before
    pub fn stage(&mut self, file: String, t: ChangeType) {

        /* If the file was untracked */
        let (untracked, ctr) = contains(&self.untracked, &file);
        if untracked {
            self.untracked.remove(ctr);
        }

        /* If the file was unstaged before */
        let (unstaged, ctr) = contains(&self.unstaged, &file);
        if unstaged {
            let _type = get_type(&self.unstaged, &file).unwrap();
            if _type != t {
                panic!("Invalid staging!");
            }

            self.unstaged.remove(ctr);
        }

        self.staged.push((file, t));
    }

    /// Add a file as unstaged
    pub fn add_unstaged(&mut self, file: String, t: ChangeType) {

        /* If added => untracked, if modified or deleted => just unstaged */
        match t {
            ChangeType::Added => self.untracked.push((file, t)),
            _ => self.unstaged.push((file, t)),
        }
    }

    pub fn set_remote(&mut self, remote: &str) {
        self.remote = String::from(remote);
    }

    pub fn set_head(&mut self, head: &str) {
        self.head = String::from(head);
    }

    pub fn set_local(&mut self, local: &str) {
        self.local = String::from(local);
    }

    pub fn clear(&mut self) {
        self.unstaged.clear();
        self.untracked.clear();
        self.staged.clear();
    }

    pub fn move_up(&mut self) {
        if self.len() == 0 {
            return;
        }
        if self.position > 0 {
            self.position -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.len() == 0 {
            return;
        }
        if self.position < self.len() as u64 - 1 {
            // FIXME: Why is the -1 required?
            self.position += 1;
        }
    }

    /// Get the currently selected file that can then be
    pub fn get_selection(&self) -> (String, ChangeType) {
        let untracked = self.untracked.len() as u64;
        let unstaged = self.unstaged.len() as u64;

        if self.position < untracked {
            return get_element(&self.untracked, self.position);

        } else if self.position >= untracked && self.position < untracked + unstaged {
            // Over unstaged
            return get_element(&self.unstaged, self.position - untracked);

        } else {
            return get_element(&self.staged, self.position - untracked - unstaged);
        }
    }

    pub fn render(&self) -> String {
        let mut text = String::new();

        /* Write information about the git repository */
        write!(&mut text, "{}\n", &self.remote).ok();
        write!(&mut text, "{}\n", &self.local).ok();
        write!(&mut text, "{}\n", &self.head).ok();

        /* Some space */
        write!(&mut text, "\n").ok();

        let mut current_line = 0;

        /* First add all untracked files */
        write!(&mut text, "Untracked files: \n").ok();
        for f in &self.untracked {
            if current_line == self.position {
                write!(&mut text, "{} {}\n", CURSOR_CHAR, &f.0).ok();
            } else {
                write!(&mut text, "  {}\n", &f.0).ok();
            }

            current_line += 1;
        }

        /* Some space */
        write!(&mut text, "\n").ok();

        /* Then add all unstaged */
        write!(&mut text, "Changes: \n").ok();
        for f in &self.unstaged {
            if current_line == self.position {
                write!(&mut text, "{} {}\t  {}\n", CURSOR_CHAR, &f.1, &f.0).ok();
            } else {
                write!(&mut text, "  {}\t  {}\n", &f.1, &f.0).ok();
            }

            current_line += 1;
        }

        /* Some space */
        write!(&mut text, "\n").ok();

        /* Finally everything staged */
        write!(&mut text, "Staged Changes: \n").ok();
        for f in &self.staged {
            if current_line == self.position {
                write!(&mut text, "{} {}\t  {}\n", CURSOR_CHAR, &f.1, &f.0).ok();
            } else {
                write!(&mut text, "  {}\t  {}\n", &f.1, &f.0).ok();
            }

            current_line += 1;
        }

        /* Some more space */
        write!(&mut text, "\n\n").ok();

        /* Add small cheat sheet */
        write!(&mut text, "{}", HELP_FOOTER).ok();

        return text;
    }

    pub fn len(&self) -> usize {
        return self.untracked.len() + self.unstaged.len() + self.staged.len();
    }

    //         let mut res = String::new();
    // for (i, ch) in x.chars().enumerate() {
    //     write!(&mut res, "{} {}\n", i, ch).unwrap();
    // }
}

fn contains(vec: &Vec<(String, ChangeType)>, item: &String) -> (bool, usize) {
    let mut ctr = 0;
    for meh in vec {
        if &meh.0 == item {
            return (true, ctr);
        }
        ctr += 1;
    }
    return (false, ctr);
}

/// Get the type of a string file
fn get_type(vec: &Vec<(String, ChangeType)>, item: &String) -> Option<ChangeType> {
    for meh in vec {
        if &meh.0 == item {
            return Some(meh.1.clone());
        }
    }

    return None;
}

fn get_element(vec: &Vec<(String, ChangeType)>, element: u64) -> (String, ChangeType) {
    let mut ctr = 0;
    for meh in vec {
        if ctr == element {
            return meh.clone();
        }

        ctr += 1;
    }

    return ("".to_owned(), ChangeType::Added);
}

/// A small utility function which counds the number of lines a string occupies
fn count_rows(string: &String) -> u64 {
    let mut lines = 1;
    for c in string.chars() {
        if c == '\n' {
            lines += 1;
        }
    }

    return lines;
}