use std::process::Command;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    Command::new(input.trim())
        .spawn()
        .unwrap();
}
