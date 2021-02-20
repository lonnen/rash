use std::io::Write;
use std::io::{stdin, stdout};
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        stdout().flush().ok().expect("Could not flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut child = Command::new(input.trim())
           .spawn()
           .unwrap();

        // wait for the command
        child.wait().ok().expect("1");
    }
}
