//! A wrapper module around git commands
//!
//! Every function that fool has access to should be conveniently wrapped here


use std::process::Command;


pub struct Git;
impl Git {

    /// Parses git status --porcelain to fill the buffer
    pub fn get_status() {

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