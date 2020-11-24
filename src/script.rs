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
                        Ok(node) => {
                            let result = eval(env, &node);
                            match result.ty {
                                NodeType::Num => {
                                    println!("{}", result.value);
                                }
                                NodeType::FNum => {
                                    println!("{}", result.fvalue);
                                }
                                _ => {
                                    println!("eval error");
                                }
                            }
                        }
                        Err(e) => {
                            println!("parse error: {}", e);
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
