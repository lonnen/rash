use chrono::prelude::Local;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    let user = env::var("USER").unwrap();
    let exit_code: u8 = 0;

    loop {
        print!(
            "{user}@rash:{directory}:\n[{time}]",
            user = user,
            directory = env::current_dir().unwrap().to_string_lossy(),
            time = Local::now().format("%H:%M:%S").to_string(),
        );
        if exit_code != 0 {
            print!(" {}", exit_code);
        }
        print!(" $ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("failed to read line from prompt");

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut tokens = command.trim().split_whitespace();
            let command = tokens.next().unwrap();
            let args = tokens;

            match command {
                "" => continue,
                "cd" => {
                    const DEFAULT_DIR: &str = "/";
                    let new_dir = args.peekable().peek().map_or(DEFAULT_DIR, |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                }
                "exit" => return,
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // another command is piped so we need to patch the
                        // output to the next command
                        Stdio::piped()
                    } else {
                        // no more commands are piped so send to stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // wait until things are done
            final_command.wait().unwrap();
        }
    }
}
