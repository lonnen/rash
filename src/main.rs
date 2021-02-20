use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("$ ");
        stdout().flush().ok().expect("Could not flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut tokens = input.trim().split_whitespace();
        let command = tokens.next().unwrap();
        let args = tokens;

        match command {
            "cd" => {
                const DEFAULT_DIR: &str = "/";
                let new_dir = args.peekable().peek().map_or(DEFAULT_DIR, |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn()
                    .unwrap();

                // wait for the command
                child.wait().ok().expect("1");
            }
        }
    }
}
