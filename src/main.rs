use getopts::Options;
use std::env;

use rc::*;

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
    opts.optflag("", "test", "run built-in test");
    opts.optopt("s", "script", "run script", "FILE");

    let matches = opts.parse(&args[1..]).unwrap();
    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }
    let mut env = Env::new();
    env.built_in();

    if matches.opt_present("debug") {
        env.set_debug(true);
    }
    if matches.opt_present("test") {
        run_test(&mut env);
        std::process::exit(0);
    }
    readline(&mut env);
}

// TODO: script mode (input from FILE or stdin(--))
// TODO: script mode test(diff)
// TODO: online help
// TODO: syntax error handling
