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
mod units;

pub use env::*;
pub use lexer::*;
pub use parser::*;
pub use readline::readline;
pub use run_test::run_test;
pub use script::*;
pub use units::*;

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

pub fn eval_fvalue(_env: &Env, n: &Node) -> Result<f64, MyError> {
    match n {
        Node::Num(n, _) => Ok(*n as f64),
        Node::FNum(f, _) => Ok(*f),
        Node::None => Err(MyError::EvalError(
            "Node::None cannot convert to fvalue".to_owned(),
        )),
        _ => Err(MyError::EvalError(
            "Unexpected input: eval_fvalue".to_owned(),
        )),
    }
}

pub fn eval_cvalue(_env: &Env, n: &Node) -> Result<Complex64, MyError> {
    match n {
        Node::Num(n, _) => Ok(Complex64::new(*n as f64, 0.0)),
        Node::FNum(f, _) => Ok(Complex64::new(*f, 0.0)),
        Node::CNum(c, _) => Ok(*c),
        Node::None => Err(MyError::EvalError(
            "Node::None cannot convert to cvalue".to_owned(),
        )),
        _ => Err(MyError::EvalError(
            "Unexpected input: eval_fvalue".to_owned(),
        )),
    }
}

fn eval_const(env: &Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_const {:?}\r", n);
    }
    if let Node::Var(Token::Ident(ident)) = n {
        if let Some(constant) = env.is_const(ident.as_str()) {
            return Ok(constant);
        } else if let Some(variable) = env.is_variable(ident.as_str()) {
            return Ok(variable);
        }
    }
    Err(MyError::EvalError(format!(
        "unknown constant/variable: {:?}",
        n
    )))
}

fn node_to_token(n: Node) -> Vec<Token> {
    match n {
        Node::Num(n, _) => vec![Token::Num(n)],
        Node::FNum(f, _) => vec![Token::FNum(f)],
        Node::CNum(c, _) => vec![
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

fn eval_func(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_func {:?}\r", n);
    }
    if let Node::Func(Token::Ident(ident), param) = n {
        if let Some(func_tuple) = env.is_func(ident.as_str()) {
            let mut params: Vec<Node> = Vec::new();
            for i in param {
                let param_value = eval(env, i)?;
                params.push(param_value);
            }
            return Ok(func_tuple.0(env, &params));
        }
        if let Some(tokens) = env.is_user_func((*ident).clone()) {
            let mut params: Vec<Node> = Vec::new();
            for i in param {
                let param_value = eval(env, i)?;
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
                    // env.set_variable(id.clone(), (**rhs).clone())?; // assign is bind of AST
                    env.set_variable(id.clone(), do_eval(&mut env.clone(), rhs)?)?; // assign is bind of value at the assignment time
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

fn eval_num(env: &mut Env, node: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_num {:?}\r", node);
    }
    match node {
        Node::Num(n, u) => {
            if let Node::Units(units) = &**u {
                let (new_node, is_final) = eval_unit(env, units);
                if is_final {
                    Ok(Node::Num(*n, u.clone()))
                } else {
                    do_eval(
                        env,
                        &Node::BinOp(
                            Token::Op(TokenOp::Mul),
                            Box::new(Node::Num(*n, Box::new(Node::None))),
                            Box::new(new_node),
                        ),
                    )
                }
            } else {
                Ok(Node::Num(*n, u.clone()))
            }
        }
        Node::FNum(f, u) => {
            if let Node::Units(units) = &**u {
                let (new_node, is_final) = eval_unit(env, units);
                if is_final {
                    Ok(Node::FNum(*f, u.clone()))
                } else {
                    do_eval(
                        env,
                        &Node::BinOp(
                            Token::Op(TokenOp::Mul),
                            Box::new(Node::FNum(*f, Box::new(Node::None))),
                            Box::new(new_node),
                        ),
                    )
                }
            } else {
                Ok(Node::FNum(*f, u.clone()))
            }
        }
        Node::CNum(c, u) => Ok(Node::CNum(*c, u.clone())),
        _ => Ok(node.clone()),
    }
}

fn eval_binop(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_binop {:?}\r", n);
    }
    if let Node::BinOp(tok, lhs, rhs) = n {
        if *tok == Token::Op(TokenOp::Equal) {
            return eval_assign(env, n);
        }
        let lhs = do_eval(env, lhs)?;
        let rhs = do_eval(env, rhs)?;
        match tok {
            Token::Op(TokenOp::Plus) => match (lhs.clone(), rhs.clone()) {
                (Node::Num(nl, _ul), Node::Num(nr, ur)) => {
                    return Ok(Node::Num(nl + nr, ur));
                }
                (Node::Num(nl, _ul), Node::FNum(fr, ur)) => {
                    return Ok(Node::FNum(nl as f64 + fr, ur));
                }
                (Node::FNum(fl, _ul), Node::Num(nr, ur)) => {
                    return Ok(Node::FNum(fl + nr as f64, ur));
                }
                (Node::FNum(fl, _ul), Node::FNum(fr, ur)) => {
                    return Ok(Node::FNum(fl + fr, ur));
                }
                (_, _) => {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? + eval_cvalue(env, &rhs)?,
                        Box::new(Node::Units(Box::new(Node::None))),
                    ));
                }
            },
            Token::Op(TokenOp::Minus) => match (lhs.clone(), rhs.clone()) {
                (Node::Num(nl, _ul), Node::Num(nr, ur)) => {
                    return Ok(Node::Num(nl - nr, ur));
                }
                (Node::Num(nl, _ul), Node::FNum(fr, ur)) => {
                    return Ok(Node::FNum(nl as f64 - fr, ur));
                }
                (Node::FNum(fl, _ul), Node::Num(nr, ur)) => {
                    return Ok(Node::FNum(fl - nr as f64, ur));
                }
                (Node::FNum(fl, _ul), Node::FNum(fr, ur)) => {
                    return Ok(Node::FNum(fl - fr, ur));
                }
                (_, _) => {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? - eval_cvalue(env, &rhs)?,
                        Box::new(Node::Units(Box::new(Node::None))),
                    ));
                }
            },
            Token::Op(TokenOp::Mul) => match (lhs.clone(), rhs.clone()) {
                (Node::Num(nl, ul), Node::Num(nr, ur)) => {
                    return Ok(Node::Num(nl * nr, Box::new(eval_units_mul(env, &ul, &ur))));
                }
                (Node::Num(nl, ul), Node::FNum(fr, ur)) => {
                    return Ok(Node::FNum(
                        nl as f64 * fr,
                        Box::new(eval_units_mul(env, &ul, &ur)),
                    ));
                }
                (Node::FNum(fl, ul), Node::Num(nr, ur)) => {
                    return Ok(Node::FNum(
                        fl * nr as f64,
                        Box::new(eval_units_mul(env, &ul, &ur)),
                    ));
                }
                (Node::FNum(fl, ul), Node::FNum(fr, ur)) => {
                    return Ok(Node::FNum(fl * fr, Box::new(eval_units_mul(env, &ul, &ur))));
                }
                (_, _) => {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? * eval_cvalue(env, &rhs)?,
                        Box::new(Node::Units(Box::new(Node::None))),
                    ));
                }
            },
            Token::Op(TokenOp::Div) => match (lhs.clone(), rhs.clone()) {
                (Node::Num(nl, ul), Node::Num(nr, ur)) => {
                    let units = eval_units_div(env, &ul, &ur);
                    if nr == 0 {
                        return Ok(Node::FNum(f64::INFINITY, Box::new(units)));
                    }
                    return Ok(Node::Num(nl / nr, Box::new(units)));
                }
                (Node::Num(nl, ul), Node::FNum(fr, ur)) => {
                    return Ok(Node::FNum(
                        nl as f64 / fr,
                        Box::new(eval_units_div(env, &ul, &ur)),
                    ));
                }
                (Node::FNum(fl, ul), Node::Num(nr, ur)) => {
                    return Ok(Node::FNum(
                        fl / nr as f64,
                        Box::new(eval_units_div(env, &ul, &ur)),
                    ));
                }
                (Node::FNum(fl, ul), Node::FNum(fr, ur)) => {
                    return Ok(Node::FNum(fl / fr, Box::new(eval_units_div(env, &ul, &ur))));
                }
                (_, _) => {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? / eval_cvalue(env, &rhs)?,
                        Box::new(Node::Units(Box::new(Node::None))),
                    ));
                }
            },
            Token::Op(TokenOp::Para) => {
                if let Node::CNum(_, ref units) = lhs {
                    let lhs = eval_cvalue(env, &lhs)?;
                    let rhs = eval_cvalue(env, &rhs)?;
                    return Ok(Node::CNum((lhs * rhs) / (lhs + rhs), units.clone()));
                }
                if let Node::CNum(_, ref units) = rhs {
                    let lhs = eval_cvalue(env, &lhs)?;
                    let rhs = eval_cvalue(env, &rhs)?;
                    return Ok(Node::CNum((lhs * rhs) / (lhs + rhs), units.clone()));
                }
                let lhs = eval_fvalue(env, &lhs)?;
                let rhs = eval_fvalue(env, &rhs)?;
                return Ok(Node::FNum(
                    (lhs * rhs) / (lhs + rhs),
                    Box::new(Node::Units(Box::new(Node::None))),
                ));
            }
            Token::Op(TokenOp::Mod) => {
                if let Node::Num(nl, _) = lhs {
                    if let Node::Num(nr, units) = rhs {
                        return Ok(Node::Num(nl % nr, units));
                    }
                }
                return Ok(Node::Num(0, Box::new(Node::Units(Box::new(Node::None)))));
            }
            Token::Op(TokenOp::Caret) => {
                if let Node::Num(nr, _) = rhs {
                    if let Node::Num(nl, units) = lhs {
                        if nr > 0 {
                            return Ok(Node::Num(nl.pow(nr as u32), units));
                        } else {
                            return Ok(Node::FNum((nl as f64).powi(nr as i32), units));
                        }
                    } else if let Node::FNum(nl, units) = lhs {
                        return Ok(Node::FNum(nl.powi(nr as i32), units));
                    } else if let Node::CNum(nl, units) = lhs {
                        return Ok(Node::CNum(nl.powi(nr as i32), units));
                    }
                } else if let Node::FNum(nr, _) = rhs {
                    if let Node::Num(nl, units) = lhs {
                        return Ok(Node::FNum((nl as f64).powf(nr), units));
                    } else if let Node::FNum(nl, units) = lhs {
                        return Ok(Node::FNum(nl.powf(nr), units));
                    } else if let Node::CNum(nl, units) = lhs {
                        return Ok(Node::CNum(nl.powf(nr), units));
                    }
                } else if let Node::CNum(nr, _) = rhs {
                    if let Node::Num(nl, units) = lhs {
                        return Ok(Node::CNum(Complex64::new(nl as f64, 0.0).powc(nr), units));
                    } else if let Node::FNum(nl, units) = lhs {
                        return Ok(Node::CNum(Complex64::new(nl, 0.0).powc(nr), units));
                    } else if let Node::CNum(nl, units) = lhs {
                        return Ok(Node::CNum(nl.powc(nr), units));
                    }
                }
                return Ok(Node::Num(0, Box::new(Node::Units(Box::new(Node::None)))));
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
        eprintln!("eval_unary {:?}\r", n);
    }
    if let Node::Unary(tok, param) = n {
        if *tok == Token::Op(TokenOp::Plus) {
            return Ok(*(*param).clone());
        }
        if *tok == Token::Op(TokenOp::Minus) {
            let para: Node = *(*param).clone();
            if let Node::Num(n, units) = para {
                return Ok(Node::Num(-n, units));
            } else if let Node::FNum(f, units) = para {
                return Ok(Node::FNum(-f, units));
            } else if let Node::CNum(c, units) = para {
                return Ok(Node::CNum(-c, units));
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
    match n {
        Node::Num(_n, _units) => eval_num(env, n),
        Node::FNum(_f, _units) => eval_num(env, n),
        Node::CNum(_c, _units) => eval_num(env, n),
        Node::Unary(_tok, _param) => eval_unary(env, n),
        Node::BinOp(_tok, _lhs, _rhs) => eval_binop(env, n),
        Node::Var(_tok) => eval_const(env, n),
        Node::Func(_tok, _params) => eval_func(env, n),
        Node::Command(_tok, _params, _result) => eval_command(env, n),
        Node::None => Err(MyError::EvalError(format!("invalid node {:?}", n))),
        Node::Units(_) => todo!(),
        Node::UnitsFraction(_, _) => todo!(),
    }
}

fn eval(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval {:?}\r", n);
    }
    let ret = do_eval(env, n)?;
    let ret = eval_units_reduce(env, ret);
    let ret = match ret {
        Node::Num(n, ref u) => {
            if let Node::Units(units_content) = &**u {
                let u = eval_units_fraction(env, (**units_content).clone());
                Node::Num(n, Box::new(Node::Units(Box::new(u))))
            } else {
                ret
            }
        }
        Node::FNum(f, ref u) => {
            if let Node::Units(units_content) = &**u {
                let u = eval_units_fraction(env, (**units_content).clone());
                Node::FNum(f, Box::new(Node::Units(Box::new(u))))
            } else {
                ret
            }
        }
        Node::CNum(c, ref u) => {
            if let Node::Units(units_content) = &**u {
                let u = eval_units_fraction(env, (**units_content).clone());
                Node::CNum(c, Box::new(Node::Units(Box::new(u))))
            } else {
                ret
            }
        }
        _ => ret,
    };
    match ret {
        Node::Num(_, _) | Node::FNum(_, _) | Node::CNum(_, _) => Ok(ret),
        Node::Command(_, _, _) => Ok(ret),
        Node::None => Ok(ret),
        _ => eval(env, &ret),
    }
}

pub fn eval_top(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval {:?}\r", n);
    }
    let result = eval(env, n)?;
    match result {
        Node::Num(_, _) | Node::FNum(_, _) | Node::CNum(_, _) => {
            env.set_variable("ans".to_owned(), result.clone())?;
            Ok(result)
        }
        Node::Command(_, _, _) => Ok(result),
        Node::None => Ok(result),
        _ => Err(MyError::EvalError("unexpected return of eval()".to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_as_string(env: &mut Env, input: &str) -> String {
        let node = parse(env, &(lexer(input.to_owned())).unwrap()).unwrap();
        let node = eval(env, &node).unwrap();
        match node {
            Node::Num(num, ref u) => match &**u {
                Node::Units(un) => match &**un {
                    Node::UnitsFraction(a, b) => {
                        let units_str = units_fraction_to_string(a, b);
                        format!("Num({}, {})", num, units_str)
                    }
                    _ => format!("{:?}", node),
                },
                _ => format!("{:?}", node),
            },
            Node::FNum(fnum, ref u) => match &**u {
                Node::Units(un) => match &**un {
                    Node::UnitsFraction(a, b) => {
                        let units_str = units_fraction_to_string(a, b);
                        format!("FNum({:?}, {})", fnum, units_str)
                    }
                    _ => format!("{:?}", node),
                },
                _ => format!("{:?}", node),
            },
            Node::CNum(cnum, ref u) => match &**u {
                Node::Units(un) => match &**un {
                    Node::UnitsFraction(a, b) => {
                        let units_str = units_fraction_to_string(a, b);
                        format!("CNum({:?}, {})", cnum, units_str)
                    }
                    _ => format!("{:?}", node),
                },
                _ => format!("{:?}", node),
            },
            _ => format!("{:?}", node),
        }
    }

    fn eval_as_f64(env: &mut Env, input: &str) -> f64 {
        let n = parse(env, &(lexer(input.to_owned())).unwrap()).unwrap();
        if let Node::FNum(f, _) = eval(env, &n).unwrap() {
            return f;
        }
        assert!(false);
        0.0
    }

    fn eval_as_complex64(env: &mut Env, input: &str) -> Complex64 {
        let n = parse(env, &(lexer(input.to_owned())).unwrap()).unwrap();
        if let Node::CNum(c, _) = eval(env, &n).unwrap() {
            return c;
        }
        assert!(false);
        Complex64::new(0.0, 0.0)
    }

    #[test]
    fn test_eval() {
        let mut env = Env::new();
        env.built_in();

        assert_eq!(eval_as_string(&mut env, "1+2"), "Num(3, []/[])".to_owned());
        assert_eq!(
            eval_as_string(&mut env, "1+2*3"),
            "Num(7, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1*2+3"),
            "Num(5, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1+2+3"),
            "Num(6, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2)*3"),
            "Num(9, []/[])".to_owned()
        );
        assert_eq!(eval_as_string(&mut env, "-2"), "Num(-2, []/[])".to_owned());
        assert_eq!(
            eval_as_string(&mut env, "-9223372036854775807"),
            "Num(-9223372036854775807, []/[])".to_owned()
        );
        assert!(((eval_as_f64(&mut env, "1.1+2.2") - 3.3).abs()) < 1e-10);
        assert_eq!(
            eval_as_string(&mut env, "-(2+3)"),
            "Num(-5, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "+(2+3)"),
            "Num(5, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1.0+2"),
            "FNum(3.0, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1+2.0"),
            "FNum(3.0, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2.0)*3"),
            "FNum(9.0, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "pi"),
            "FNum(3.141592653589793, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2k*3u"),
            "FNum(0.006, []/[])".to_owned()
        );

        assert_eq!(
            eval_as_string(&mut env, "5//5"),
            "FNum(2.5, []/[])".to_owned()
        );

        assert!((eval_as_f64(&mut env, "sin(0.0)")).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "cos(pi/2)")).abs() < 1e-10);
        assert_eq!(
            eval_as_string(&mut env, "sin(0)"),
            "FNum(0.0, []/[])".to_owned()
        );
        assert!((eval_as_f64(&mut env, "sin(pi)").abs()) < 1e-10);
        assert!(((eval_as_f64(&mut env, "sin(pi/2)") - 1.0).abs()) < 1e-10);
        assert!(((eval_as_f64(&mut env, "abs(-2)") - 2.0).abs()) < 1e-10);
        assert_eq!(
            eval_as_string(&mut env, "sin(0)"),
            "FNum(0.0, []/[])".to_owned()
        );
        assert_eq!(eval_as_string(&mut env, "1%3"), "Num(1, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "2%3"), "Num(2, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "3%3"), "Num(0, []/[])".to_owned());
        assert_eq!(
            eval_as_string(&mut env, "3.0%3"),
            "Num(0, []/[])".to_owned()
        );
        assert_eq!(eval_as_string(&mut env, "1/3"), "Num(0, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "3/3"), "Num(1, []/[])".to_owned());
        assert_eq!(
            eval_as_string(&mut env, "3.0/2"),
            "FNum(1.5, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "ave(1,2,3)"),
            "FNum(2.0, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "max(1,2,3)"),
            "FNum(3.0, []/[])".to_owned()
        );
        eval_as_string(&mut env, "a=1");
        assert_eq!(eval_as_string(&mut env, "a"), "Num(1, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "2^3"), "Num(8, []/[])".to_owned());
        assert_eq!(
            eval_as_string(&mut env, "2^3^4"),
            "Num(2417851639229258349412352, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2^-0.5"),
            "FNum(0.7071067811865476, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1+2i"),
            "CNum(Complex { re: 1.0, im: 2.0 }, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2i) - (3-5i)"),
            "CNum(Complex { re: -2.0, im: 7.0 }, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2i) * (3-5i)"),
            "CNum(Complex { re: 13.0, im: 1.0 }, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2i) / (1-1.0i)"),
            "CNum(Complex { re: -0.5, im: 1.5 }, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2 // 2i"),
            "CNum(Complex { re: 1.0, im: 1.0 }, []/[])".to_owned()
        );
        assert!((eval_as_complex64(&mut env, "exp(i*pi)").re + 1.0).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "exp(i*pi)").im).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "i^i").re - 0.20787957635076193).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "i^i").im).abs() < 1e-10);
        assert_eq!(
            eval_as_string(&mut env, "-pi"),
            "FNum(-3.141592653589793, []/[])".to_owned()
        );
        eval_as_string(&mut env, "defun double 2*_1");
        assert_eq!(
            eval_as_string(&mut env, "double(2)"),
            "Num(4, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "double(double(2))"),
            "Num(8, []/[])".to_owned()
        );
        eval_as_string(&mut env, "defun add _1+_2");
        assert_eq!(
            eval_as_string(&mut env, "add(2,3)"),
            "Num(5, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "add(2,a)"),
            "Num(3, []/[])".to_owned()
        );
        eval_as_string(&mut env, "defun plus_a a+_1");
        eval_as_string(&mut env, "a=5");
        assert_eq!(
            eval_as_string(&mut env, "plus_a(8)"),
            "Num(13, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "abs(-2)"),
            "FNum(2.0, []/[])".to_owned()
        );
        assert!((eval_as_f64(&mut env, "abs(-2.5)") - 2.5).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "abs(1+i)") - 1.4142135623730951).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "sqrt(2)") - 1.4142135623730951).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "sqrt(2i)").re - 1.0).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "sqrt(2i)").im - 1.0).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "arg(1+i)") - 0.7853981633974483).abs() < 1e-10);
        eval_as_string(&mut env, "b=a");
        eval_as_string(&mut env, "c=b");
        assert_eq!(eval_as_string(&mut env, "c"), "Num(5, []/[])".to_owned());
        eval_as_string(&mut env, "d=c");
        eval_as_string(&mut env, "ee=d+c");
        assert_eq!(eval_as_string(&mut env, "ee"), "Num(10, []/[])".to_owned());

        // assignment
        eval_as_string(&mut env, "aa=1");
        eval_as_string(&mut env, "bb=aa");
        eval_as_string(&mut env, "aa=2");
        // assert_eq!(eval_as_string(&mut env, "bb"), "Num(2)".to_owned());    // assign to bb is binded to AST of aa
        assert_eq!(eval_as_string(&mut env, "bb"), "Num(1, []/[])".to_owned()); // assign to bb is ?binded to value of aa at the assigned time

        // divided by zero
        assert_eq!(
            eval_as_string(&mut env, "1/0"),
            "FNum(inf, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1.0/0.0"),
            "FNum(inf, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1/(0.0+0.0i)"),
            "CNum(Complex { re: NaN, im: NaN }, []/[])".to_owned()
        );
    }

    #[test]
    fn test_eval_error() {
        let mut env = Env::new();
        env.built_in();

        let n = parse(&mut env, &(lexer("pi=3".to_owned())).unwrap()).unwrap();
        if let Ok(_) = eval(&mut env, &n) {
            assert!(false);
        }
        let n = parse(&mut env, &(lexer("abs(1-i+)".to_owned())).unwrap()).unwrap();
        if let Ok(_) = eval(&mut env, &n) {
            assert!(false);
        }
    }
}
