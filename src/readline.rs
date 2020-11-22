use std::io;
use std::io::Write;

use super::*;

pub fn readline(env: &mut Env) -> String {
    let mut line = String::new();
    loop {
        print!("rc> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        // println!("{}", line);
        match lexer(line.clone()) {
            Ok(v) => {
                let node = parse(env, &v);
                println!("{:?}", eval(env, &node));
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        line.clear();
    }
}
