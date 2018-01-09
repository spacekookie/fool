//! A wrapper module around git commands
//!
//! Every function that fool has access to should be conveniently wrapped here


use std::process::{Command, Stdio};
use std::io::Read;

use buffer::ChangeType;

pub struct Git;
impl Git {
    /// Parses git status --porcelain to fill the buffer
    pub fn get_status() {

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

            let mut ctr = 0;
            let mut ctrl: ChangeType = ChangeType::None;
            let mut second = false;

            /* Start consuming the string until we hit a ctl character */
            for c in line.chars() {
                if c == ' ' {
                    ctr += 1;
                    continue;
                }

                ctrl = match c {
                   'M' => ChangeType::Modified,
                   'D' => ChangeType::Deleted,
                   'A' => ChangeType::Added,
                   '?' => 
                       if second {
                            second = false;
                            ctr += 1;
                            ChangeType::Added
                       } else {
                            second = true;
                            continue;
                       }
                    _ => continue,
                }
            }

            if ctrl == ChangeType::None {
                continue;
            }

            /* Next up read the rest of the string */
            let mut meh: &str = &line[ctr..].trim();          
            let file = String::from(meh);
            println!("{} => '{}'", ctrl, file);
        }
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