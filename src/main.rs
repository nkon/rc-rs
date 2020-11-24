use getopts::Options;
use rc::*;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("rc: CUI calclator\nUsage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help");
    opts.optflag("d", "debug", "debug mode");
    opts.optflag("", "test", "run built-in test");
    opts.optmulti("s", "script", "run script", "FILE");

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
                let filenames = matches.opt_strs("script");
                if !filenames.is_empty() {
                    if filenames[0] == "--" {
                        let mut buf_file = BufReader::new(std::io::stdin());
                        run_script(&mut env, &mut buf_file);
                    } else {
                        filenames
                            .iter()
                            .for_each(|filename| match File::open(filename) {
                                Ok(file) => {
                                    run_script(&mut env, &mut BufReader::new(file));
                                }
                                Err(e) => println!("{}: {}", filename, e),
                            })
                    }
                    std::process::exit(0);
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

// TODO: script and comment
// TODO: load history, history command
// TODO: online help
// TODO: initial file `~.rc-rs`
// TODO: complex number and functions
