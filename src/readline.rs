use std::io;
use std::io::Write;

use super::*;

pub fn readline() -> String {
    let mut line = String::new();
    loop {
        print!("rc> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        // println!("{}", line);
        match lexer(line.clone()) {
            Ok(v) => {println!("{:?}", eval(&parse(&v)));}
            Err(e) => {println!("{}", e);}
        }
        line.clear();
    }
}
