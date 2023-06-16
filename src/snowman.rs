// snowman.rs - The library module for handling the execution of commands

use std::process::{Command, ExitStatus};
use std::io::Result;

pub struct SSH {
    shell: String,
    run: String,
    hostname: String
}

pub fn cmd_win() -> Result<ExitStatus> {
    // Arguments for the SSH Command
    let ssh_win = SSH {
        shell: String::from("cmd"),
        run: String::from("ping"),
        hostname: String::from("google.com")
    };
    
    // builds and spawns the SSH Command
    Command::new(ssh_win.shell)
        .arg("/k")
        .arg(ssh_win.run)
        .arg(ssh_win.hostname)
        .spawn()?.wait()
}