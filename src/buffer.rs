//! A buffer that holds a git context and then renders it
//! 
//! Is aware of cursor position and updates the rendered text according
//! to the user actions taken

use std::fmt::{Write, Display, Formatter, Result};


#[derive(Eq, PartialEq)]
pub enum ChangeType {
    Added, Modified, Deleted
}

impl Display for ChangeType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return match self {
            &ChangeType::Added => write!(f, "Added"),
            &ChangeType::Modified => write!(f, "Modified"),
            &ChangeType::Deleted => write!(f, "Deleted"),
        };
    }
}


pub struct Buffer {

    /// The selected position in the buffer 
    position: u64,

    /// Any file in the repo that is untracked
    pub untracked: Vec<String>,

    /// Staged files for a commit
    pub staged: Vec<(String, ChangeType)>,

    /// All changes with the type that is being applied
    pub unstaged: Vec<(String, ChangeType)>,
}


impl Buffer {
    pub fn new() -> Buffer {
        return Buffer {
            position: 0,
            untracked: Vec::new(),
            staged: Vec::new(),
            unstaged: Vec::new(),
        };
    }

    /// Add a file as untracked
    /// 
    /// If it is currently staged it will be removed from staged
    /// If it is not staged, a new file is entered into the scope
    pub fn untracked(&mut self, file: String) {
        
        /* If the file was staged before */
        let (c, ctr) = contains(&self.staged, &file);
        if c {
            self.staged.remove(ctr);
        }
        
        self.untracked.push(file);
    }


    pub fn render(&self) -> String{
        let mut text = String::new();

        /* Write information about the git repository */
        write!(&mut text, "Local:    master ~/Projects/code/fool\n").ok();
        write!(&mut text, "Head:     adcb557 Working on the buffer logic\n").ok();

        /* Some space */
        write!(&mut text, "\n").ok();

        /* First add all untracked files */
        write!(&mut text, "Untracked files: \n").ok();
        for f in &self.untracked {
            write!(&mut text, "  {}\n", &f).ok();
        }

        /* Some space */
        write!(&mut text, "\n").ok();

        /* Then add all unstaged */
        write!(&mut text, "Changes: \n").ok();
        for f in &self.unstaged {
            write!(&mut text, "  {}\t  {}\n", &f.1, &f.0).ok();
        }

        /* Some space */
        write!(&mut text, "\n").ok();

        /* Finally everything staged */
        write!(&mut text, "Staged Changes: \n").ok();
        for f in &self.staged {
            write!(&mut text, "  {}\t  {}\n", &f.1, &f.0).ok();
        }

        /* Some more space */
        write!(&mut text, "\n\n").ok();

        /* Add small cheat sheet */
        write!(&mut text, "# Cheat Sheet\n").ok();
        write!(&mut text, "#    s = stage file/section, S = stage all unstaged files\n").ok();
        write!(&mut text, "#    c = commit, C = commit -a (add unstaged)\n").ok();
        write!(&mut text, "#    P = push to upstream\n").ok();

        return text;
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