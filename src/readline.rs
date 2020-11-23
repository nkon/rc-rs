use std::io::{stdin, stdout, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;

use super::*;

fn redraw(
    output: &mut termion::raw::RawTerminal<std::io::Stdout>,
    prompt: &str,
    line: &str,
    prev: u16,
    cur: u16,
) {
    write!(
        output,
        "{}{}",
        clear::CurrentLine,
        cursor::Left(prev + prompt.len() as u16)
    )
    .unwrap();
    if cur < line.len() as u16 {
        write!(
            output,
            "{}{}{}",
            prompt,
            line,
            cursor::Left(line.len() as u16 - cur)
        )
        .unwrap();
    } else {
        write!(output, "{}{}", prompt, line).unwrap();
    }
    output.flush().unwrap();
}

fn do_backspace(line: &mut String, prev: u16) -> u16 {
    if prev == 0 {
        0
    } else if prev <= line.len() as u16 {
        line.remove(prev as usize - 1);
        prev - 1
    } else {
        unreachable!()
    }
}

fn do_left(line: &mut String, prev: u16) -> u16 {
    if prev == 0 {
        0
    } else if prev <= line.len() as u16 {
        prev - 1
    } else {
        unreachable!()
    }
}

fn do_right(line: &mut String, prev: u16) -> u16 {
    if prev < line.len() as u16 {
        prev + 1
    } else {
        prev
    }
}

fn do_insert(line: &mut String, prev: u16, c: char) -> u16 {
    line.insert(prev as usize, c);
    prev + 1
}

pub fn readline(env: &mut Env) -> String {
    let mut line = String::new();
    let mut cur_x: u16 = 0;
    let mut prev_cur_x: u16;
    let mut history_index = 0;
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
            // TODO: Delete
            Ok(event::Key::Backspace) => {
                prev_cur_x = cur_x;
                cur_x = do_backspace(&mut line, prev_cur_x);
                redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
            }
            Ok(event::Key::Left) => {
                prev_cur_x = cur_x;
                cur_x = do_left(&mut line, prev_cur_x);
                redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
            }
            Ok(event::Key::Right) => {
                prev_cur_x = cur_x;
                cur_x = do_right(&mut line, prev_cur_x);
                redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
            }
            Ok(event::Key::Up) => {
                if history_index > 0 {
                    history_index -= 1;
                    if history_index < history.len() {
                        prev_cur_x = line.len() as u16;
                        line = history[history_index].clone();
                        cur_x = line.len() as u16;
                        redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                    }
                }
            }
            Ok(event::Key::Down) => {
                history_index += 1;
                prev_cur_x = line.len() as u16;
                if history_index < history.len() {
                    line = history[history_index].clone();
                } else {
                    line = String::new();
                }
                cur_x = line.len() as u16;
                redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
            }
            Ok(event::Key::Char(c)) => match c {
                '\n' => {
                    history.push(line.clone());
                    history_index = history.len();
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
                c => {
                    // TODO: do_insert and test.
                    prev_cur_x = cur_x;
                    cur_x = do_insert(&mut line, cur_x, c);
                    redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                }
            },
            _ => {}
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backspace() {
        let mut line = String::from("01234");
        let next = do_backspace(&mut line, 0);
        assert_eq!(next, 0);
        assert_eq!(line, "01234");

        let mut line = String::from("01234");
        let next = do_backspace(&mut line, 1);
        assert_eq!(next, 0);
        assert_eq!(line, "1234");

        let mut line = String::from("01234");
        let next = do_backspace(&mut line, 4);
        assert_eq!(next, 3);
        assert_eq!(line, "0124");

        let mut line = String::from("01234");
        let next = do_backspace(&mut line, 5);
        assert_eq!(next, 4);
        assert_eq!(line, "0123");
    }
}

// BUG: test
// FIXME: test