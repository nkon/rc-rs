use getopts::Options;
use rc::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

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

    match opts.parse(&args[1..]) {
        Ok(matches) => {
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
            if matches.opt_present("script") {
                let mut script = String::new();
                if let Some(filename) = matches.opt_str("script") {
                    if filename == "--" {
                        std::io::stdin()
                            .read_to_string(&mut script)
                            .expect("something went wrong reading stdin");
                    } else {
                        let fname = filename.clone();
                        let mut f = File::open(filename)
                            .expect(format!("file not found: {}", fname).as_str());
                        f.read_to_string(&mut script)
                            .expect("something went wrong reading the file");
                    }
                    run_script(&mut env, &script);
                } else {
                    eprintln!("-s FILE required");
                    std::process::exit(1);
                }
            }
            readline(&mut env);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(0);
        }
    }
}

// TODO: script mode (input from FILE or stdin(--))
// TODO: script and comment
// TODO: script mode test(diff)
// TODO: load history, history command
// TODO: online help
// TODO: initial file `~.rc-rs`
// TODO: complex number and functions
