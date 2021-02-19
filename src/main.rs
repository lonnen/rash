use std::io::Write;
use std::io::{stdin, stdout};
use std::process::Command;

fn main() {
    let mut input = String::new();
    loop {
        print!("$ ");
        stdout().flush().ok().expect("Could not flush stdout");

        //stdin().read_line(&mut input).unwrap();

        //Command::new(input.trim())
        //    .spawn()
        //    .unwrap();
    }
}
