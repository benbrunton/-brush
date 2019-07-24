use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;
use std::env;

const CD_DEFAULT: &str = "~";

fn main() {
    println!("Brush - the \x1b[1;33mBR\x1b[0munton \x1b[1;33mSH\x1b[0mell");
    loop {
        print!("> ");
        stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        // read_to_string 
        //      requires the std::io::Read trait to be imported

        // some whitespace characters need handling
        // but this will require reading from raw input
        // e.g.  clears screen in bash
        // ^[[A retrieves previous command
        // println!("{:?}", input);
        // outputting a \r allows overwrite of lines

        let mut parts = input.trim().split_whitespace();
        let command_option = parts.next();
        let command = command_option.unwrap_or("");
        let args = parts;

        match command {
            "" => (),
            "cd" => {
                let new_dir = args.peekable()
                    .peek()
                    .map_or(CD_DEFAULT, |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { 
                        let _ = child.wait(); 
                    },
                    Err(e) => eprintln!("{}", e)
                };
            }
        }

    }
}
