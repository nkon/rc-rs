use getopts::Options;
use std::env;

use rc::*;

fn run_test() {
    println!("lexer");
    println!("1 -> {:?}", lexer("1".to_string()));
    println!("10 1 -> {:?}", lexer("10 1".to_string()));
    println!("1+1 -> {:?}", lexer("1+1".to_string()));
    println!("1-1 -> {:?}", lexer("1-1".to_string()));
    println!("-1 -> {:?}", lexer("-1".to_string()));
    println!("+-*/%()^100 -> {:?}", lexer("+-*/%()^-100".to_string()));
    println!("");
    println!("parser");
    println!("1 -> {:?}", parse(&lexer("1".to_string())));
    println!("0 -> {:?}", parse(&lexer("0".to_string())));
    println!("-1 -> {:?}", parse(&lexer("-1".to_string())));
    println!(
        "9223372036854775807 -> {:?}",
        parse(&lexer("9223372036854775807".to_string()))
    );
    println!(
        "-9223372036854775808 -> {:?}",
        parse(&lexer("-9223372036854775808".to_string()))
    );
    println!("1+2 -> {:?}", parse(&lexer("1+2".to_string())));
    println!("1-2 -> {:?}", parse(&lexer("1-2".to_string())));
    println!("1+-2 -> {:?}", parse(&lexer("1+-2".to_string())));
    println!("1*2 -> {:?}", parse(&lexer("1*2".to_string())));
    println!("1*2+3 -> {:?}", parse(&lexer("1*2+3".to_string())));
    println!("1+2*3 -> {:?}", parse(&lexer("1+2*3".to_string())));
    println!("1*(2+3) -> {:?}", parse(&lexer("1*(2+3)".to_string())));
    println!("(1+2)*3 -> {:?}", parse(&lexer("(1+2)*3".to_string())));
    println!("1+2+3 -> {:?}", parse(&lexer("1+2+3".to_string())));
    println!("1*2*3 -> {:?}", parse(&lexer("1*2*3".to_string())));
    println!("");
    println!("eval");
    println!("1 -> {:?}", eval(&parse(&lexer("1".to_string()))));
    println!("0 -> {:?}", eval(&parse(&lexer("0".to_string()))));
    println!("-1 -> {:?}", eval(&parse(&lexer("-1".to_string()))));
    println!(
        "9223372036854775807 -> {:?}",
        eval(&parse(&lexer("9223372036854775807".to_string())))
    );
    println!(
        "-9223372036854775807 -> {:?}",
        eval(&parse(&lexer("-9223372036854775807".to_string())))
    );
    println!("1+2 -> {:?}", eval(&parse(&lexer("1+2".to_string()))));
    println!("1-2 -> {:?}", eval(&parse(&lexer("1-2".to_string()))));
    println!("1+-2 -> {:?}", eval(&parse(&lexer("1+-2".to_string()))));
    println!("1*2 -> {:?}", eval(&parse(&lexer("1*2".to_string()))));
    println!("1*2+3 -> {:?}", eval(&parse(&lexer("1*2+3".to_string()))));
    println!("1+2*3 -> {:?}", eval(&parse(&lexer("1+2*3".to_string()))));
    println!(
        "1*(2+3) -> {:?}",
        eval(&parse(&lexer("1*(2+3)".to_string())))
    );
    println!(
        "(1+2)*3 -> {:?}",
        eval(&parse(&lexer("(1+2)*3".to_string())))
    );
    println!("1+2+3 -> {:?}", eval(&parse(&lexer("1+2+3".to_string()))));
    println!("1*2*3 -> {:?}", eval(&parse(&lexer("1*2*3".to_string()))));
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}


fn main() {


    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help");
    opts.optflag("d", "debug", "debug mode");
    opts.optflag("", "test", "run bult-in test");
    opts.optopt("s", "script", "run script", "FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }
    if matches.opt_present("test") {
        run_test();
        std::process::exit(0);
    }
}
