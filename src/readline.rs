use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;

use super::*;

pub fn readline(env: &mut Env) -> String {
    let mut line = String::new();
    let mut history: Vec<String> = Vec::new();

    // goto raw mode
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();

    println!("Ctrl-c to exit");
    write!(stdout, "{}", cursor::Left(500)).unwrap();
    write!(stdout, "rc> ").unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c {
            Ok(event::Key::Ctrl('c')) => break,
            Ok(event::Key::Char(c)) => match c {
                '\n' => {
                    println!();
                    history.push(line.clone());                    
                    write!(stdout, "{}", cursor::Left(500)).unwrap();
                    match lexer(line.clone()) {
                        Ok(v) => {
                            let node = parse(env, &v);
                            let result = eval(env, &node);
                            match result.ty {
                                NodeType::Num => {
                                    write!(stdout, "{}", style::Bold).unwrap();
                                    println!("{}", result.value);
                                    write!(stdout, "{}", style::Reset).unwrap();
                                }
                                NodeType::FNum => {
                                    write!(stdout, "{}", style::Bold).unwrap();
                                    println!("{}", result.fvalue);
                                    write!(stdout, "{}", style::Reset).unwrap();
                                }
                                _ => {
                                    println!("eval eror");
                                }
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                    line.clear();
                    println!();
                    write!(stdout, "{}", cursor::Left(500)).unwrap();
                    write!(stdout, "rc> ").unwrap();
                    stdout.flush().unwrap();
                }
                _ => {
                    print!("{}", c);
                    line.push(c);
                    stdout.flush().unwrap();
                }
            },
            _ => {}
        }
    }
    String::new()
}
