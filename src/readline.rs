pub use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    Command, Result,
};
use std::io::{stdout, Write};
use std::iter::FromIterator;
use std::{thread, time};

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

fn redraw_highlight<W>(
    output: &mut W,
    prompt: &str,
    line: &str,
    prev: u16,
    cur: u16,
    highlight: u16,
) where
    W: Write,
{
    queue!(
        output,
        terminal::Clear(ClearType::CurrentLine),
        cursor::MoveLeft(prev + prompt.len() as u16)
    )
    .unwrap();
    if cur < line.len() as u16 {
        let mut chars: Vec<char> = line.chars().collect();
        let left = chars.split_off(highlight as usize + 1);
        let c = chars.split_off(chars.len() - 1);
        //        eprintln!("front={:?} c={:?} left={:?}\r", chars, c, left);
        queue!(
            output,
            style::Print(prompt.to_owned()),
            style::Print(String::from_iter(chars)),
            style::SetAttribute(style::Attribute::Bold),
            style::Print(String::from_iter(c)),
            style::SetAttribute(style::Attribute::Reset),
            style::Print(String::from_iter(left)),
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

fn find_match_paren(line: &str, index: usize) -> Option<usize> {
    let mut count = 0;
    let mut pos;
    if line.len() <= index {
        return None;
    }
    let chars: Vec<char> = line.chars().collect();
    if chars[index] == ')' {
        pos = index - 1;
        loop {
            if chars[pos] == ')' {
                count += 1;
            } else if chars[pos] == '(' {
                if count == 0 {
                    return Some(pos as usize);
                } else {
                    count -= 1;
                }
            }
            if pos == 0 {
                return None;
            }
            pos -= 1;
        }
    }
    if chars[index] == '(' {
        pos = index + 1;
        while pos < chars.len() {
            if chars[pos] == '(' {
                count += 1;
            } else if chars[pos] == ')' {
                if count == 0 {
                    return Some(pos as usize);
                } else {
                    count -= 1;
                }
            }
            pos += 1;
        }
    }
    None
}

fn print_result<W>(output: &mut W, env: &mut Env, node: Node)
where
    W: Write,
{
    match node {
        Node::Num(n) => {
            result_print(
                output,
                format!("{}\r\n", output_format_num(env, n)).as_str(),
            );
        }
        Node::FNum(f) => {
            result_print(
                output,
                format!("{}\r\n", output_format_float(env, f)).as_str(),
            );
        }
        Node::CNum(c) => {
            result_print(output, format!("{}\r\n", c).as_str());
        }
        Node::Command(cmd, params, result) => {
            error_print(output, format!("{}\r\n", result).as_str());
            if cmd == Token::Ident("history".to_owned()) && !params.is_empty() {
                do_line(output, env, &result)
            }
        }
        Node::None => {}
        _ => {
            error_print(
                output,
                format!("eval error: Unexpected eval result {:?}\r\n", node).as_str(),
            );
        }
    }
}

fn do_line<W>(output: &mut W, env: &mut Env, line: &str)
where
    W: Write,
{
    match lexer(line.to_owned()) {
        Ok(v) => {
            if v.is_empty() {
                return;
            }
            match parse(env, &v) {
                Ok(node) => match eval_top(env, &node) {
                    Ok(node) => {
                        print_result(output, env, node);
                    }
                    Err(e) => {
                        error_print(output, format!("{}\r\n", e).as_str());
                    }
                },
                Err(e) => {
                    error_print(output, format!("{}\r\n", e).as_str());
                }
            }
        }
        Err(e) => {
            error_print(output, format!("{}\r\n", e).as_str());
        }
    }
}

pub fn readline(env: &mut Env) {
    let mut line = String::new();
    let mut cur_x: u16 = 0;
    let mut prev_cur_x: u16;

    enable_raw_mode().unwrap();
    let mut stdout = stdout();

    // goto raw mode
    write!(stdout, "Ctrl-c or \"exit\" to exit\r\n").unwrap();
    write!(stdout, "rc> ").unwrap();
    stdout.flush().unwrap();

    loop {
        let event = read().unwrap();
        // println!("Event::{:?}\r", event);

        if let Event::Key(keyev) = event {
            // print!("keyev={:?}\r\n", keyev);
            if keyev.modifiers == KeyModifiers::CONTROL && keyev.code == KeyCode::Char('c') {
                write!(stdout, "\r\n").unwrap();
                save_history(&env);
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
                    if let Some(c) = find_match_paren(&line, cur_x as usize) {
                        // eprintln!("highlight {}\r", c);
                        redraw_highlight(&mut stdout, "rc> ", &line, prev_cur_x, cur_x, c as u16);
                    } else {
                        redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                    }
                }
                KeyCode::Right => {
                    prev_cur_x = cur_x;
                    cur_x = do_right(&mut line, prev_cur_x);
                    if let Some(c) = find_match_paren(&line, cur_x as usize) {
                        // eprintln!("highlight {}\r", c);
                        redraw_highlight(&mut stdout, "rc> ", &line, prev_cur_x, cur_x, c as u16);
                    } else {
                        redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                    }
                }
                KeyCode::Up => {
                    if env.history_index > 0 {
                        env.history_index -= 1;
                        if env.history_index < env.history.len() {
                            prev_cur_x = line.len() as u16;
                            line = env.history[env.history_index].clone();
                            cur_x = line.len() as u16;
                            redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                        }
                    }
                }
                KeyCode::Down => {
                    env.history_index += 1;
                    prev_cur_x = line.len() as u16;
                    if env.history_index < env.history.len() {
                        line = env.history[env.history_index].clone();
                    } else {
                        line = String::new();
                    }
                    cur_x = line.len() as u16;
                    redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                }
                KeyCode::Enter => {
                    if !line.is_empty() && line.find("history") == None && line.find("exit") == None
                    {
                        env.history.push(line.clone());
                        env.history_index = env.history.len();
                    }
                    write!(stdout, "\r\n").unwrap();
                    do_line(&mut stdout, env, &line);
                    line.clear();
                    cur_x = 0;
                    prev_cur_x = cur_x;
                    redraw(&mut stdout, "rc> ", &line, prev_cur_x, cur_x);
                }
                KeyCode::Char(c) => {
                    prev_cur_x = cur_x;
                    cur_x = do_insert(&mut line, cur_x, c);
                    if c == ')' {
                        if let Some(i) = find_match_paren(&line, cur_x as usize - 1) {
                            redraw_highlight(
                                &mut stdout,
                                "rc> ",
                                &line,
                                prev_cur_x,
                                prev_cur_x,
                                i as u16,
                            );
                            thread::sleep(time::Duration::from_millis(200));
                        }
                    }
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

    #[test]
    fn test_paren() {
        let line = String::from("()");
        let ret = find_match_paren(&line, 0);
        assert_eq!(ret, Some(1));
        let line = String::from("()");
        let ret = find_match_paren(&line, 1);
        assert_eq!(ret, Some(0));
        let line = String::from("(())");
        let ret = find_match_paren(&line, 0);
        assert_eq!(ret, Some(3));
        let line = String::from("(())");
        let ret = find_match_paren(&line, 3);
        assert_eq!(ret, Some(0));
        let line = String::from("(a(b)c(d)e)");
        let ret = find_match_paren(&line, 10);
        assert_eq!(ret, Some(0));
    }
}
