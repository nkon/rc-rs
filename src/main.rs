use getopts::Options;
use rc::*;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("rc: CUI calculator\nUsage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help");
    opts.optflag("d", "debug", "debug mode");
    opts.optflag("", "test", "run built-in test");
    opts.optopt("r", "init_file", "rc file path", "rc_file");
    opts.optmulti("s", "script", "run script", "FILE");
    opts.optflag("v", "version", "version");

    let mut rc_file_path = path::PathBuf::new();
    if let Some(mut home_dir) = dirs::home_dir() {
        home_dir.push(".rc_rc");
        rc_file_path = home_dir;
    }

    match opts.parse(&args[1..]) {
        Ok(matches) => {
            if matches.opt_present("h") {
                print_usage(&program, opts);
                std::process::exit(0);
            }

            if matches.opt_present("v") {
                let version = env!("CARGO_PKG_VERSION");
                let git_commit_hash = include!("git_commit_hash.txt");
                println!("{} {}-{}", program, version, git_commit_hash);
                std::process::exit(0);
            }

            let mut env = Env::new();
            env.built_in();

            // overwritten by '-r' option
            if let Some(rc_file_str) = matches.opt_str("r") {
                rc_file_path = path::Path::new(&rc_file_str).to_path_buf();
                if !rc_file_path.exists() {
                    eprintln!("file not found {}", rc_file_path.to_str().unwrap());
                    std::process::exit(1);
                }
            }
            if rc_file_path.exists() {
                if let Ok(file) = File::open(rc_file_path) {
                    run_rc(&mut env, &mut BufReader::new(file));
                }
            }

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
                    if filenames[0] == "-" {
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

// TODO: load history, history command
// TODO: load command
// TODO: online help
// TODO: complex functions
// TODO: improve num handling(Num, FNum, CNum)
