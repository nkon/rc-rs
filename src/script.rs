use std::io::BufRead;

use super::*;

fn do_script(env: &mut Env, line: &str) -> Result<String, MyError> {
    if env.debug {
        eprint!("{}", line);
    }
    let tokens = lexer(line.to_string())?;
    if tokens.is_empty() {
        return Ok("".to_owned());
    }
    let node = parse(env, &tokens)?;
    match eval(env, &node)? {
        Node::Num(n) => Ok(output_format_num(env, n)),
        Node::FNum(f) => Ok(format!("{}", f)),
        Node::CNum(c) => Ok(format!("{}", c)),
        // Node::Command(_cmd, _params, result) => Ok(format!("{}\r\n", result)),
        Node::Command(_cmd, _params, _result) => Ok("".to_string()),
        Node::None => Ok("".to_string()),
        _ => Err(MyError::EvalError("_".to_owned())),
    }
}

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
