//! A buffer that holds a git context and then renders it
//! 
//! Is aware of cursor position and updates the rendered text according
//! to the user actions taken

enum ChangeType {
    Added, Modified, Deleted
}

pub struct Buffer {

    /// The selected position in the buffer 
    position: u64,

    /// Any file in the repo that is untracked
    untracked: Vec<String>,

    /// Staged files for a commit
    staged: Vec<(String, ChangeType)>,

    /// All changes with the type that is being applied
    unstaged: Vec<(String, ChangeType)>,

    // files: LinkedList<(String, BufferType)>,
}


impl Buffer {
    pub fn new() {
    }

    
}