use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;

use super::*;

pub fn readline(env: &mut Env) -> String {
    let mut line = String::new();
    let mut cur_x = 0;
    let mut history: Vec<String> = Vec::new();

    // goto raw mode
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();

    writeln!(stdout, "Ctrl-c to exit").unwrap();
    write!(stdout, "{}", cursor::Left(500)).unwrap();
    write!(stdout, "rc> ").unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c {
            Ok(event::Key::Ctrl('c')) => break,
            Ok(event::Key::Backspace) => {
                let mut chars: Vec<char> = line.chars().collect();
                if chars.is_empty() {
                    continue;
                }
                chars.remove(cur_x - 1);
                cur_x -= 1;
                line = chars.into_iter().collect();
                write!(
                    stdout,
                    "{}{}",
                    clear::CurrentLine,
                    cursor::Left(line.len() as u16 + 5)
                )
                .unwrap();
                write!(
                    stdout,
                    "rc> {}{}",
                    line,
                    cursor::Left(line.len() as u16 - cur_x as u16)
                )
                .unwrap();
                stdout.flush().unwrap();
            }
            Ok(event::Key::Left) => {
                cur_x -= 1;
                write!(
                    stdout,
                    "{}{}",
                    clear::CurrentLine,
                    cursor::Left(line.len() as u16 + 5)
                )
                .unwrap();
                write!(
                    stdout,
                    "rc> {}{}",
                    line,
                    cursor::Left(line.len() as u16 - cur_x as u16)
                )
                .unwrap();
                stdout.flush().unwrap();
            }
            Ok(event::Key::Char(c)) => match c {
                '\n' => {
                    history.push(line.clone());
                    writeln!(stdout).unwrap();
                    write!(stdout, "{}", cursor::Left(500)).unwrap();
                    cur_x = 0;
                    match lexer(line.clone()) {
                        Ok(v) => {
                            let node = parse(env, &v);
                            let result = eval(env, &node);
                            match result.ty {
                                NodeType::Num => {
                                    write!(stdout, "{}", style::Bold).unwrap();
                                    writeln!(stdout, "{}", result.value).unwrap();
                                    write!(stdout, "{}", style::Reset).unwrap();
                                }
                                NodeType::FNum => {
                                    write!(stdout, "{}", style::Bold).unwrap();
                                    writeln!(stdout, "{}", result.fvalue).unwrap();
                                    write!(stdout, "{}", style::Reset).unwrap();
                                }
                                _ => {
                                    writeln!(stdout, "eval eror").unwrap();
                                }
                            }
                        }
                        Err(e) => {
                            writeln!(stdout, "{}", e).unwrap();
                        }
                    }
                    line.clear();
                    write!(stdout, "").unwrap();
                    write!(stdout, "{}", cursor::Left(500)).unwrap();
                    write!(stdout, "rc> ").unwrap();
                    stdout.flush().unwrap();
                }
                _ => {
                    write!(stdout, "{}", c).unwrap();
                    line.push(c);
                    cur_x += 1;
                    stdout.flush().unwrap();
                }
            },
            _ => {}
        }
    }
    String::new()
}
