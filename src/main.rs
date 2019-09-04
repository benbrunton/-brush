use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor::DetectCursorPos;
use std::{io, thread, time};
use std::io::{stdin, stdout, Write};
use std::process::{Command, Stdio};
use std::path::Path;
use std::env;

const CD_DEFAULT: &str = "~";

fn main() {
    println!("Brush - the \x1b[1;33mBR\x1b[0munton \x1b[1;33mSH\x1b[0mell");

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    let prompt = "> ";
    write!(stdout, "{}", prompt);
    stdout.lock().flush().unwrap();

    let mut last_command = String::new();

    loop {

        let input = stdin.next();

        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Ctrl('c') => break,
                termion::event::Key::Ctrl('l') => {
                    write!(
                        stdout,
                        "{}{}{}",
                        prompt,
                        termion::clear::All,
                        termion::cursor::Goto(1, 1)
                    ).unwrap();
                    stdout.lock().flush().unwrap();

                },
                termion::event::Key::Char('\n') => {
                    write!(
                        stdout,
                        "{}{}",
                        '\r',
                        '\n',
                    ).unwrap();

                    let (
                        output,
                        end_shell
                    ) = execute_str(&last_command);

                    if !end_shell {
                        break; 
                    }

                    let formatted_output = format_output(output);

                    write!(
                        stdout,
                        "{}{}",
                        formatted_output,
                        prompt
                    ).unwrap();

                    stdout.lock().flush().unwrap();
                    last_command = String::new();
                    
                },
                termion::event::Key::Char(n) => {
                    write!(
                        stdout,
                        "{}",
                        n
                    )
                    .unwrap();

                    stdout.lock().flush().unwrap();

                    last_command.push(n);

                },
                termion::event::Key::Left => {
                    write!(
                        stdout,
                        "{}",
                        termion::cursor::Left(0)
                    )
                    .unwrap();
                    stdout.lock().flush().unwrap();
                },
                termion::event::Key::Right => {
                    write!(
                        stdout,
                        "{}",
                        termion::cursor::Right(0)
                    )
                    .unwrap();

                    stdout.lock().flush().unwrap();
                },

                termion::event::Key::Backspace => {
                    write!(
                        stdout,
                        "{}{}",
                        termion::cursor::Left(0),
                        termion::clear::AfterCursor,
                    )
                    .unwrap();

                    stdout.lock().flush().unwrap();

                },
                _ => ()
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}

fn execute_str(input: &str) -> (String, bool) {
   
    let mut parts = input.trim().split_whitespace();
    let command_option = parts.next();
    let command = command_option.unwrap_or("");
    let args = parts;

    match command {
        "" => (String::new(), true),
        "cd" => {
            let new_dir = args.peekable()
                .peek()
                .map_or(CD_DEFAULT, |x| *x);
            let root = Path::new(new_dir);
            env::set_current_dir(&root);
            (String::new(), true)
        },
        "exit" => (String::new(), false),
        command => {
            let child = Command::new(command)
                .args(args)
                .stdout(Stdio::piped())
                .spawn();

            let output = match child {
                Ok(mut child) => { 
                    let output = child.wait_with_output().unwrap(); 
                    String::from_utf8(output.stdout).unwrap()
                },
                Err(e) => format!("{}", e)
            };
            (output, true)
        }
    }
}

fn format_output(output: String) -> String{
    output.replace("\n", "\n\r")
}

