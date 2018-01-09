//! A wrapper module around git commands
//!
//! Every function that fool has access to should be conveniently wrapped here


use std::process::{Command, Stdio};
use std::io::Read;

use buffer::ChangeType;

/// Some git functions (stateless)
pub struct Git;
impl Git {

    /// Parses git status --porcelain to fill the buffer
    pub fn get_status() -> Vec<(ChangeType, String, bool)> {

        let mut vec = Vec::new();

        /* Makes *a lot* of assumptions */
        let res = Command::new("git")
            .stdout(Stdio::piped())
            .arg("status")
            .arg("--porcelain")
            .spawn()
            .ok()
            .unwrap();

        let output = &mut res.stdout.unwrap();
        let mut text = String::new();
        output.read_to_string(&mut text).ok();

        /* Split the array */
        let array = text.split("\n");
        for line in array {

            /* Check if valid */
            match &line.chars().nth(1) {
                &None => continue,
                _ => {}
            };

            let stage = &line.chars().nth(0).unwrap();
            let state = &line.chars().nth(1).unwrap();
            let file = String::from(line[2..].trim());

            // Test if data is valid for staging stage
            let stage_file = match stage {
                &'?' => Some(( ChangeType::Untracked, file.clone(), false )), // New file, not staged
                &'M' => Some(( ChangeType::Modified, file.clone(), true )), // Modification staged
                &'A' => Some(( ChangeType::Added, file.clone(), true )), // Addition staged
                _ => None
            };

            let modified_file = match state {
                &'?' => Some(( ChangeType::Untracked, file.clone(), false )), // New file, untracked
                &'D' => Some(( ChangeType::Deleted, file.clone(), false )), // Deletion
                &'M' => Some(( ChangeType::Modified, file.clone(), false )), // Modification
                _ => None
            };

            if stage_file.is_some() && stage_file != modified_file {
                vec.push(stage_file.unwrap());
            }

            if modified_file.is_some() {
                vec.push(modified_file.unwrap());
            }

            // /* Start consuming the string until we hit a ctl character */
            // for c in line.chars() {

            //     /* Means nothing was staged */
            //     if line.starts_with(' ') {
            //         staged = false;
            //         if c == ' ' {
            //             ctr += 1;
            //             continue;
            //         }
            //     }

            //     ctrl = match c {
            //        'M' => ChangeType::Modified,
            //        'D' => ChangeType::Deleted,
            //        'A' => ChangeType::Added,
            //        '?' =>
            //            if second {
            //                 second = false;
            //                 ctr += 1;
            //                 ChangeType::Added
            //            } else {
            //                 second = true;
            //                 continue;
            //            }
            //         _ => continue,
            //     }
            // }

            // if ctrl == ChangeType::None {
            //     continue;
            // }

            /* Next up read the rest of the string */
            // let mut meh: &str = &line[ctr..].trim();
            // let file = String::from(meh);

            // println!("Staged: {} | {} {}", staged, ctrl, file);
            // vec.push((file, ctrl, staged));
        }

        return vec;
    }

    pub fn stage(file: &str) {
        Command::new("git")
            .arg("add")
            .arg(format!("{}", file))
            .output()
            .ok();
    }

    pub fn stage_all() {
        Command::new("git").arg("add -A").output().ok();
    }

    pub fn unstage(file: &str) {
        Command::new("git")
            .arg("reset")
            .arg(format!("{}", file))
            .output()
            .ok();
    }

    pub fn commit(msg: &str) {
        Command::new("git")
            .arg(format!("commit"))
            .arg("-m")
            .arg(format!("\"{}\"", msg))
            .output()
            .ok();
    }

    pub fn commit_ammend() {}

    pub fn push() {
        Command::new("git").arg("push").output().ok();
   }
}