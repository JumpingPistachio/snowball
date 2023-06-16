use std::process::{Command, ExitStatus};
use std::io::Result;

struct SSH {
    shell: String,
    run: String,
    hostname: String
}

pub fn cmd_win() -> Result<ExitStatus> {
    let ssh_win = SSH {
        shell: String::from("cmd"),
        run: String::from("ping"),
        hostname: String::from("google.com")
    };
    
    Command::new(ssh_win.shell)
        .arg("/k")
        .arg(ssh_win.run)
        .arg(ssh_win.hostname)
        .spawn()?.wait()
}
fn main() {
    let _output = cmd_win();

}
