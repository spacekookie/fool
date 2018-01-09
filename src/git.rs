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
                &'D' => Some(( ChangeType::Deleted, file.clone(), true )), // Deletion staged
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
        Command::new("git").arg("add").arg("-A").output().ok();
    }

    pub fn unstage(file: &str) {
        Command::new("git")
            .arg("reset")
            .arg(format!("{}", file))
            .output()
            .ok();
    }

    pub fn unstage_all() {
        Command::new("git")
            .arg("reset")
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

    pub fn push() {
        Command::new("git").arg("push").output().ok();
   }

    pub fn get_remote() -> String {
        let remote = String::from(run_utility("git remote show").trim());
        let url =  run_utility(&format!("git remote get-url {}", &remote));
        return format!("{} @ {}", remote.trim(), url.trim());
    }

    pub fn get_directory() -> String {
        let child = Command::new("bash").stdout(Stdio::piped()).arg("-c").arg("dirs +0").spawn().ok().unwrap();
        let output = &mut child.stdout.unwrap();
        let mut text = String::new();
        output.read_to_string(&mut text).ok();
        return String::from(text.trim());
    }

    pub fn get_branch_data() -> (String, String) {
        // * master ad7457a [ahead 6] "Changing frame sizes"
        let cool_string = String::from(run_utility("git branch -v").trim());
        let lines = cool_string.split("\n").collect::<Vec<&str>>();

        for line in lines {
            if line.starts_with('*') {
                let mut vector = line.split(" ").collect::<Vec<&str>>();
                vector.remove(0);

                let branch = vector[0];
                vector.remove(0);
                let mut commit = String::new();

                for word in &vector {
                    if word == &"" {
                        continue;
                    }

                    commit.push_str(word);
                    commit.push(' ');
                }

                return (String::from(branch), commit);
            }
        }

        return ("".to_owned(), "".to_owned());
    }
}

fn run_utility(command: &str) -> String {
    
    let mut array = command.split(" ");
    let mut res = Command::new(String::from(array.nth(0).unwrap()));
    res.stdout(Stdio::piped());
    let vec = array.collect::<Vec<&str>>();

    for arg in vec {
        res.arg(arg);
    }
    let child = res.spawn().ok().unwrap();

    let output = &mut child.stdout.unwrap();
    let mut text = String::new();
    output.read_to_string(&mut text).ok();
    return text;
}