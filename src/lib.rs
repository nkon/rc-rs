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

pub fn eval_fvalue(_env: &mut Env, n: &Node) -> f64 {
    match n {
        Node::Num(n) => *n as f64,
        Node::FNum(f) => *f,
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
                return Ok(Node::FNum(constant));
            } else if let Some(variable) = env.is_variable(ident.as_str()) {
                return Ok(Node::FNum(variable));
            }
        }
    }
    Err(MyError::EvalError(format!(
        "unknown constant/variable: {:?}",
        n
    )))
}

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
                    let n_param: Node;
                    match param_value {
                        Node::Num(n) => {
                            n_param = Node::FNum(n as f64);
                        }
                        Node::FNum(f) => {
                            n_param = Node::FNum(f);
                        }
                        _ => {
                            n_param = Node::None;
                        }
                    }
                    params.push(n_param.clone());
                }
                return Ok(Node::FNum(func_tuple.0(env, &params)));
            }
        }
    }
    Err(MyError::EvalError(format!("unknown function: {:?}", n)))
}

fn eval_assign(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_assign {:?}\r", n);
    }
    if let Node::BinOp(tok, lhs, rhs) = n {
        assert_eq!(*tok, Token::Op(TokenOp::Equal));
        let rhs = eval_fvalue(env, rhs);
        match &**lhs {
            Node::Var(Token::Ident(id)) => {
                if env.is_variable(id).is_some() {
                    env.set_variable(id.clone(), rhs)?;
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
                return Ok(Node::FNum(eval_fvalue(env, &lhs) + eval_fvalue(env, &rhs)));
            }
            Token::Op(TokenOp::Minus) => {
                if let Node::Num(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::Num(nl - nr));
                    }
                }
                return Ok(Node::FNum(eval_fvalue(env, &lhs) - eval_fvalue(env, &rhs)));
            }
            Token::Op(TokenOp::Mul) => {
                if let Node::Num(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::Num(nl * nr));
                    }
                }
                return Ok(Node::FNum(eval_fvalue(env, &lhs) * eval_fvalue(env, &rhs)));
            }
            Token::Op(TokenOp::Div) => {
                if let Node::Num(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::Num(nl / nr));
                    }
                }
                return Ok(Node::FNum(eval_fvalue(env, &lhs) / eval_fvalue(env, &rhs)));
            }
            Token::Op(TokenOp::Para) => {
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
                if let Node::Num(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::Num(nl.pow(nr as u32)));
                    } else if let Node::FNum(nr) = rhs {
                        return Ok(Node::FNum((nl as f64).powf(nr)));
                    }
                } else if let Node::FNum(nl) = lhs {
                    if let Node::Num(nr) = rhs {
                        return Ok(Node::FNum(nl.powi(nr as i32)));
                    } else if let Node::FNum(nr) = rhs {
                        return Ok(Node::FNum(nl.powf(nr)));
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

pub fn eval(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval {:?}\r", n);
    }
    match &*n {
        Node::Num(n) => Ok(Node::Num(*n)),
        Node::FNum(f) => Ok(Node::FNum(*f)),
        Node::Unary(tok, para_boxed) => {
            let para: Node = *(*para_boxed).clone();
            match tok {
                Token::Op(TokenOp::Minus) => {
                    if let Node::Num(n) = para {
                        Ok(Node::Num(-n))
                    } else if let Node::FNum(f) = para {
                        Ok(Node::FNum(-f))
                    } else if let Node::BinOp(tok, lhs_box, rhs_box) = para {
                        let n_result = eval_binop(env, &Node::BinOp(tok, lhs_box, rhs_box))?;
                        if let Node::Num(n) = n_result {
                            Ok(Node::Num(-n))
                        } else if let Node::FNum(f) = n_result {
                            Ok(Node::FNum(-f))
                        } else {
                            Err(MyError::EvalError(format!(
                                "invalid operand {:?}",
                                n_result
                            )))
                        }
                    } else {
                        Err(MyError::EvalError(format!("invalid operand {:?}", tok)))
                    }
                }
                Token::Op(TokenOp::Plus) => {
                    if let Node::Num(n) = para {
                        Ok(Node::Num(n))
                    } else if let Node::FNum(f) = para {
                        Ok(Node::FNum(f))
                    } else if let Node::BinOp(tok, lhs_box, rhs_box) = para {
                        eval_binop(env, &Node::BinOp(tok, lhs_box, rhs_box))
                    } else {
                        Err(MyError::EvalError(format!("invalid operand {:?}", tok)))
                    }
                }
                _ => Err(MyError::EvalError(format!("unknown token {:?}", tok))),
            }
        }
        Node::BinOp(_tok, _lhs, _rhs) => eval_binop(env, n),
        Node::Var(_tok) => eval_const(env, n),
        Node::Func(_tok, _params) => eval_func(env, n),
        Node::None => Err(MyError::EvalError(format!("invalid node {:?}", n))),
        _ => Ok(n.clone()),
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
        assert_eq!(eval_as_string(&mut env, "a"), "FNum(1.0)".to_string());
        assert_eq!(eval_as_string(&mut env, "2^3"), "Num(8)".to_string());
        assert_eq!(
            eval_as_string(&mut env, "2^3^4"),
            "Num(2417851639229258349412352)".to_string()
        );
        assert_eq!(
            eval_as_string(&mut env, "2^-0.5"),
            "FNum(0.7071067811865476)".to_string()
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
