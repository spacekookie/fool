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


use std::fmt::{Display, Formatter, Result};
use super::git::Git;


///
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ChangeType {
    /// An error type
    None,

    /// A new, unstaged file
    Untracked,

    /// A new, staged file
    Added,

    /// Either tracked or staged
    Modified,

    /// Either tracked or staged
    Deleted,

    /// Merge conflicted
    Conflicted,

    /// If a file content remained the same but then moved
    Renamed,
}

impl Display for ChangeType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return match self {
            &ChangeType::Added => write!(f, "Added"),
            &ChangeType::Modified => write!(f, "Modified"),
            &ChangeType::Deleted => write!(f, "Deleted"),
            &ChangeType::Untracked => write!(f, "Untracked"),
            &ChangeType::Conflicted => write!(f, "Conflict"),
            &ChangeType::Renamed => write!(f, "Renamed"),
            _ => write!(f, "NONE <Invalid>"),
        };
    }
}


/// Represent the buffered git repository state.
///
/// It also handles string concatination. It gives direct access to the buffered
/// git state, without doing any processing on the text itself. It shouldn't have
/// to track where the cursor position is and should at some point be merged with
/// a better git interface module.
///
/// Can be triggered to update it's view. It calls directly to the Git module
#[derive(Default, Debug)]
pub struct Buffer {
    /// Any file in the repo that is untracked
    pub untracked: Vec<(String, ChangeType)>,

    /// Staged files for a commit
    pub staged: Vec<(String, ChangeType)>,

    /// All changes with the type that is being applied
    pub unstaged: Vec<(String, ChangeType)>,

    /// The currently selected remote
    pub remote: String,

    /// The local repository state/ path
    pub local: String,

    /// The latest/ selected commit
    pub head: String,
}


impl Buffer {
    pub fn new() -> Buffer {
        return Default::default();
    }

    pub fn is_empty(&self) -> bool {
        return self.staged.is_empty() | self.unstaged.is_empty() | self.untracked.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.staged.len() + self.unstaged.len() + self.untracked.len()
    }

    pub fn has_untracked(&self) -> bool {
        return !self.untracked.is_empty();
    }

    pub fn has_unstaged(&self) -> bool {
        return !self.unstaged.is_empty();
    }

    pub fn has_staged(&self) -> bool {
        return !self.staged.is_empty();
    }

    pub fn get_element(&self, pos: usize) -> (String, ChangeType) {
        let untracked = self.untracked.len();
        let unstaged = self.unstaged.len();

        if pos < untracked {
            return get_element(&self.untracked, pos);
        } else if pos >= untracked && pos < untracked + unstaged {
            return get_element(&self.unstaged, pos - untracked);
        } else {
            return get_element(&self.staged, pos - untracked - unstaged);
        }
    }

    /// Trigger the buffer to update itself via the git interface
    pub fn update(&mut self) {
        self.clear();

        /* Fill in all the files */
        for (t, f, s) in Git::get_status() {
            if s {
                self.staged.push((f, t));
            } else {
                match t {
                    ChangeType::Untracked => self.untracked.push((f, ChangeType::Added)),
                    _ => self.unstaged.push((f, t)),
                }
            }
        }

        /* Fill in some metadata  */
        self.remote = Git::get_remote();
        self.local = format!("{} {}", Git::get_branch_data().0, Git::get_directory());
        self.head = Git::get_branch_data().1;
    }

    /// Clear the current state to start over from
    fn clear(&mut self) {
        self.untracked.clear();
        self.unstaged.clear();
        self.staged.clear();
    }
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

fn get_element(vec: &Vec<(String, ChangeType)>, element: usize) -> (String, ChangeType) {
    let mut ctr = 0;
    for meh in vec {
        if ctr == element {
            return meh.clone();
        }

        ctr += 1;
    }

    return ("".to_owned(), ChangeType::Added);
}
