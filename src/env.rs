use super::*;
use std::collections::HashMap;
use std::str;

pub type TypeFn = fn(&mut Env, &[Node]) -> Node;
pub type TypeCmd = fn(&mut Env, &[Token]) -> String;

pub struct Env<'a> {
    pub constant: HashMap<&'a str, Node>,
    pub variable: HashMap<String, Node>,
    pub func: HashMap<&'a str, (TypeFn, usize)>, // (function pointer, arg num: 0=variable)
    pub cmd: HashMap<&'a str, (TypeCmd, usize)>, // (function pointer, arg num: 0=variable)
    pub debug: bool,
    pub output_radix: u8,
    pub separate_digit: usize,
}
// TODO: Output floating format: fix=12345.6, sci=1.23456e4, eng=12.3456e3

// Implement of functions.
fn impl_sin(env: &mut Env, arg: &[Node]) -> Node {
    Node::FNum(eval_fvalue(env, &arg[0]).sin())
}

fn impl_exp(_env: &mut Env, arg: &[Node]) -> Node {
    Node::BinOp(
        Token::Op(TokenOp::Hat),
        Box::new(Node::FNum(std::f64::consts::E)),
        Box::new(arg[0].clone()),
    )
}

fn impl_abs(env: &mut Env, arg: &[Node]) -> Node {
    Node::FNum(eval_fvalue(env, &arg[0]).abs())
}

fn impl_max(env: &mut Env, arg: &[Node]) -> Node {
    if arg.is_empty() {
        return Node::FNum(0.0);
    }
    let mut max = eval_fvalue(env, &arg[0]);
    for i in arg {
        if max < eval_fvalue(env, &i) {
            max = eval_fvalue(env, &i);
        }
    }
    Node::FNum(max)
}

fn impl_ave(env: &mut Env, arg: &[Node]) -> Node {
    if arg.is_empty() {
        return Node::FNum(0.0);
    }
    let mut sum: f64 = 0.0;
    for i in arg {
        sum += eval_fvalue(env, &i);
    }
    Node::FNum(sum / arg.len() as f64)
}

// Implement of commands.
fn impl_output_format(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_output_format {:?}\r", arg);
    }
    if arg.is_empty() {
        return format!(
            "output_format(radix = {}, separate = {})",
            env.output_radix, env.separate_digit
        );
    }
    for a in arg {
        match a {
            Token::Num(2) => {
                env.output_radix = 2;
            }
            Token::Num(10) => {
                env.output_radix = 10;
            }
            Token::Num(16) => {
                env.output_radix = 16;
            }
            Token::Ident(id) => {
                if id == "radix2" || id == "binary" {
                    env.output_radix = 2;
                } else if id == "radix10" || id == "decimal" {
                    env.output_radix = 10;
                } else if id == "radix16" || id == "hexadecimal" {
                    env.output_radix = 2;
                } else if id == "sep3" {
                    env.separate_digit = 3;
                } else if id == "sep4" {
                    env.separate_digit = 4;
                } else if id == "sep0" {
                    env.separate_digit = 0;
                } else {
                }
            }
            _ => {}
        }
    }
    format!(
        "output_format(radix = {}, separate = {})",
        env.output_radix, env.separate_digit
    )
}

fn separate_digit(s: String, sep: &str, n: usize) -> String {
    let bytes: Vec<_> = s.bytes().rev().collect();
    let chunks: Vec<_> = bytes
        .chunks(n)
        .map(|chunk| str::from_utf8(chunk).unwrap())
        .collect();
    let result: Vec<_> = chunks.join(sep).bytes().rev().collect();
    String::from_utf8(result).unwrap()
}

pub fn output_format_num(env: &mut Env, n: i128) -> String {
    let mut num_string: String;

    match env.output_radix {
        2 => {
            num_string = format!("{:b}", n);
        }
        10 => {
            num_string = format!("{}", n);
        }
        16 => {
            num_string = format!("{:x}", n);
        }
        _ => {
            num_string = format!("{:?}", n);
        }
    }

    if env.separate_digit != 0 {
        num_string = separate_digit(num_string, "_", env.separate_digit);
    }

    match env.output_radix {
        2 => {
            num_string = format!("0b{}", num_string);
        }
        16 => {
            num_string = format!("0x{}", num_string);
        }
        _ => {}
    }

    num_string
}

fn impl_debug(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_debug {:?}\r", arg);
    }
    if arg.is_empty() {
        return format!("debug({})", env.debug);
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
    format!("debug({})", env.debug)
}

fn impl_exit(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_exit {:?}\r", arg);
    }
    std::process::exit(0);
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            constant: HashMap::new(),
            variable: HashMap::new(),
            func: HashMap::new(),
            cmd: HashMap::new(),
            debug: false,
            output_radix: 10,
            separate_digit: 0,
        }
    }

    pub fn built_in(&mut self) {
        self.constant.insert("pi", Node::FNum(std::f64::consts::PI));
        self.constant.insert("e", Node::FNum(std::f64::consts::E));
        self.constant.insert("eps", Node::FNum(std::f64::EPSILON));
        self.constant
            .insert("i", Node::CNum(Complex64::new(0.0, 1.0)));
        self.constant
            .insert("j", Node::CNum(Complex64::new(0.0, 1.0)));
        self.func.insert("sin", (impl_sin as TypeFn, 1));
        self.func.insert("exp", (impl_exp as TypeFn, 1));
        self.func.insert("abs", (impl_abs as TypeFn, 1));
        self.func.insert("max", (impl_max as TypeFn, 0));
        self.func.insert("ave", (impl_ave as TypeFn, 0));
        self.cmd
            .insert("output_format", (impl_output_format as TypeCmd, 0));
        self.cmd.insert("debug", (impl_debug as TypeCmd, 1));
        self.cmd.insert("exit", (impl_exit as TypeCmd, 0));
    }

    pub fn is_const(&mut self, key: &str) -> Option<Node> {
        match self.constant.get(key) {
            Some(f) => Some(f.clone()),
            None => None,
        }
    }

    pub fn is_variable(&mut self, key: &str) -> Option<Node> {
        match self.variable.get(key) {
            Some(f) => Some(f.clone()),
            None => None,
        }
    }

    pub fn new_variable(&mut self, key: String) {
        self.variable.insert(key, Node::None);
    }

    pub fn set_variable(&mut self, key: String, value: Node) -> Result<(), MyError> {
        if self.is_variable(&key).is_some() {
            self.variable.insert(key, value);
        } else {
            return Err(MyError::EvalError(format!("can not assign to {}", key)));
        }
        Ok(())
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
