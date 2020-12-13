pub use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    Command, Result,
};
use std::io::{stdout, Write};

use super::*;

fn redraw<W>(output: &mut W, prompt: &str, line: &str, prev: u16, cur: u16)
where
    W: Write,
{
    queue!(
        output,
        terminal::Clear(ClearType::CurrentLine),
        cursor::MoveLeft(prev + prompt.len() as u16)
    )
    .unwrap();
    if cur < line.len() as u16 {
        queue!(
            output,
            style::Print(format!("{}{}", prompt, line)),
            cursor::MoveLeft(line.len() as u16 - cur)
        )
        .unwrap();
    } else {
        queue!(output, style::Print(format!("{}{}", prompt, line))).unwrap();
    }
    output.flush().unwrap();
}

fn result_print<W>(output: &mut W, s: &str)
where
    W: Write,
{
    queue!(
        output,
        style::SetAttribute(style::Attribute::Bold),
        style::SetForegroundColor(style::Color::Yellow),
        style::Print(s),
        style::SetAttribute(style::Attribute::Reset),
    )
    .unwrap();
    output.flush().unwrap();
}

fn error_print<W>(output: &mut W, s: &str)
where
    W: Write,
{
    queue!(
        output,
        style::SetAttribute(style::Attribute::Bold),
        style::SetForegroundColor(style::Color::Red),
        style::Print(s),
        style::SetAttribute(style::Attribute::Reset),
    )
    .unwrap();
    output.flush().unwrap();
}

fn do_delete(line: &mut String, prev: u16) -> u16 {
    if prev < line.len() as u16 {
        line.remove(prev as usize);
    }
    prev
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

pub fn readline(env: &mut Env) {
    let mut line = String::new();
    let mut cur_x: u16 = 0;
    let mut prev_cur_x: u16;
    let mut history_index = 0;
    let mut history: Vec<String> = Vec::new();

    enable_raw_mode().unwrap();
    let mut stdout = stdout();

    // goto raw mode
    write!(stdout, "Ctrl-c or \"exit()\" to exit\r\n").unwrap();
    write!(stdout, "rc> ").unwrap();
    stdout.flush().unwrap();

    loop {
        let event = read().unwrap();
        // println!("Event::{:?}\r", event);

        if let Event::Key(keyev) = event {
            // print!("keyev={:?}\r\n", keyev);
            if keyev.modifiers == KeyModifiers::CONTROL && keyev.code == KeyCode::Char('c') {
                write!(stdout, "\r\n").unwrap();
                break;
            }
            match keyev.code {
                KeyCode::Delete => {
                    prev_cur_x = cur_x;
                    cur_x = do_delete(&mut line, prev_cur_x);
                    redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                }
                KeyCode::Backspace => {
                    prev_cur_x = cur_x;
                    cur_x = do_backspace(&mut line, prev_cur_x);
                    redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                }
                KeyCode::Left => {
                    prev_cur_x = cur_x;
                    cur_x = do_left(&mut line, prev_cur_x);
                    redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                }
                KeyCode::Right => {
                    prev_cur_x = cur_x;
                    cur_x = do_right(&mut line, prev_cur_x);
                    redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                }
                KeyCode::Up => {
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
                KeyCode::Down => {
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
                KeyCode::Enter => {
                    history.push(line.clone());
                    history_index = history.len();
                    write!(stdout, "\r\n").unwrap();
                    match lexer(line.clone()) {
                        Ok(v) => {
                            if v.is_empty() {
                                line.clear();
                                cur_x = 0;
                                prev_cur_x = cur_x;
                                redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                                continue;
                            }
                            match parse(env, &v) {
                                Ok(node) => {
                                    match eval(env, &node) {
                                        Ok(node) => match node {
                                            Node::Num(n) => {
                                                result_print(
                                                    &mut stdout,
                                                    format!("{}\r\n", output_format_num(env, n))
                                                        .as_str(),
                                                );
                                            }
                                            Node::FNum(f) => {
                                                result_print(
                                                    &mut stdout,
                                                    format!("{}\r\n", output_format_float(env, f))
                                                        .as_str(),
                                                );
                                            }
                                            Node::CNum(c) => {
                                                result_print(
                                                    &mut stdout,
                                                    format!("{}\r\n", c).as_str(),
                                                );
                                            }
                                            Node::Command(_cmd, _params, result) => {
                                                error_print(
                                                    &mut stdout,
                                                    format!("{}\r\n", result).as_str(),
                                                );
                                            }
                                            _ => {
                                                error_print(&mut stdout, format!("eval error: Unexpected eval result {:?}\r\n", node).as_str());
                                            }
                                        },
                                        Err(e) => {
                                            error_print(&mut stdout, format!("{}\r\n", e).as_str());
                                        }
                                    }
                                }
                                Err(e) => {
                                    error_print(&mut stdout, format!("{}\r\n", e).as_str());
                                }
                            }
                        }
                        Err(e) => {
                            error_print(&mut stdout, format!("{}\r\n", e).as_str());
                        }
                    }
                    line.clear();
                    cur_x = 0;
                    prev_cur_x = cur_x;
                    redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                }
                KeyCode::Char(c) => {
                    prev_cur_x = cur_x;
                    cur_x = do_insert(&mut line, cur_x, c);
                    redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                }
                _ => {}
            }
        }
    }
    disable_raw_mode().unwrap();
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

        let mut line = String::from("01234");
        let next = do_delete(&mut line, 4);
        assert_eq!(next, 4);
        assert_eq!(line, "0123");

        let mut line = String::from("01234");
        let next = do_insert(&mut line, 5, 'c');
        assert_eq!(next, 6);
        assert_eq!(line, "01234c");
    }
}

// BUG: test
// FIXME: test
