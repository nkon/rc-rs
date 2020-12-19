use num_complex::Complex64;
use thiserror::Error;

// use anyhow;
// TODO: use anyhow for better error handling

mod env;
mod lexer;
mod parser;
mod readline;
mod run_test;
mod script;

pub use env::*;
pub use lexer::*;
pub use parser::*;
pub use readline::readline;
pub use run_test::run_test;
pub use script::{run_rc, run_script};

#[derive(Error, Debug)]
pub enum MyError {
    #[error("lexer error: {1} {0}")]
    LexerIntError(String, std::num::ParseIntError),

    // #[error(transparent)]
    // LexerFloatError(#[from] std::num::ParseFloatError),
    #[error("lexer error: {1} {0}")]
    LexerFloatError(String, std::num::ParseFloatError),

    #[error("parser error: {0}")]
    ParseError(String),

    #[error("eval error: {0}")]
    EvalError(String),
}

// TODO: prevent panic when unexpected input.
pub fn eval_fvalue(_env: &mut Env, n: &Node) -> f64 {
    match n {
        Node::Num(n) => *n as f64,
        Node::FNum(f) => *f,
        Node::None => unreachable!(),
        _ => unreachable!(),
    }
}

pub fn eval_cvalue(_env: &mut Env, n: &Node) -> Complex64 {
    match n {
        Node::Num(n) => Complex64::new(*n as f64, 0.0),
        Node::FNum(f) => Complex64::new(*f, 0.0),
        Node::CNum(c) => *c,
        Node::None => unreachable!(),
        _ => unreachable!(),
    }
}

fn eval_const(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_const {:?}\r", n);
    }
    if let Node::Var(id) = n {
        if let Token::Ident(ident) = id {
            if let Some(constant) = env.is_const(ident.as_str()) {
                return Ok(constant);
            } else if let Some(variable) = env.is_variable(ident.as_str()) {
                return Ok(variable);
            }
        }
    }
    Err(MyError::EvalError(format!(
        "unknown constant/variable: {:?}",
        n
    )))
}

fn node_to_token(n: Node) -> Vec<Token> {
    match n {
        Node::Num(n) => vec![Token::Num(n)],
        Node::FNum(f) => vec![Token::FNum(f)],
        Node::CNum(c) => vec![
            Token::Op(TokenOp::ParenLeft),
            Token::FNum(c.re),
            Token::Op(TokenOp::Plus),
            Token::FNum(c.im),
            Token::Op(TokenOp::Mul),
            Token::Ident("i".to_owned()),
            Token::Op(TokenOp::ParenRight),
        ],
        _ => Vec::new(),
    }
}

// TODO: command does not use '(', ')'.
// TODO: refactor to separate functions.
fn eval_func(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_func {:?}\r", n);
    }
    if let Node::Func(tok, param) = n {
        if let Token::Ident(ident) = tok {
            if let Some(func_tuple) = env.is_func(ident.as_str()) {
                let mut params: Vec<Node> = Vec::new();
                for i in param {
                    let param_value = eval(env, &i)?;
                    params.push(param_value);
                }
                return Ok(func_tuple.0(env, &params));
            }
            if let Some(tokens) = env.is_user_func((*ident).clone()) {
                let mut params: Vec<Node> = Vec::new();
                for i in param {
                    let param_value = eval(env, &i)?;
                    params.push(param_value);
                }
                let mut new_tokens: Vec<Token> = Vec::new();
                for t in tokens {
                    match t {
                        Token::Ident(ref id) => {
                            if id == "_1" {
                                if params.is_empty() {
                                    return Err(MyError::EvalError(format!(
                                        "no parameter {:?}",
                                        new_tokens
                                    )));
                                }
                                new_tokens.append(&mut node_to_token(params[0].clone()));
                            } else if id == "_2" {
                                if params.len() < 2 {
                                    return Err(MyError::EvalError(format!(
                                        "no parameter {:?}",
                                        new_tokens
                                    )));
                                }
                                new_tokens.append(&mut node_to_token(params[1].clone()));
                            } else if id == "_3" {
                                if params.len() < 3 {
                                    return Err(MyError::EvalError(format!(
                                        "no parameter {:?}",
                                        new_tokens
                                    )));
                                }
                                new_tokens.append(&mut node_to_token(params[2].clone()));
                            } else if id == "_4" {
                                if params.len() < 4 {
                                    return Err(MyError::EvalError(format!(
                                        "no parameter {:?}",
                                        new_tokens
                                    )));
                                }
                                new_tokens.append(&mut node_to_token(params[3].clone()));
                            } else if id == "_5" {
                                if params.len() < 5 {
                                    return Err(MyError::EvalError(format!(
                                        "no parameter {:?}",
                                        new_tokens
                                    )));
                                }
                                new_tokens.append(&mut node_to_token(params[4].clone()));
                            } else if id == "_6" {
                                if params.len() < 6 {
                                    return Err(MyError::EvalError(format!(
                                        "no parameter {:?}",
                                        new_tokens
                                    )));
                                }
                                new_tokens.append(&mut node_to_token(params[5].clone()));
                            } else if id == "_7" {
                                if params.len() < 7 {
                                    return Err(MyError::EvalError(format!(
                                        "no parameter {:?}",
                                        new_tokens
                                    )));
                                }
                                new_tokens.append(&mut node_to_token(params[6].clone()));
                            } else if id == "_8" {
                                if params.len() < 8 {
                                    return Err(MyError::EvalError(format!(
                                        "no parameter {:?}",
                                        new_tokens
                                    )));
                                }
                                new_tokens.append(&mut node_to_token(params[7].clone()));
                            } else if id == "_9" {
                                if params.len() < 9 {
                                    return Err(MyError::EvalError(format!(
                                        "no parameter {:?}",
                                        new_tokens
                                    )));
                                }
                                new_tokens.append(&mut node_to_token(params[8].clone()));
                            } else {
                                new_tokens.push(t);
                            }
                        }
                        _ => {
                            new_tokens.push(t);
                        }
                    }
                }
                if env.is_debug() {
                    eprintln!("eval_func re-wrote tokens: {:?}\r", new_tokens);
                }
                let func_node = parse(env, &new_tokens)?;
                return eval(env, &func_node);
            }
        }
    }
    Err(MyError::EvalError(format!("unknown function: {:?}", n)))
}

fn eval_command(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_command {:?}\r", n);
    }
    if let Node::Command(tok, params, _result) = n {
        if let Token::Ident(ident) = tok {
            if let Some(cmd_tuple) = env.is_cmd(ident.as_str()) {
                let result = cmd_tuple.0(env, params);
                return Ok(Node::Command(tok.clone(), params.clone(), result));
            }
        }
    }
    Err(MyError::EvalError(format!("unknown command: {:?}", n)))
}

fn eval_assign(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_assign {:?}\r", n);
    }
    if let Node::BinOp(tok, lhs, rhs) = n {
        assert_eq!(*tok, Token::Op(TokenOp::Equal));
        match &**lhs {
            Node::Var(Token::Ident(id)) => {
                if env.is_variable(id).is_some() {
                    env.set_variable(id.clone(), (**rhs).clone())?;
                    return Ok(Node::None);
                } else {
                    return Err(MyError::EvalError(format!("Can not assign to {:?}", id)));
                }
            }
            _ => {
                return Err(MyError::EvalError(format!("'=' operator: {:?}", n)));
            }
        }
    }
    Err(MyError::EvalError(format!("'=' operator: {:?}", n)))
}

fn eval_binop(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_binop {:?}\r", n);
    }
    if let Node::BinOp(tok, lhs, rhs) = n {
        if *tok == Token::Op(TokenOp::Equal) {
            return Ok(eval_assign(env, n)?);
        }
        let lhs = eval(env, lhs)?;
        let rhs = eval(env, rhs)?;
        match tok {
            Token::Op(TokenOp::Plus) => {
                if let Node::Num(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::Num(nl + nr));
                    }
                }
                if let Node::CNum(_) = lhs {
                    return Ok(Node::CNum(eval_cvalue(env, &lhs) + eval_cvalue(env, &rhs)));
                }
                if let Node::CNum(_) = rhs {
                    return Ok(Node::CNum(eval_cvalue(env, &lhs) + eval_cvalue(env, &rhs)));
                }
                return Ok(Node::FNum(eval_fvalue(env, &lhs) + eval_fvalue(env, &rhs)));
            }
            Token::Op(TokenOp::Minus) => {
                if let Node::Num(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::Num(nl - nr));
                    }
                }
                if let Node::CNum(_) = lhs {
                    return Ok(Node::CNum(eval_cvalue(env, &lhs) - eval_cvalue(env, &rhs)));
                }
                if let Node::CNum(_) = rhs {
                    return Ok(Node::CNum(eval_cvalue(env, &lhs) - eval_cvalue(env, &rhs)));
                }
                return Ok(Node::FNum(eval_fvalue(env, &lhs) - eval_fvalue(env, &rhs)));
            }
            Token::Op(TokenOp::Mul) => {
                if let Node::Num(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::Num(nl * nr));
                    }
                }
                if let Node::CNum(_) = lhs {
                    return Ok(Node::CNum(eval_cvalue(env, &lhs) * eval_cvalue(env, &rhs)));
                }
                if let Node::CNum(_) = rhs {
                    return Ok(Node::CNum(eval_cvalue(env, &lhs) * eval_cvalue(env, &rhs)));
                }
                return Ok(Node::FNum(eval_fvalue(env, &lhs) * eval_fvalue(env, &rhs)));
            }
            Token::Op(TokenOp::Div) => {
                if let Node::Num(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::Num(nl / nr));
                    }
                }
                if let Node::CNum(_) = lhs {
                    return Ok(Node::CNum(eval_cvalue(env, &lhs) / eval_cvalue(env, &rhs)));
                }
                if let Node::CNum(_) = rhs {
                    return Ok(Node::CNum(eval_cvalue(env, &lhs) / eval_cvalue(env, &rhs)));
                }
                return Ok(Node::FNum(eval_fvalue(env, &lhs) / eval_fvalue(env, &rhs)));
            }
            Token::Op(TokenOp::Para) => {
                if let Node::CNum(_) = lhs {
                    let lhs = eval_cvalue(env, &lhs);
                    let rhs = eval_cvalue(env, &rhs);
                    return Ok(Node::CNum((lhs * rhs) / (lhs + rhs)));
                }
                if let Node::CNum(_) = rhs {
                    let lhs = eval_cvalue(env, &lhs);
                    let rhs = eval_cvalue(env, &rhs);
                    return Ok(Node::CNum((lhs * rhs) / (lhs + rhs)));
                }
                let lhs = eval_fvalue(env, &lhs);
                let rhs = eval_fvalue(env, &rhs);
                return Ok(Node::FNum((lhs * rhs) / (lhs + rhs)));
            }
            Token::Op(TokenOp::Mod) => {
                if let Node::Num(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::Num(nl % nr));
                    }
                }
                return Ok(Node::Num(0));
            }
            Token::Op(TokenOp::Hat) => {
                if let Node::Num(nr) = rhs {
                    if let Node::Num(nl) = lhs {
                        if nr > 0 {
                            return Ok(Node::Num(nl.pow(nr as u32)));
                        } else {
                            return Ok(Node::FNum((nl as f64).powi(nr as i32)));
                        }
                    } else if let Node::FNum(nl) = lhs {
                        return Ok(Node::FNum(nl.powi(nr as i32)));
                    } else if let Node::CNum(nl) = lhs {
                        return Ok(Node::CNum(nl.powi(nr as i32)));
                    }
                } else if let Node::FNum(nr) = rhs {
                    if let Node::Num(nl) = lhs {
                        return Ok(Node::FNum((nl as f64).powf(nr)));
                    } else if let Node::FNum(nl) = lhs {
                        return Ok(Node::FNum(nl.powf(nr)));
                    } else if let Node::CNum(nl) = lhs {
                        return Ok(Node::CNum(nl.powf(nr)));
                    }
                } else if let Node::CNum(nr) = rhs {
                    if let Node::Num(nl) = lhs {
                        return Ok(Node::CNum(Complex64::new(nl as f64, 0.0).powc(nr)));
                    } else if let Node::FNum(nl) = lhs {
                        return Ok(Node::CNum(Complex64::new(nl, 0.0).powc(nr)));
                    } else if let Node::CNum(nl) = lhs {
                        return Ok(Node::CNum(nl.powc(nr)));
                    }
                }
                return Ok(Node::Num(0));
            }
            _ => {
                return Err(MyError::EvalError(format!(
                    "unknown binary operator: {:?}",
                    n
                )));
            }
        }
    }
    Err(MyError::EvalError(format!("binary operator: {:?}", n)))
}

fn eval_unary(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("do_eval {:?}\r", n);
    }
    if let Node::Unary(tok, param) = n {
        if *tok == Token::Op(TokenOp::Plus) {
            return Ok(*(*param).clone());
        }
        if *tok == Token::Op(TokenOp::Minus) {
            let para: Node = *(*param).clone();
            if let Node::Num(n) = para {
                return Ok(Node::Num(-n));
            } else if let Node::FNum(f) = para {
                return Ok(Node::FNum(-f));
            } else if let Node::CNum(c) = para {
                return Ok(Node::CNum(-c));
            } else {
                let result = eval(env, &para)?;
                let new_node = Node::Unary(Token::Op(TokenOp::Minus), Box::new(result));
                return eval_unary(env, &new_node);
            }
        }
    }
    Err(MyError::EvalError(format!("unary operator {:?}", n)))
}

fn do_eval(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("do_eval {:?}\r", n);
    }
    match &*n {
        Node::Num(n) => Ok(Node::Num(*n)),
        Node::FNum(f) => Ok(Node::FNum(*f)),
        Node::CNum(c) => Ok(Node::CNum(*c)),
        Node::Unary(_tok, _param) => eval_unary(env, n),
        Node::BinOp(_tok, _lhs, _rhs) => eval_binop(env, n),
        Node::Var(_tok) => eval_const(env, n),
        Node::Func(_tok, _params) => eval_func(env, n),
        Node::Command(_tok, _params, _result) => eval_command(env, n),
        Node::None => Err(MyError::EvalError(format!("invalid node {:?}", n))),
    }
}

#[allow(clippy::never_loop)]
pub fn eval(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval {:?}\r", n);
    }
    loop {
        let result = do_eval(env, n)?;
        match result {
            Node::Num(_) | Node::FNum(_) | Node::CNum(_) => {
                return Ok(result);
            }
            Node::Command(_, _, _) => {
                return Ok(result);
            }
            Node::None => {
                return Ok(result);
            }
            _ => {
                return do_eval(env, &result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_as_string(env: &mut Env, input: &str) -> String {
        let n = parse(env, &(lexer(input.to_string())).unwrap()).unwrap();
        let n = eval(env, &n).unwrap();
        format!("{:?}", n)
    }

    fn eval_as_f64(env: &mut Env, input: &str) -> f64 {
        let n = parse(env, &(lexer(input.to_string())).unwrap()).unwrap();
        if let Node::FNum(f) = eval(env, &n).unwrap() {
            return f;
        }
        assert!(false);
        0.0
    }

    #[test]
    fn test_eval() {
        let mut env = Env::new();
        env.built_in();

        assert_eq!(eval_as_string(&mut env, "1+2"), "Num(3)".to_string());
        assert_eq!(eval_as_string(&mut env, "1+2*3"), "Num(7)".to_string());
        assert_eq!(eval_as_string(&mut env, "1*2+3"), "Num(5)".to_string());
        assert_eq!(eval_as_string(&mut env, "1+2+3"), "Num(6)".to_string());
        assert_eq!(eval_as_string(&mut env, "(1+2)*3"), "Num(9)".to_string());
        assert_eq!(eval_as_string(&mut env, "-2"), "Num(-2)".to_string());
        assert_eq!(
            eval_as_string(&mut env, "-9223372036854775807"),
            "Num(-9223372036854775807)".to_string()
        );
        assert!(((eval_as_f64(&mut env, "1.1+2.2") - 3.3).abs()) < 1e-10);
        assert_eq!(eval_as_string(&mut env, "-(2+3)"), "Num(-5)".to_string());
        assert_eq!(eval_as_string(&mut env, "+(2+3)"), "Num(5)".to_string());
        assert_eq!(eval_as_string(&mut env, "1.0+2"), "FNum(3.0)".to_string());
        assert_eq!(eval_as_string(&mut env, "1+2.0"), "FNum(3.0)".to_string());
        assert_eq!(
            eval_as_string(&mut env, "(1+2.0)*3"),
            "FNum(9.0)".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "pi"),
            "FNum(3.141592653589793)".to_string()
        );
        assert_eq!(eval_as_string(&mut env, "2k*3u"), "FNum(0.006)".to_string());

        assert_eq!(eval_as_string(&mut env, "5//5"), "FNum(2.5)".to_string());

        assert_eq!(
            eval_as_string(&mut env, "sin(0.0)"),
            "FNum(0.0)".to_string()
        );
        assert_eq!(eval_as_string(&mut env, "sin(0)"), "FNum(0.0)".to_string());
        assert!((eval_as_f64(&mut env, "sin(pi)").abs()) < 1e-10);
        assert!(((eval_as_f64(&mut env, "sin(pi/2)") - 1.0).abs()) < 1e-10);
        assert!(((eval_as_f64(&mut env, "abs(-2)") - 2.0).abs()) < 1e-10);
        assert_eq!(eval_as_string(&mut env, "sin(0)"), "FNum(0.0)".to_string());
        assert_eq!(eval_as_string(&mut env, "1%3"), "Num(1)".to_string());
        assert_eq!(eval_as_string(&mut env, "2%3"), "Num(2)".to_string());
        assert_eq!(eval_as_string(&mut env, "3%3"), "Num(0)".to_string());
        assert_eq!(eval_as_string(&mut env, "3.0%3"), "Num(0)".to_string());
        assert_eq!(eval_as_string(&mut env, "1/3"), "Num(0)".to_string());
        assert_eq!(eval_as_string(&mut env, "3/3"), "Num(1)".to_string());
        assert_eq!(eval_as_string(&mut env, "3.0/2"), "FNum(1.5)".to_string());
        assert_eq!(
            eval_as_string(&mut env, "ave(1,2,3)"),
            "FNum(2.0)".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "max(1,2,3)"),
            "FNum(3.0)".to_string()
        );
        eval_as_string(&mut env, "a=1");
        assert_eq!(eval_as_string(&mut env, "a"), "Num(1)".to_string());
        assert_eq!(eval_as_string(&mut env, "2^3"), "Num(8)".to_string());
        assert_eq!(
            eval_as_string(&mut env, "2^3^4"),
            "Num(2417851639229258349412352)".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "2^-0.5"),
            "FNum(0.7071067811865476)".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "1+2i"),
            "CNum(Complex { re: 1.0, im: 2.0 })".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2i) - (3-5i)"),
            "CNum(Complex { re: -2.0, im: 7.0 })".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2i) * (3-5i)"),
            "CNum(Complex { re: 13.0, im: 1.0 })".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2i) / (1-1.0i)"),
            "CNum(Complex { re: -0.5, im: 1.5 })".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "2 // 2i"),
            "CNum(Complex { re: 1.0, im: 1.0 })".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "i^i"),
            "CNum(Complex { re: 0.20787957635076193, im: 0.0 })".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "-pi"),
            "FNum(-3.141592653589793)".to_string()
        );
        eval_as_string(&mut env, "defun(double, 2*_1)");
        assert_eq!(eval_as_string(&mut env, "double(2)"), "Num(4)".to_string());
        assert_eq!(
            eval_as_string(&mut env, "double(double(2))"),
            "Num(8)".to_string()
        );
        eval_as_string(&mut env, "defun(add, _1+_2)");
        assert_eq!(eval_as_string(&mut env, "add(2,3)"), "Num(5)".to_string());
        assert_eq!(eval_as_string(&mut env, "add(2,a)"), "Num(3)".to_string());
        eval_as_string(&mut env, "defun(plus_a, a+_1)");
        eval_as_string(&mut env, "a=5");
        assert_eq!(eval_as_string(&mut env, "plus_a(8)"), "Num(13)".to_string());
        assert_eq!(eval_as_string(&mut env, "abs(-2)"), "FNum(2.0)".to_string());
        assert_eq!(
            eval_as_string(&mut env, "abs(-2.5)"),
            "FNum(2.5)".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "abs(1+i)"),
            "FNum(1.4142135623730951)".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "sqrt(2)"),
            "FNum(1.4142135623730951)".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "sqrt(2i)"),
            "CNum(Complex { re: 1.0000000000000002, im: 1.0 })".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "arg(1+i)"),
            "FNum(0.7853981633974483)".to_string()
        );
    }

    #[test]
    fn test_eva_error() {
        let mut env = Env::new();
        env.built_in();

        let n = parse(&mut env, &(lexer("pi=3".to_string())).unwrap()).unwrap();
        if let Ok(_) = eval(&mut env, &n) {
            assert!(false);
        }
    }
}
