use super::*;
use std::collections::HashMap;
use std::str;

pub type TypeFn = fn(&mut Env, &[Node]) -> Node;
pub type TypeCmd = fn(&mut Env, &[Token]) -> String;

#[derive(Debug, Clone)]
pub enum FloatFormat {
    Fix,
    Sci,
    Eng,
}

#[derive(Clone)]
pub struct Env<'a> {
    pub constant: HashMap<&'a str, Node>,
    pub variable: HashMap<String, Node>,
    pub func: HashMap<&'a str, (TypeFn, usize)>, // (function pointer, arg num: 0=variable)
    pub user_func: HashMap<String, Vec<Token>>,  // user defined function
    pub cmd: HashMap<&'a str, (TypeCmd, usize, &'a str)>, // (function pointer, arg num: 0=variable)
    pub debug: bool,
    pub output_radix: u8,
    pub separate_digit: usize,
    pub float_format: FloatFormat,
}

// Implement of functions.

fn impl_sin(_env: &mut Env, arg: &[Node]) -> Node {
    if let Node::Num(n) = arg[0] {
        Node::FNum((n as f64).sin())
    } else if let Node::FNum(f) = arg[0] {
        Node::FNum(f.sin())
    } else if let Node::CNum(c) = arg[0] {
        Node::CNum(c.sin())
    } else {
        Node::None
    }
}

fn impl_cos(_env: &mut Env, arg: &[Node]) -> Node {
    if let Node::Num(n) = arg[0] {
        Node::FNum((n as f64).cos())
    } else if let Node::FNum(f) = arg[0] {
        Node::FNum(f.cos())
    } else if let Node::CNum(c) = arg[0] {
        Node::CNum(c.cos())
    } else {
        Node::None
    }
}

fn impl_exp(_env: &mut Env, arg: &[Node]) -> Node {
    Node::BinOp(
        Token::Op(TokenOp::Caret),
        Box::new(Node::FNum(std::f64::consts::E)),
        Box::new(arg[0].clone()),
    )
}

fn impl_abs(_env: &mut Env, arg: &[Node]) -> Node {
    if let Node::Num(n) = arg[0] {
        Node::FNum((n as f64).abs())
    } else if let Node::FNum(f) = arg[0] {
        Node::FNum(f.abs())
    } else if let Node::CNum(c) = &arg[0] {
        Node::FNum(c.norm())
    } else {
        Node::None
    }
}

fn impl_arg(_env: &mut Env, arg: &[Node]) -> Node {
    if let Node::Num(_) = arg[0] {
        Node::FNum(0.0)
    } else if let Node::FNum(_) = arg[0] {
        Node::FNum(0.0)
    } else if let Node::CNum(c) = &arg[0] {
        Node::FNum(c.arg())
    } else {
        Node::None
    }
}

fn impl_sqrt(_env: &mut Env, arg: &[Node]) -> Node {
    Node::BinOp(
        Token::Op(TokenOp::Caret),
        Box::new(arg[0].clone()),
        Box::new(Node::FNum(0.5)),
    )
}

fn impl_max(env: &mut Env, arg: &[Node]) -> Node {
    if arg.is_empty() {
        return Node::FNum(0.0);
    }
    let mut max = std::f64::MIN;
    for i in arg {
        if let Ok(val) = eval_fvalue(env, &i) {
            if max < val {
                max = val;
            }
        } else {
            return Node::FNum(0.0);
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
        if let Ok(val) = eval_fvalue(env, &i) {
            sum += val;
        }
    }
    Node::FNum(sum / arg.len() as f64)
}

fn impl_e12(_env: &mut Env, arg: &[Node]) -> Node {
    let input;
    if let Node::Num(n) = arg[0] {
        input = n as f64;
    } else if let Node::FNum(f) = arg[0] {
        input = f;
    } else {
        return Node::None;
    }
    // 1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2
    let mut mantissa = input;
    let mut exponent = 1.0;
    let ret: f64;
    while mantissa > 10.0 {
        mantissa /= 10.0;
        exponent *= 10.0;
    }
    while mantissa < 1.0 {
        mantissa *= 10.0;
        exponent /= 10.0;
    }
    if mantissa < (1.0 + 1.2) / 2.0 {
        ret = 1.0 * exponent;
    } else if mantissa < (1.2 + 1.5) / 2.0 {
        ret = 1.2 * exponent;
    } else if mantissa < (1.5 + 1.8) / 2.0 {
        ret = 1.5 * exponent;
    } else if mantissa < (1.8 + 2.2) / 2.0 {
        ret = 1.8 * exponent;
    } else if mantissa < (2.2 + 2.7) / 2.0 {
        ret = 2.2 * exponent;
    } else if mantissa < (2.7 + 3.3) / 2.0 {
        ret = 2.7 * exponent;
    } else if mantissa < (3.3 + 3.9) / 2.0 {
        ret = 3.3 * exponent;
    } else if mantissa < (3.9 + 4.7) / 2.0 {
        ret = 3.9 * exponent;
    } else if mantissa < (4.7 + 5.6) / 2.0 {
        ret = 4.7 * exponent;
    } else if mantissa < (5.6 + 6.8) / 2.0 {
        ret = 5.6 * exponent;
    } else if mantissa < (6.8 + 8.2) / 2.0 {
        ret = 6.8 * exponent;
    } else if mantissa < (8.2 + 10.0) / 2.0 {
        ret = 8.2 * exponent;
    } else {
        ret = 10.0 * exponent;
    }
    Node::FNum(ret)
}

// Implement of commands.
fn impl_output_format(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_output_format {:?}\r", arg);
    }
    if arg.is_empty() {
        return format!(
            "format(radix = {}, separate = {})",
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
                } else if id == "sci" {
                    env.float_format = FloatFormat::Sci;
                } else if id == "eng" {
                    env.float_format = FloatFormat::Eng;
                } else if id == "fix" {
                    env.float_format = FloatFormat::Fix;
                } else {
                }
            }
            _ => {}
        }
    }
    format!(
        "format(radix = {}, separate = {}, float = {:?})",
        env.output_radix, env.separate_digit, env.float_format,
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

pub fn output_format_float(env: &mut Env, f: f64) -> String {
    match env.float_format {
        FloatFormat::Fix => {
            format!("{}", f)
        }
        FloatFormat::Sci => {
            let mut exponent = 0;
            let mut mantissa = f;
            while mantissa >= 10.0 {
                mantissa /= 10.0;
                exponent += 1;
            }
            while mantissa < 1.0 {
                mantissa *= 10.0;
                exponent -= 1;
            }
            format!("{}e{}", mantissa, exponent)
        }
        FloatFormat::Eng => {
            let mut exponent = 0;
            let mut mantissa = f;
            while mantissa >= 1000.0 {
                mantissa /= 1000.0;
                exponent += 3;
            }
            while mantissa < 1.0 {
                mantissa *= 1000.0;
                exponent -= 3;
            }
            if exponent == 0 {
                format!("{}", mantissa)
            } else if exponent == 3 {
                format!("{}k", mantissa)
            } else if exponent == 6 {
                format!("{}M", mantissa)
            } else if exponent == 9 {
                format!("{}G", mantissa)
            } else if exponent == 12 {
                format!("{}T", mantissa)
            } else if exponent == -3 {
                format!("{}m", mantissa)
            } else if exponent == -6 {
                format!("{}u", mantissa)
            } else if exponent == -9 {
                format!("{}n", mantissa)
            } else if exponent == -12 {
                format!("{}p", mantissa)
            } else {
                format!("{}e{}", mantissa, exponent)
            }
        }
    }
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
            } else if id == "off" || id == "false" {
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

fn impl_defun(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_defun {:?}\r", arg);
    }
    if let Token::Ident(id) = &arg[0] {
        let mut implement = Vec::new();
        for i in arg {
            implement.push((*i).clone());
        }
        implement.remove(0);
        env.new_user_func((*id).to_string(), &implement);
    }
    String::from("")
}

fn print_var(env: &mut Env, key: &str, n: &Node) -> String {
    if let Ok(n) = eval(env, n) {
        match n {
            Node::Num(_) => {
                if let Ok(value) = eval_fvalue(env, &n) {
                    return format!("{} = {}\r\n", key, value);
                }
            }
            Node::FNum(_) => {
                if let Ok(value) = eval_fvalue(env, &n) {
                    return format!("{} = {}\r\n", key, value);
                }
            }
            Node::CNum(_) => {
                if let Ok(value) = eval_cvalue(env, &n) {
                    return format!("{} = {}\r\n", key, value);
                }
            }
            _ => {}
        }
    }
    String::new()
}

fn impl_constant(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_constant {:?}\r", arg);
    }
    let mut ret = String::new();
    for (key, node) in env.clone().constant.iter() {
        ret.push_str(print_var(env, key, node).as_str());
    }
    ret
}

fn impl_variable(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_variable {:?}\r", arg);
    }
    let mut ret = String::new();
    for (key, node) in env.clone().variable.iter() {
        ret.push_str(print_var(env, key, node).as_str());
    }
    ret
}

fn impl_func(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_func {:?}\r", arg);
    }
    let mut ret = String::new();
    for key in env.func.keys() {
        ret.push_str(format!("{}\r\n", key).as_str());
    }
    ret
}

fn impl_user_func(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_user_func {:?}\r", arg);
    }
    let mut ret = String::new();
    for key in env.user_func.keys() {
        ret.push_str(format!("{}\r\n", key).as_str());
    }
    ret
}

fn impl_cmd(env: &mut Env, arg: &[Token]) -> String {
    if env.is_debug() {
        eprintln!("impl_cmd {:?}\r", arg);
    }
    let mut ret = String::new();
    for (key, val) in env.cmd.iter() {
        ret.push_str(format!("{} : {}\r\n", key, val.2).as_str());
    }
    ret
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            constant: HashMap::new(),
            variable: HashMap::new(),
            func: HashMap::new(),
            user_func: HashMap::new(),
            cmd: HashMap::new(),
            debug: false,
            output_radix: 10,
            separate_digit: 0,
            float_format: FloatFormat::Fix,
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
        self.func.insert("cos", (impl_cos as TypeFn, 1));
        self.func.insert("exp", (impl_exp as TypeFn, 1));
        self.func.insert("abs", (impl_abs as TypeFn, 1));
        self.func.insert("arg", (impl_arg as TypeFn, 1));
        self.func.insert("max", (impl_max as TypeFn, 0));
        self.func.insert("ave", (impl_ave as TypeFn, 0));
        self.func.insert("sqrt", (impl_sqrt as TypeFn, 1));
        self.func.insert("E12", (impl_e12 as TypeFn, 1));
        self.cmd.insert(
            "format",
            (impl_output_format as TypeCmd, 0, "set output format"),
        );
        self.cmd
            .insert("debug", (impl_debug as TypeCmd, 1, "set/reset debug mode"));
        self.cmd
            .insert("exit", (impl_exit as TypeCmd, 0, "exit REPL"));
        self.cmd
            .insert("defun", (impl_defun as TypeCmd, 0, "define user function"));
        self.cmd
            .insert("constant", (impl_constant as TypeCmd, 0, "list constants"));
        self.cmd.insert(
            "variable",
            (impl_variable as TypeCmd, 0, "list user defined variables"),
        );
        self.cmd
            .insert("func", (impl_func as TypeCmd, 0, "list functions"));
        self.cmd.insert(
            "user_func",
            (impl_user_func as TypeCmd, 0, "list user defined functions"),
        );
        self.cmd
            .insert("cmd", (impl_cmd as TypeCmd, 0, "list commands"));

        self.new_variable("ans".to_string());
    }

    pub fn is_const(&self, key: &str) -> Option<Node> {
        match self.constant.get(key) {
            Some(f) => Some(f.clone()),
            None => None,
        }
    }

    pub fn is_variable(&self, key: &str) -> Option<Node> {
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

    pub fn is_func(&self, key: &str) -> Option<(TypeFn, usize)> {
        match self.func.get(key) {
            Some(&f) => Some(f),
            None => None,
        }
    }

    pub fn is_cmd(&self, key: &str) -> Option<(TypeCmd, usize, &str)> {
        match self.cmd.get(key) {
            Some(&f) => Some(f),
            None => None,
        }
    }

    fn new_user_func(&mut self, key: String, arg: &[Token]) {
        self.user_func.insert(key, arg.to_vec());
    }

    pub fn is_user_func(&self, key: String) -> Option<Vec<Token>> {
        match self.user_func.get(&key) {
            Some(v) => Some(v.clone()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_num() {
        let mut env = Env::new();
        assert_eq!(output_format_num(&mut env, 1), "1".to_string());
        env.separate_digit = 4;
        assert_eq!(
            output_format_num(&mut env, 12345678),
            "1234_5678".to_string()
        );
    }
    #[test]
    fn test_format_float() {
        let mut env = Env::new();
        assert_eq!(output_format_float(&mut env, 1.23), "1.23".to_string());
        env.float_format = FloatFormat::Sci;
        assert_eq!(output_format_float(&mut env, 1e10), "1e10".to_string());
        env.float_format = FloatFormat::Eng;
        assert_eq!(output_format_float(&mut env, 1e10), "10G".to_string());
    }
}
