use super::*;
use std::collections::HashMap;

pub type TypeFn = fn(&mut Env, &[Node]) -> f64;
pub type TypeCmd = fn(&mut Env, &[Token]);

pub struct Env<'a> {
    pub constant: HashMap<&'a str, f64>,
    pub func: HashMap<&'a str, (TypeFn, usize)>, // (function pointer, arg num: 0=variable)
    pub cmd: HashMap<&'a str, (TypeCmd, usize)>, // (function pointer, arg num: 0=variable)
    pub debug: bool,
    pub output_base: u8,
}

// Impliment of functions.
fn impl_sin(env: &mut Env, arg: &[Node]) -> f64 {
    eval_fvalue(env, &arg[0]).sin()
}

fn impl_abs(env: &mut Env, arg: &[Node]) -> f64 {
    eval_fvalue(env, &arg[0]).abs()
}

fn impl_max(env: &mut Env, arg: &[Node]) -> f64 {
    if arg.is_empty() {
        return 0.0;
    }
    let mut max = eval_fvalue(env, &arg[0]);
    for i in arg {
        if max < eval_fvalue(env, &i) {
            max = eval_fvalue(env, &i);
        }
    }
    max
}

fn impl_ave(env: &mut Env, arg: &[Node]) -> f64 {
    if arg.is_empty() {
        return 0.0;
    }
    let mut sum: f64 = 0.0;
    for i in arg {
        sum += eval_fvalue(env, &i);
    }
    sum / arg.len() as f64
}

// Impliment of commands.
fn impl_output_format(env: &mut Env, arg: &[Token]) {
    if env.is_debug() {
        eprintln!("impl_output_format {:?}\r", arg);
    }
    if arg.is_empty() {
        return;
    }
    for a in arg {
        match a {
            Token::Num(2) => {
                env.output_base = 2;
            }
            Token::Num(10) => {
                env.output_base = 10;
            }
            Token::Num(16) => {
                env.output_base = 16;
            }
            _ => {}
        }
    }
}

pub fn output_format(env: &mut Env, n: i128) -> String {
    match env.output_base {
        2 => {
            format!("0b{:b}", n)
        }
        10 => {
            format!("{}", n)
        }
        16 => {
            format!("0x{:x}", n)
        }
        _ => {
            format!("{:?}", n)
        }
    }
}

fn impl_debug(env: &mut Env, arg: &[Token]) {
    if env.is_debug() {
        eprintln!("impl_debug {:?}\r", arg);
    }
    if arg.is_empty() {
        return;
    }
    match &arg[0] {
        Token::Num(0) => {
            env.debug = false;
        }
        Token::Num(1) => {
            env.debug = true;
        }
        Token::Ident(id) => {
            if id == "on" || id == "true" {
                env.debug = true;
            } else if id == "off" || id == "faluse" {
                env.debug = false;
            }
        }
        _ => {}
    }
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            constant: HashMap::new(),
            func: HashMap::new(),
            cmd: HashMap::new(),
            debug: false,
            output_base: 10,
        }
    }

    pub fn built_in(&mut self) {
        self.constant.insert("pi", std::f64::consts::PI);
        self.constant.insert("e", std::f64::consts::E);
        self.constant.insert("eps", std::f64::EPSILON);
        self.func.insert("sin", (impl_sin as TypeFn, 1));
        self.func.insert("abs", (impl_abs as TypeFn, 1));
        self.func.insert("max", (impl_max as TypeFn, 0));
        self.func.insert("ave", (impl_ave as TypeFn, 0));
        self.cmd
            .insert("output_format", (impl_output_format as TypeCmd, 0));
        self.cmd.insert("debug", (impl_debug as TypeCmd, 1));
    }

    pub fn is_const(&mut self, key: &str) -> Option<f64> {
        match self.constant.get(key) {
            Some(&f) => Some(f),
            None => None,
        }
    }

    pub fn is_func(&mut self, key: &str) -> Option<(TypeFn, usize)> {
        match self.func.get(key) {
            Some(&f) => Some(f),
            None => None,
        }
    }

    pub fn is_cmd(&mut self, key: &str) -> Option<(TypeCmd, usize)> {
        match self.cmd.get(key) {
            Some(&f) => Some(f),
            None => None,
        }
    }

    pub fn set_debug(&mut self, flag: bool) {
        self.debug = flag;
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }
}

impl Default for Env<'_> {
    fn default() -> Self {
        Self::new()
    }
}
