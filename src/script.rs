use std::fs::File;
use std::io::BufRead;
use std::io::Write;

use super::*;

/// read one line -> parse and evaluate. return result as String.
pub fn do_script(env: &mut Env, line: &str) -> Result<String, MyError> {
    if env.debug {
        eprint!("{}", line);
    }
    let tokens = lexer(line.to_owned())?;
    if tokens.is_empty() {
        return Ok("".to_owned());
    }
    let node = parse(env, &tokens)?;
    match eval_top(env, &node)? {
        Node::Num(n) => Ok(output_format_num(env, n)),
        Node::FNum(f) => Ok(format!("{}", f)),
        Node::CNum(c) => Ok(format!("{}", c)),
        // Node::Command(_cmd, _params, result) => Ok(format!("{}\r\n", result)),
        Node::Command(_cmd, _params, _result) => Ok("".to_owned()),
        Node::None => Ok("".to_owned()),
        _ => Err(MyError::EvalError("_".to_owned())),
    }
}

/// read from BufRead stream -> parse and evaluate all lines.
/// print result to stdout. error to exit.
pub fn run_script(env: &mut Env, stream: &mut dyn BufRead) {
    let mut line = String::new();
    loop {
        match stream.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                match do_script(env, &line) {
                    Ok(str_result) => {
                        println!("{}", str_result);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(0);
                    }
                }
                line.clear();
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(0);
            }
        }
    }
}

/// similar to `run_script()`.
/// execute silent. error to break.
pub fn run_rc(env: &mut Env, stream: &mut dyn BufRead) {
    let mut line = String::new();
    loop {
        match stream.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                do_script(env, &line)
                    .map_err(|e| eprintln!("{}", e))
                    .unwrap();
                line.clear();
            }
            Err(_e) => {
                break;
            }
        }
    }
}

pub fn run_history(env: &mut Env, stream: &mut dyn BufRead) {
    let mut line = String::new();
    loop {
        match stream.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                line.pop();
                if line.is_empty() {
                    continue;
                }
                env.history.push(line.clone());
                env.history_index += 1;
                if do_script(env, &line).is_ok() {}
                line.clear();
            }
            Err(_e) => {
                break;
            }
        }
    }
}

pub fn save_history(env: &Env) {
    let mut history = env.history.clone();
    if env.history_max == 0 {
        return;
    }

    if history.len() > env.history_max {
        history = history.split_off(history.len() - env.history_max);
    }

    if let Some(mut history_file_path) = dirs::home_dir() {
        history_file_path.push(".rc.history");
        let mut file = File::create(history_file_path).expect("file create");
        while !history.is_empty() {
            file.write_all(format!("{}\n", history.remove(0)).as_bytes())
                .unwrap();
        }
        file.flush().unwrap();
    }
}
