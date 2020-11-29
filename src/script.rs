use std::io::BufRead;

use super::*;

pub fn run_script(env: &mut Env, stream: &mut dyn BufRead) {
    let mut line = String::new();
    loop {
        match stream.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                if env.debug {
                    print!("{}", line);
                }
                if let Ok(tokens) = lexer(line.clone()) {
                    if tokens.is_empty() {
                        continue;
                    }
                    match parse(env, &tokens) {
                        Ok(node) => match eval(env, &node) {
                            Node::Num(n) => {
                                println!("{}", output_format_num(env, n));
                            }
                            Node::FNum(f) => {
                                println!("{}", f);
                            }
                            _ => {
                                println!("eval error");
                            }
                        },
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }
                line.clear();
            }
            Err(e) => {
                println!("{}", e);
                break;
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
                if env.debug {
                    print!("{}", line);
                }
                if let Ok(tokens) = lexer(line.clone()) {
                    if tokens.is_empty() {
                        continue;
                    }
                    if let Ok(node) = parse(env, &tokens) {
                        eval(env, &node);
                    }
                }
            }
            Err(_e) => {
                break;
            }
        }
    }
}
