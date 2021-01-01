use getopts::Options;
use rc::*;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path;

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "rc: CUI calculator\nUsage: {} [options] [expression]",
        program
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help");
    opts.optflag("d", "debug", "debug mode");
    opts.optflag("", "test", "run built-in test");
    opts.optopt("i", "init", "initialize file path", "rc_file");
    opts.optmulti("s", "script", "run script", "script_file");
    opts.optflag("v", "version", "version");

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

            let mut rc_file_path = path::PathBuf::new();
            if let Some(home_dir) = dirs::home_dir() {
                rc_file_path = home_dir;
                rc_file_path.push(".rc_rc");
            }
            // overwritten `~/.rc_rc` by '-i' option
            if let Some(rc_file_str) = matches.opt_str("i") {
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
                } else {
                    eprintln!("-s FILE required");
                    std::process::exit(1);
                }
            }

            if env.history_max > 0 {
                if let Some(mut history_file_path) = dirs::home_dir() {
                    history_file_path.push(".rc.history");
                    env.history_path = history_file_path.clone();
                    eprintln!("history_path={:?}", env.history_path);
                    if let Ok(file) = File::open(history_file_path) {
                        run_history(&mut env, &mut BufReader::new(file));
                    }
                }
            }

            if matches.free.is_empty() {
                readline(&mut env);
            } else {
                let mut expression = String::new();
                for item in matches.free {
                    expression.push_str(item.as_str());
                    expression.push(' ');
                }
                println!("{}", do_script(&mut env, &expression).unwrap());
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(0);
        }
    }
}

// TODO: load command
// TODO: online help, refer `HELP.md`.
// TODO: map -> graph
