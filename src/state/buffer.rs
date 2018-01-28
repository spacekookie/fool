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
    
    
    /// Trigger the buffer to update itself via the git interface
    pub fn update(&mut self) {
        self.clear();

        println!("Running update...");

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