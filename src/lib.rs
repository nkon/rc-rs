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
            "Unexpected input: eval_cvalue".to_owned(),
        )),
    }
}

fn eval_add(env: &Env, lhs: &Node, rhs: &Node) -> Result<Node, MyError> {
    match (lhs, rhs) {
        (Node::Num(nl, _ul), Node::Num(nr, ur)) => {
            Ok(Node::Num(nl + nr, ur.clone()))
        }
        (Node::Num(nl, _ul), Node::FNum(fr, ur)) => {
            Ok(Node::FNum(*nl as f64 + fr, ur.clone()))
        }
        (Node::FNum(fl, _ul), Node::Num(nr, ur)) => {
            Ok(Node::FNum(fl + *nr as f64, ur.clone()))
        }
        (Node::FNum(fl, _ul), Node::FNum(fr, ur)) => {
            Ok(Node::FNum(fl + fr, ur.clone()))
        }
        (_, _) => {
            Ok(Node::CNum(
                eval_cvalue(env, lhs)? + eval_cvalue(env, rhs)?,
                Box::new(Node::Units(Box::new(Node::None))),
            ))
        }
    }
}

fn eval_subtract(env: &Env, lhs: &Node, rhs: &Node) -> Result<Node, MyError> {
    match (lhs, rhs) {
        (Node::Num(nl, _ul), Node::Num(nr, ur)) => {
            Ok(Node::Num(nl - nr, ur.clone()))
        }
        (Node::Num(nl, _ul), Node::FNum(fr, ur)) => {
            Ok(Node::FNum(*nl as f64 - fr, ur.clone()))
        }
        (Node::FNum(fl, _ul), Node::Num(nr, ur)) => {
            Ok(Node::FNum(fl - *nr as f64, ur.clone()))
        }
        (Node::FNum(fl, _ul), Node::FNum(fr, ur)) => {
            Ok(Node::FNum(fl - fr, ur.clone()))
        }
        (_, _) => {
            Ok(Node::CNum(
                eval_cvalue(env, lhs)? - eval_cvalue(env, rhs)?,
                Box::new(Node::Units(Box::new(Node::None))),
            ))
        }
    }
}

fn eval_multiply(env: &mut Env, lhs: &Node, rhs: &Node) -> Result<Node, MyError> {
    match (lhs, rhs) {
        (Node::Num(nl, ul), Node::Num(nr, ur)) => {
            Ok(Node::Num(nl * nr, Box::new(eval_units_mul(env, ul, ur))))
        }
        (Node::Num(nl, ul), Node::FNum(fr, ur)) => {
            Ok(Node::FNum(
                *nl as f64 * fr,
                Box::new(eval_units_mul(env, ul, ur)),
            ))
        }
        (Node::FNum(fl, ul), Node::Num(nr, ur)) => {
            Ok(Node::FNum(
                fl * *nr as f64,
                Box::new(eval_units_mul(env, ul, ur)),
            ))
        }
        (Node::FNum(fl, ul), Node::FNum(fr, ur)) => {
            Ok(Node::FNum(fl * fr, Box::new(eval_units_mul(env, ul, ur))))
        }
        (_, _) => {
            Ok(Node::CNum(
                eval_cvalue(env, lhs)? * eval_cvalue(env, rhs)?,
                Box::new(Node::Units(Box::new(Node::None))),
            ))
        }
    }
}

fn eval_divide(env: &mut Env, lhs: &Node, rhs: &Node) -> Result<Node, MyError> {
    match (lhs, rhs) {
        (Node::Num(nl, ul), Node::Num(nr, ur)) => {
            let units = eval_units_div(env, ul, ur);
            if *nr == 0 {
                Ok(Node::FNum(f64::INFINITY, Box::new(units)))
            } else {
                Ok(Node::Num(nl / nr, Box::new(units)))
            }
        }
        (Node::Num(nl, ul), Node::FNum(fr, ur)) => {
            Ok(Node::FNum(
                *nl as f64 / fr,
                Box::new(eval_units_div(env, ul, ur)),
            ))
        }
        (Node::FNum(fl, ul), Node::Num(nr, ur)) => {
            Ok(Node::FNum(
                fl / *nr as f64,
                Box::new(eval_units_div(env, ul, ur)),
            ))
        }
        (Node::FNum(fl, ul), Node::FNum(fr, ur)) => {
            Ok(Node::FNum(fl / fr, Box::new(eval_units_div(env, ul, ur))))
        }
        (_, _) => {
            Ok(Node::CNum(
                eval_cvalue(env, lhs)? / eval_cvalue(env, rhs)?,
                Box::new(Node::Units(Box::new(Node::None))),
            ))
        }
    }
}

fn eval_parallel(env: &Env, lhs: &Node, rhs: &Node) -> Result<Node, MyError> {
    if let Node::CNum(_, ref units) = lhs {
        let lhs_val = eval_cvalue(env, lhs)?;
        let rhs_val = eval_cvalue(env, rhs)?;
        return Ok(Node::CNum((lhs_val * rhs_val) / (lhs_val + rhs_val), units.clone()));
    }
    if let Node::CNum(_, ref units) = rhs {
        let lhs_val = eval_cvalue(env, lhs)?;
        let rhs_val = eval_cvalue(env, rhs)?;
        return Ok(Node::CNum((lhs_val * rhs_val) / (lhs_val + rhs_val), units.clone()));
    }
    let lhs_val = eval_fvalue(env, lhs)?;
    let rhs_val = eval_fvalue(env, rhs)?;
    Ok(Node::FNum(
        (lhs_val * rhs_val) / (lhs_val + rhs_val),
        Box::new(Node::Units(Box::new(Node::None))),
    ))
}

fn eval_modulo(lhs: &Node, rhs: &Node) -> Result<Node, MyError> {
    if let (Node::Num(nl, _), Node::Num(nr, units)) = (lhs, rhs) {
        Ok(Node::Num(nl % nr, units.clone()))
    } else {
        Ok(Node::Num(0, Box::new(Node::Units(Box::new(Node::None)))))
    }
}

fn eval_power(lhs: &Node, rhs: &Node) -> Result<Node, MyError> {
    match rhs {
        Node::Num(nr, _) => {
            match lhs {
                Node::Num(nl, units) => {
                    if *nr > 0 {
                        Ok(Node::Num(nl.pow(*nr as u32), units.clone()))
                    } else {
                        Ok(Node::FNum((*nl as f64).powi(*nr as i32), units.clone()))
                    }
                }
                Node::FNum(nl, units) => {
                    Ok(Node::FNum(nl.powi(*nr as i32), units.clone()))
                }
                Node::CNum(nl, units) => {
                    Ok(Node::CNum(nl.powi(*nr as i32), units.clone()))
                }
                _ => Ok(Node::Num(0, Box::new(Node::Units(Box::new(Node::None)))))
            }
        }
        Node::FNum(nr, _) => {
            match lhs {
                Node::Num(nl, units) => {
                    Ok(Node::FNum((*nl as f64).powf(*nr), units.clone()))
                }
                Node::FNum(nl, units) => {
                    Ok(Node::FNum(nl.powf(*nr), units.clone()))
                }
                Node::CNum(nl, units) => {
                    Ok(Node::CNum(nl.powf(*nr), units.clone()))
                }
                _ => Ok(Node::Num(0, Box::new(Node::Units(Box::new(Node::None)))))
            }
        }
        Node::CNum(nr, _) => {
            match lhs {
                Node::Num(nl, units) => {
                    Ok(Node::CNum(Complex64::new(*nl as f64, 0.0).powc(*nr), units.clone()))
                }
                Node::FNum(nl, units) => {
                    Ok(Node::CNum(Complex64::new(*nl, 0.0).powc(*nr), units.clone()))
                }
                Node::CNum(nl, units) => {
                    Ok(Node::CNum(nl.powc(*nr), units.clone()))
                }
                _ => Ok(Node::Num(0, Box::new(Node::Units(Box::new(Node::None)))))
            }
        }
        _ => Ok(Node::Num(0, Box::new(Node::Units(Box::new(Node::None)))))
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
            let new_node = func_tuple.0(env, &params);
            return do_eval(env, &new_node);
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
            Token::Op(TokenOp::Plus) => eval_add(env, &lhs, &rhs),
            Token::Op(TokenOp::Minus) => eval_subtract(env, &lhs, &rhs),
            Token::Op(TokenOp::Mul) => eval_multiply(env, &lhs, &rhs),
            Token::Op(TokenOp::Div) => eval_divide(env, &lhs, &rhs),
            Token::Op(TokenOp::Para) => eval_parallel(env, &lhs, &rhs),
            Token::Op(TokenOp::Mod) => eval_modulo(&lhs, &rhs),
            Token::Op(TokenOp::Caret) => eval_power(&lhs, &rhs),
            _ => Err(MyError::EvalError(format!(
                "unknown binary operator: {:?}",
                n
            ))),
        }
    } else {
        Err(MyError::EvalError(format!("binary operator: {:?}", n)))
    }
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
        panic!("eval_as_f64 failed");
    }

    fn eval_as_complex64(env: &mut Env, input: &str) -> Complex64 {
        let n = parse(env, &(lexer(input.to_owned())).unwrap()).unwrap();
        if let Node::CNum(c, _) = eval(env, &n).unwrap() {
            return c;
        }
        panic!("eval_as_complex64 failed");
    }

    #[test]
    fn test_basic_arithmetic() {
        let mut env = Env::new();
        env.built_in();

        // Basic integer arithmetic
        assert_eq!(eval_as_string(&mut env, "1+2"), "Num(3, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "1+2*3"), "Num(7, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "1*2+3"), "Num(5, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "1+2+3"), "Num(6, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "(1+2)*3"), "Num(9, []/[])".to_owned());
        
        // Unary operators
        assert_eq!(eval_as_string(&mut env, "-2"), "Num(-2, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "-(2+3)"), "Num(-5, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "+(2+3)"), "Num(5, []/[])".to_owned());
        
        // Large numbers
        assert_eq!(
            eval_as_string(&mut env, "-9223372036854775807"),
            "Num(-9223372036854775807, []/[])".to_owned()
        );
    }

    #[test]
    fn test_float_arithmetic() {
        let mut env = Env::new();
        env.built_in();

        // Float arithmetic
        assert!(((eval_as_f64(&mut env, "1.1+2.2") - 3.3).abs()) < 1e-10);
        assert_eq!(eval_as_string(&mut env, "1.0+2"), "FNum(3.0, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "1+2.0"), "FNum(3.0, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "(1+2.0)*3"), "FNum(9.0, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "3.0/2"), "FNum(1.5, []/[])".to_owned());
    }

    #[test]
    fn test_modulo_and_division() {
        let mut env = Env::new();
        env.built_in();

        // Modulo operations
        assert_eq!(eval_as_string(&mut env, "1%3"), "Num(1, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "2%3"), "Num(2, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "3%3"), "Num(0, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "3.0%3"), "Num(0, []/[])".to_owned());
        
        // Integer division
        assert_eq!(eval_as_string(&mut env, "1/3"), "Num(0, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "3/3"), "Num(1, []/[])".to_owned());
        
        // Division by zero
        assert_eq!(eval_as_string(&mut env, "1/0"), "FNum(inf, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "1.0/0.0"), "FNum(inf, []/[])".to_owned());
    }

    #[test]
    fn test_power_operations() {
        let mut env = Env::new();
        env.built_in();

        assert_eq!(eval_as_string(&mut env, "2^3"), "Num(8, []/[])".to_owned());
        assert_eq!(
            eval_as_string(&mut env, "2^3^4"),
            "Num(2417851639229258349412352, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2^-0.5"),
            "FNum(0.7071067811865476, []/[])".to_owned()
        );
    }

    #[test]
    fn test_parallel_operation() {
        let mut env = Env::new();
        env.built_in();

        // Parallel operation (//) - commonly used in electronics
        assert_eq!(eval_as_string(&mut env, "5//5"), "FNum(2.5, []/[])".to_owned());
    }

    #[test]
    fn test_constants_and_variables() {
        let mut env = Env::new();
        env.built_in();

        // Constants
        assert_eq!(
            eval_as_string(&mut env, "pi"),
            "FNum(3.141592653589793, []/[])".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "-pi"),
            "FNum(-3.141592653589793, []/[])".to_owned()
        );
        
        // SI prefixes
        assert_eq!(eval_as_string(&mut env, "2k*3u"), "FNum(0.006, []/[])".to_owned());
        
        // Variable assignment and retrieval
        eval_as_string(&mut env, "a=1");
        assert_eq!(eval_as_string(&mut env, "a"), "Num(1, []/[])".to_owned());
    }

    #[test]
    fn test_variable_assignment_semantics() {
        let mut env = Env::new();
        env.built_in();

        // Test variable assignment semantics (value binding vs AST binding)
        eval_as_string(&mut env, "aa=1");
        eval_as_string(&mut env, "bb=aa");
        eval_as_string(&mut env, "aa=2");
        // Assignment binds to value at assignment time, not AST
        assert_eq!(eval_as_string(&mut env, "bb"), "Num(1, []/[])".to_owned());
        
        // Chain assignment
        eval_as_string(&mut env, "a=5");
        eval_as_string(&mut env, "b=a");
        eval_as_string(&mut env, "c=b");
        assert_eq!(eval_as_string(&mut env, "c"), "Num(5, []/[])".to_owned());
        eval_as_string(&mut env, "d=c");
        eval_as_string(&mut env, "ee=d+c");
        assert_eq!(eval_as_string(&mut env, "ee"), "Num(10, []/[])".to_owned());
    }

    #[test]
    fn test_complex_numbers() {
        let mut env = Env::new();
        env.built_in();

        // Basic complex number operations
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
        
        // Parallel operation with complex numbers
        assert_eq!(
            eval_as_string(&mut env, "2 // 2i"),
            "CNum(Complex { re: 1.0, im: 1.0 }, []/[])".to_owned()
        );
        
        // Division by zero with complex numbers
        assert_eq!(
            eval_as_string(&mut env, "1/(0.0+0.0i)"),
            "CNum(Complex { re: NaN, im: NaN }, []/[])".to_owned()
        );
    }

    #[test]
    fn test_famous_complex_identities() {
        let mut env = Env::new();
        env.built_in();

        // Euler's identity: e^(i*π) = -1
        assert!((eval_as_complex64(&mut env, "exp(i*pi)").re + 1.0).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "exp(i*pi)").im).abs() < 1e-10);
        
        // i^i is real
        assert!((eval_as_complex64(&mut env, "i^i").re - 0.20787957635076193).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "i^i").im).abs() < 1e-10);
    }

    #[test]
    fn test_built_in_functions() {
        let mut env = Env::new();
        env.built_in();

        // Trigonometric functions
        assert!((eval_as_f64(&mut env, "sin(0.0)")).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "cos(pi/2)")).abs() < 1e-10);
        assert_eq!(eval_as_string(&mut env, "sin(0)"), "FNum(0.0, []/[])".to_owned());
        assert!((eval_as_f64(&mut env, "sin(pi)").abs()) < 1e-10);
        assert!(((eval_as_f64(&mut env, "sin(pi/2)") - 1.0).abs()) < 1e-10);
        
        // Absolute value and square root
        assert!(((eval_as_f64(&mut env, "abs(-2)") - 2.0).abs()) < 1e-10);
        assert_eq!(eval_as_string(&mut env, "abs(-2)"), "FNum(2.0, []/[])".to_owned());
        assert!((eval_as_f64(&mut env, "abs(-2.5)") - 2.5).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "abs(1+i)") - std::f64::consts::SQRT_2).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "sqrt(2)") - std::f64::consts::SQRT_2).abs() < 1e-10);
        
        // Complex square root
        assert!((eval_as_complex64(&mut env, "sqrt(2i)").re - 1.0).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "sqrt(2i)").im - 1.0).abs() < 1e-10);
        
        // Argument function
        assert!((eval_as_f64(&mut env, "arg(1+i)") - std::f64::consts::FRAC_PI_4).abs() < 1e-10);
        
        // Statistical functions
        assert_eq!(eval_as_string(&mut env, "ave(1,2,3)"), "FNum(2.0, []/[])".to_owned());
        assert_eq!(eval_as_string(&mut env, "max(1,2,3)"), "FNum(3.0, []/[])".to_owned());
        
        // Function with units
        assert_eq!(
            eval_as_string(&mut env, "1/sqrt(2)"),
            "FNum(0.7071067811865475, [(\"_\", 1)]/[])".to_owned()
        );
    }

    #[test]
    fn test_user_defined_functions() {
        let mut env = Env::new();
        env.built_in();

        // Simple user-defined function
        eval_as_string(&mut env, "defun double 2*_1");
        assert_eq!(eval_as_string(&mut env, "double(2)"), "Num(4, []/[])".to_owned());
        
        // Recursive user-defined function call
        assert_eq!(eval_as_string(&mut env, "double(double(2))"), "Num(8, []/[])".to_owned());
        
        // Multi-parameter user-defined function
        eval_as_string(&mut env, "defun add _1+_2");
        assert_eq!(eval_as_string(&mut env, "add(2,3)"), "Num(5, []/[])".to_owned());
        
        // User-defined function with variable
        eval_as_string(&mut env, "a=1");
        assert_eq!(eval_as_string(&mut env, "add(2,a)"), "Num(3, []/[])".to_owned());
        
        // User-defined function that captures variables
        eval_as_string(&mut env, "defun plus_a a+_1");
        eval_as_string(&mut env, "a=5");
        assert_eq!(eval_as_string(&mut env, "plus_a(8)"), "Num(13, []/[])".to_owned());
    }

    #[test]
    fn test_assignment_errors() {
        let mut env = Env::new();
        env.built_in();

        // Cannot assign to constants
        let n = parse(&mut env, &(lexer("pi=3".to_owned())).unwrap()).unwrap();
        assert!(eval(&mut env, &n).is_err(), "Should not be able to assign to constant 'pi'");
    }

    #[test]
    fn test_syntax_errors() {
        let mut env = Env::new();
        env.built_in();

        // Incomplete expression with dangling operator
        let n = parse(&mut env, &(lexer("abs(1-i+)".to_owned())).unwrap()).unwrap();
        assert!(eval(&mut env, &n).is_err(), "Should fail on incomplete expression");
    }
}
