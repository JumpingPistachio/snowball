use std::process::{Command, ExitStatus};
use std::io::Result;

pub fn win_cmd(args: &[&str]) -> Result<ExitStatus> {
    Command::new("cmd")
        .args(args)
        .spawn()?.wait()
}
fn main() {
    let _output = win_cmd(&["/k", "ping google.com"]);
}
