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
pub use script::*;

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

fn eval_units_mul(env: &mut Env, lhs_u: &Node, rhs_u: &Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_units_mul {:?} {:?}\r", lhs_u, rhs_u);
    }
    match (lhs_u, rhs_u) {
        (Node::Units(lhs_i_p), _) => {
            // unpack Units() and call recursive
            if let Node::Units(rhs_i_p) = rhs_u {
                eval_units_mul(env, lhs_i_p, rhs_i_p)
            } else {
                eval_units_mul(env, lhs_i_p, rhs_u)
            }
        }
        (_, Node::Units(rhs_i_p)) => {
            // unpack Units() and call recursive
            eval_units_mul(env, lhs_u, rhs_i_p)
        }
        (Node::None, Node::None) => Node::Units(Box::new(Node::None)),
        (Node::None, _) => {
            // (lhs_u == None) ==> return rhs_u
            rhs_u.clone()
        }
        (_, Node::None) => {
            // (rhs_u == None) ==> return lhs_u
            lhs_u.clone()
        }
        (_, _) => Node::Units(Box::new(Node::BinOp(
            Token::Op(TokenOp::Mul),
            Box::new(lhs_u.clone()),
            Box::new(rhs_u.clone()),
        ))),
    }
}

fn eval_units_div(env: &mut Env, lhs_u: &Node, rhs_u: &Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_units_div {:?} {:?}\r", lhs_u, rhs_u);
    }
    match (lhs_u, rhs_u) {
        (Node::Units(lhs_i_p), _) => {
            // unpack Units() and call recursive
            if let Node::Units(rhs_i_p) = rhs_u {
                eval_units_div(env, lhs_i_p, rhs_i_p)
            } else {
                eval_units_div(env, lhs_i_p, rhs_u)
            }
        }
        (_, Node::Units(rhs_i_p)) => {
            // unpack Units() and call recursive
            eval_units_div(env, lhs_u, rhs_i_p)
        }
        (Node::None, Node::None) => Node::Units(Box::new(Node::None)),
        (Node::None, _) => {
            // (lhs_u == None) ==> return (1/rhs_u)
            Node::Units(Box::new(Node::BinOp(
                Token::Op(TokenOp::Div),
                Box::new(Node::Num(1, Box::new(Node::Units(Box::new(Node::None))))),
                Box::new(rhs_u.clone()),
            )))
        }
        (_, _) => Node::Units(Box::new(Node::BinOp(
            Token::Op(TokenOp::Div),
            Box::new(lhs_u.clone()),
            Box::new(rhs_u.clone()),
        ))),
    }
}

fn eval_units_reduce_impl(env: &mut Env, units: Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_units_reduce_impl {:?}\r", units);
    }
    match units.clone() {
        Node::BinOp(op, lhs, rhs) => match op {
            Token::Op(TokenOp::Mul) => match (*lhs.clone(), *rhs.clone()) {
                // (g/m)*s => (g*s)/m
                (Node::BinOp(Token::Op(TokenOp::Div), llhs, lrhs), _) => Node::BinOp(
                    Token::Op(TokenOp::Div),
                    Box::new(Node::BinOp(
                        Token::Op(TokenOp::Mul),
                        Box::new(eval_units_reduce_impl(env, *llhs)),
                        Box::new(eval_units_reduce_impl(env, *rhs)),
                    )),
                    Box::new(eval_units_reduce_impl(env, *lrhs)),
                ),
                // g*(s/m) => (g*s)/m
                (_, Node::BinOp(Token::Op(TokenOp::Div), rlhs, rrhs)) => Node::BinOp(
                    Token::Op(TokenOp::Div),
                    Box::new(Node::BinOp(
                        Token::Op(TokenOp::Mul),
                        Box::new(eval_units_reduce_impl(env, *lhs)),
                        Box::new(eval_units_reduce_impl(env, *rlhs)),
                    )),
                    Box::new(eval_units_reduce_impl(env, *rrhs)),
                ),
                (_, _) => Node::BinOp(
                    Token::Op(TokenOp::Mul),
                    Box::new(eval_units_reduce_impl(env, *lhs)),
                    Box::new(eval_units_reduce_impl(env, *rhs)),
                ),
            },
            Token::Op(TokenOp::Div) => match (*lhs.clone(), *rhs.clone()) {
                // g/(m/s) => (g*s)/m
                (_, Node::BinOp(Token::Op(TokenOp::Div), rlhs, rrhs)) => Node::BinOp(
                    Token::Op(TokenOp::Div),
                    Box::new(Node::BinOp(
                        Token::Op(TokenOp::Mul),
                        Box::new(eval_units_reduce_impl(env, *lhs)),
                        Box::new(eval_units_reduce_impl(env, *rrhs)),
                    )),
                    Box::new(eval_units_reduce_impl(env, *rlhs)),
                ),
                // (g/m)/s => g/(m*s)
                (Node::BinOp(Token::Op(TokenOp::Div), llhs, lrhs), _) => Node::BinOp(
                    Token::Op(TokenOp::Mul),
                    Box::new(eval_units_reduce_impl(env, *llhs)),
                    Box::new(Node::BinOp(
                        Token::Op(TokenOp::Div),
                        Box::new(eval_units_reduce_impl(env, *lrhs)),
                        Box::new(eval_units_reduce_impl(env, *rhs)),
                    )),
                ),
                _ => Node::BinOp(
                    Token::Op(TokenOp::Div),
                    Box::new(eval_units_reduce_impl(env, *lhs)),
                    Box::new(eval_units_reduce_impl(env, *rhs)),
                ),
            },
            Token::Op(TokenOp::Caret) => {
                if let Node::Num(rhs_n, _) = *rhs {
                    if rhs_n == 2 {
                        // m^2 => m*m
                        Node::BinOp(
                            Token::Op(TokenOp::Mul),
                            Box::new(eval_units_reduce_impl(env, *lhs.clone())),
                            Box::new(eval_units_reduce_impl(env, *lhs)),
                        )
                    } else {
                        // m^n => m*m^(n-1)
                        Node::BinOp(
                            Token::Op(TokenOp::Mul),
                            Box::new(eval_units_reduce_impl(
                                env,
                                Node::BinOp(
                                    Token::Op(TokenOp::Caret),
                                    lhs.clone(),
                                    Box::new(Node::Num(rhs_n - 1, Box::new(Node::None))),
                                ),
                            )),
                            Box::new(eval_units_reduce_impl(env, *lhs)),
                        )
                    }
                } else {
                    Node::None
                }
            }
            _ => units,
        },
        _ => units,
    }
}

fn eval_units_reduce(env: &mut Env, original: Node) -> Node {
    if env.is_debug() {
        eprintln!("eval_units_reduce {:?}\r", original);
    }
    match original.clone() {
        Node::Num(n, units) => {
            if let Node::Units(u) = *units {
                Node::Num(
                    n,
                    Box::new(Node::Units(Box::new(eval_units_reduce_impl(env, *u)))),
                )
            } else {
                original
            }
        }
        Node::FNum(f, units) => {
            if let Node::Units(u) = *units {
                Node::FNum(
                    f,
                    Box::new(Node::Units(Box::new(eval_units_reduce_impl(env, *u)))),
                )
            } else {
                original
            }
        }
        Node::CNum(n, units) => {
            if let Node::Units(u) = *units {
                Node::CNum(
                    n,
                    Box::new(Node::Units(Box::new(eval_units_reduce_impl(env, *u)))),
                )
            } else {
                original
            }
        }
        _ => original,
    }
}

fn eval_unit_prefix(env: &mut Env, units: &Node) -> (Node, bool) {
    if env.is_debug() {
        eprintln!("eval_unit_prefix {:?}\r", units);
    }
    match units {
        Node::Var(Token::Ident(unit_str)) => match unit_str.as_str() {
            // TODO: add more unit conversion
            "km" => (
                Node::FNum(
                    1000.0,
                    Box::new(Node::Units(Box::new(Node::Var(Token::Ident(
                        "m".to_owned(),
                    ))))),
                ),
                false,
            ),
            "mm" => (
                Node::FNum(
                    0.001,
                    Box::new(Node::Units(Box::new(Node::Var(Token::Ident(
                        "m".to_owned(),
                    ))))),
                ),
                false,
            ),
            _ => (Node::Num(1, Box::new(units.clone())), true),
        },
        Node::BinOp(op, lhs, rhs) => {
            let (left_node, final_left) = eval_unit_prefix(env, lhs);
            let (right_node, final_right) = eval_unit_prefix(env, rhs);
            let new_node = eval_binop(
                env,
                &Node::BinOp(op.clone(), Box::new(left_node), Box::new(right_node)),
            );
            (new_node.unwrap(), final_left && final_right)
        }
        Node::Num(_n, _u) => (units.clone(), true),
        Node::FNum(_n, _u) => (units.clone(), true),
        _ => (Node::Num(1, Box::new(units.clone())), true),
    }
}

fn eval_num(env: &mut Env, node: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval_num {:?}\r", node);
    }
    match node {
        Node::Num(n, u) => {
            if let Node::Units(units) = &**u {
                let (new_node, is_final) = eval_unit_prefix(env, units);
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
                let (new_node, is_final) = eval_unit_prefix(env, units);
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
        let lhs = eval(env, lhs)?;
        let rhs = eval(env, rhs)?;
        match tok {
            Token::Op(TokenOp::Plus) => {
                if let Node::Num(nl, _) = lhs {
                    if let Node::Num(nr, units) = rhs {
                        return Ok(Node::Num(nl + nr, units));
                    }
                }
                if let Node::CNum(_, ref units) = lhs {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? + eval_cvalue(env, &rhs)?,
                        units.clone(),
                    ));
                }
                if let Node::CNum(_, ref units) = rhs {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? + eval_cvalue(env, &rhs)?,
                        units.clone(),
                    ));
                }
                return Ok(Node::FNum(
                    eval_fvalue(env, &lhs)? + eval_fvalue(env, &rhs)?,
                    Box::new(Node::Units(Box::new(Node::None))),
                ));
            }
            Token::Op(TokenOp::Minus) => {
                if let Node::Num(nl, _) = lhs {
                    if let Node::Num(nr, units) = rhs {
                        return Ok(Node::Num(nl - nr, units));
                    }
                }
                if let Node::CNum(_, ref units) = lhs {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? - eval_cvalue(env, &rhs)?,
                        units.clone(),
                    ));
                }
                if let Node::CNum(_, ref units) = rhs {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? - eval_cvalue(env, &rhs)?,
                        units.clone(),
                    ));
                }
                return Ok(Node::FNum(
                    eval_fvalue(env, &lhs)? - eval_fvalue(env, &rhs)?,
                    Box::new(Node::Units(Box::new(Node::None))),
                ));
            }
            Token::Op(TokenOp::Mul) => {
                if let Node::Num(nl, ref lhs_u) = lhs {
                    if let Node::Num(nr, ref rhs_u) = rhs {
                        let units = eval_units_mul(env, lhs_u, rhs_u);
                        if let Node::Units(_) = units {
                            return Ok(Node::Num(nl * nr, Box::new(units)));
                        } else {
                            return Ok(Node::Num(nl * nr, Box::new(Node::Units(Box::new(units)))));
                        }
                    } else if let Node::FNum(fr, ref rhs_u) = rhs {
                        let units = eval_units_mul(env, lhs_u, rhs_u);
                        if let Node::Units(_) = units {
                            return Ok(Node::FNum(nl as f64 * fr, Box::new(units)));
                        } else {
                            return Ok(Node::FNum(
                                nl as f64 * fr,
                                Box::new(Node::Units(Box::new(units))),
                            ));
                        }
                    }
                } else if let Node::FNum(fl, ref lhs_u) = lhs {
                    if let Node::Num(nr, ref rhs_u) = rhs {
                        let units = eval_units_mul(env, lhs_u, rhs_u);
                        if let Node::Units(_) = units {
                            return Ok(Node::FNum(fl * nr as f64, Box::new(units)));
                        } else {
                            return Ok(Node::FNum(
                                fl * nr as f64,
                                Box::new(Node::Units(Box::new(units))),
                            ));
                        }
                    } else if let Node::FNum(fr, ref rhs_u) = rhs {
                        let units = eval_units_mul(env, lhs_u, rhs_u);
                        if let Node::Units(_) = units {
                            return Ok(Node::FNum(fl * fr, Box::new(units)));
                        } else {
                            return Ok(Node::FNum(fl * fr, Box::new(Node::Units(Box::new(units)))));
                        }
                    }
                }
                if let Node::CNum(_, ref units) = lhs {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? * eval_cvalue(env, &rhs)?,
                        units.clone(),
                    ));
                }
                if let Node::CNum(_, ref units) = rhs {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? * eval_cvalue(env, &rhs)?,
                        units.clone(),
                    ));
                }
            }
            Token::Op(TokenOp::Div) => {
                if let Node::Num(nl, ref lhs_u) = lhs {
                    if let Node::Num(nr, ref rhs_u) = rhs {
                        let units = eval_units_div(env, lhs_u, rhs_u);
                        if nr == 0 {
                            return Ok(Node::FNum(std::f64::INFINITY, Box::new(units)));
                        }
                        if let Node::Units(_) = units {
                            return Ok(Node::Num(nl / nr, Box::new(units)));
                        } else {
                            return Ok(Node::Num(nl / nr, Box::new(Node::Units(Box::new(units)))));
                        }
                    } else if let Node::FNum(fr, ref rhs_u) = rhs {
                        let units = eval_units_div(env, lhs_u, rhs_u);
                        if let Node::Units(_) = units {
                            return Ok(Node::FNum(nl as f64 / fr, Box::new(units)));
                        } else {
                            return Ok(Node::FNum(
                                nl as f64 / fr,
                                Box::new(Node::Units(Box::new(units))),
                            ));
                        }
                    }
                } else if let Node::FNum(fl, ref lhs_u) = lhs {
                    if let Node::Num(nr, ref rhs_u) = rhs {
                        let units = eval_units_div(env, lhs_u, rhs_u);
                        if let Node::Units(_) = units {
                            return Ok(Node::FNum(fl / nr as f64, Box::new(units)));
                        } else {
                            return Ok(Node::FNum(
                                fl / nr as f64,
                                Box::new(Node::Units(Box::new(units))),
                            ));
                        }
                    } else if let Node::FNum(fr, ref rhs_u) = rhs {
                        let units = eval_units_div(env, lhs_u, rhs_u);
                        if let Node::Units(_) = units {
                            return Ok(Node::FNum(fl / fr, Box::new(units)));
                        } else {
                            return Ok(Node::FNum(fl / fr, Box::new(Node::Units(Box::new(units)))));
                        }
                    }
                }
                if let Node::CNum(_, ref units) = lhs {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? / eval_cvalue(env, &rhs)?,
                        units.clone(),
                    ));
                }
                if let Node::CNum(_, ref units) = rhs {
                    return Ok(Node::CNum(
                        eval_cvalue(env, &lhs)? / eval_cvalue(env, &rhs)?,
                        units.clone(),
                    ));
                }
            }
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
    }
}

fn eval(env: &mut Env, n: &Node) -> Result<Node, MyError> {
    if env.is_debug() {
        eprintln!("eval {:?}\r", n);
    }
    let result = do_eval(env, n)?;
    let result = eval_units_reduce(env, result);
    match result {
        Node::Num(_, _) | Node::FNum(_, _) | Node::CNum(_, _) => Ok(result),
        Node::Command(_, _, _) => Ok(result),
        Node::None => Ok(result),
        _ => eval(env, &result),
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
        let n = parse(env, &(lexer(input.to_owned())).unwrap()).unwrap();
        let n = eval(env, &n).unwrap();
        format!("{:?}", n)
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

        assert_eq!(
            eval_as_string(&mut env, "1+2"),
            "Num(3, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1+2*3"),
            "Num(7, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1*2+3"),
            "Num(5, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1+2+3"),
            "Num(6, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2)*3"),
            "Num(9, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "-2"),
            "Num(-2, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "-9223372036854775807"),
            "Num(-9223372036854775807, Units(None))".to_owned()
        );
        assert!(((eval_as_f64(&mut env, "1.1+2.2") - 3.3).abs()) < 1e-10);
        assert_eq!(
            eval_as_string(&mut env, "-(2+3)"),
            "Num(-5, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "+(2+3)"),
            "Num(5, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1.0+2"),
            "FNum(3.0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1+2.0"),
            "FNum(3.0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2.0)*3"),
            "FNum(9.0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "pi"),
            "FNum(3.141592653589793, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2k*3u"),
            "FNum(0.006, Units(None))".to_owned()
        );

        assert_eq!(
            eval_as_string(&mut env, "5//5"),
            "FNum(2.5, Units(None))".to_owned()
        );

        assert_eq!(
            eval_as_string(&mut env, "5*inch2mm"),
            "FNum(127.0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "5*feet2mm"),
            "FNum(1524.0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "5*oz2g"),
            "FNum(141.7475, Units(None))".to_owned()
        );

        assert!((eval_as_f64(&mut env, "sin(0.0)")).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "cos(pi/2)")).abs() < 1e-10);
        assert_eq!(
            eval_as_string(&mut env, "sin(0)"),
            "FNum(0.0, Units(None))".to_owned()
        );
        assert!((eval_as_f64(&mut env, "sin(pi)").abs()) < 1e-10);
        assert!(((eval_as_f64(&mut env, "sin(pi/2)") - 1.0).abs()) < 1e-10);
        assert!(((eval_as_f64(&mut env, "abs(-2)") - 2.0).abs()) < 1e-10);
        assert_eq!(
            eval_as_string(&mut env, "sin(0)"),
            "FNum(0.0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1%3"),
            "Num(1, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2%3"),
            "Num(2, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "3%3"),
            "Num(0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "3.0%3"),
            "Num(0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1/3"),
            "Num(0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "3/3"),
            "Num(1, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "3.0/2"),
            "FNum(1.5, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "ave(1,2,3)"),
            "FNum(2.0, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "max(1,2,3)"),
            "FNum(3.0, Units(None))".to_owned()
        );
        eval_as_string(&mut env, "a=1");
        assert_eq!(
            eval_as_string(&mut env, "a"),
            "Num(1, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2^3"),
            "Num(8, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2^3^4"),
            "Num(2417851639229258349412352, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2^-0.5"),
            "FNum(0.7071067811865476, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1+2i"),
            "CNum(Complex { re: 1.0, im: 2.0 }, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2i) - (3-5i)"),
            "CNum(Complex { re: -2.0, im: 7.0 }, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2i) * (3-5i)"),
            "CNum(Complex { re: 13.0, im: 1.0 }, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "(1+2i) / (1-1.0i)"),
            "CNum(Complex { re: -0.5, im: 1.5 }, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2 // 2i"),
            "CNum(Complex { re: 1.0, im: 1.0 }, Units(None))".to_owned()
        );
        assert!((eval_as_complex64(&mut env, "exp(i*pi)").re + 1.0).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "exp(i*pi)").im).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "i^i").re - 0.20787957635076193).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "i^i").im).abs() < 1e-10);
        assert_eq!(
            eval_as_string(&mut env, "-pi"),
            "FNum(-3.141592653589793, Units(None))".to_owned()
        );
        eval_as_string(&mut env, "defun double 2*_1");
        assert_eq!(
            eval_as_string(&mut env, "double(2)"),
            "Num(4, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "double(double(2))"),
            "Num(8, Units(None))".to_owned()
        );
        eval_as_string(&mut env, "defun add _1+_2");
        assert_eq!(
            eval_as_string(&mut env, "add(2,3)"),
            "Num(5, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "add(2,a)"),
            "Num(3, Units(None))".to_owned()
        );
        eval_as_string(&mut env, "defun plus_a a+_1");
        eval_as_string(&mut env, "a=5");
        assert_eq!(
            eval_as_string(&mut env, "plus_a(8)"),
            "Num(13, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "abs(-2)"),
            "FNum(2.0, Units(None))".to_owned()
        );
        assert!((eval_as_f64(&mut env, "abs(-2.5)") - 2.5).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "abs(1+i)") - 1.4142135623730951).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "sqrt(2)") - 1.4142135623730951).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "sqrt(2i)").re - 1.0).abs() < 1e-10);
        assert!((eval_as_complex64(&mut env, "sqrt(2i)").im - 1.0).abs() < 1e-10);
        assert!((eval_as_f64(&mut env, "arg(1+i)") - 0.7853981633974483).abs() < 1e-10);
        eval_as_string(&mut env, "b=a");
        eval_as_string(&mut env, "c=b");
        assert_eq!(
            eval_as_string(&mut env, "c"),
            "Num(5, Units(None))".to_owned()
        );
        eval_as_string(&mut env, "d=c");
        eval_as_string(&mut env, "ee=d+c");
        assert_eq!(
            eval_as_string(&mut env, "ee"),
            "Num(10, Units(None))".to_owned()
        );

        // assignment
        eval_as_string(&mut env, "aa=1");
        eval_as_string(&mut env, "bb=aa");
        eval_as_string(&mut env, "aa=2");
        // assert_eq!(eval_as_string(&mut env, "bb"), "Num(2)".to_owned());    // assign to bb is binded to AST of aa
        assert_eq!(
            eval_as_string(&mut env, "bb"),
            "Num(1, Units(None))".to_owned()
        ); // assign to bb is ?binded to value of aa at the assigned time

        // divided by zero
        assert_eq!(
            eval_as_string(&mut env, "1/0"),
            "FNum(inf, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1.0/0.0"),
            "FNum(inf, Units(None))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1/(0.0+0.0i)"),
            "CNum(Complex { re: NaN, im: NaN }, Units(None))".to_owned()
        );
    }

    #[test]
    fn test_eval_units() {
        let mut env = Env::new();
        env.built_in();

        assert_eq!(
            eval_as_string(&mut env, "1[m]"),
            "Num(1, Units(Var(Ident(\"m\"))))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1[1/m]"),
            "Num(1, Units(BinOp(Op(Div), Num(1, Units(None)), Var(Ident(\"m\")))))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "2[m*s]"),
            "Num(2, Units(BinOp(Op(Mul), Var(Ident(\"m\")), Var(Ident(\"s\")))))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "1[m]*2[s]"),
            "Num(2, Units(BinOp(Op(Mul), Var(Ident(\"m\")), Var(Ident(\"s\")))))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "3[m/s]"),
            "Num(3, Units(BinOp(Op(Div), Var(Ident(\"m\")), Var(Ident(\"s\")))))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "6[m]/2[s]"),
            "Num(3, Units(BinOp(Op(Div), Var(Ident(\"m\")), Var(Ident(\"s\")))))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "6[m*m]/2[s]"),
            "Num(3, Units(BinOp(Op(Div), BinOp(Op(Mul), Var(Ident(\"m\")), Var(Ident(\"m\"))), Var(Ident(\"s\")))))".to_owned()
        );
        assert_eq!(
            eval_as_string(&mut env, "3*2[m]"),
            eval_as_string(&mut env, "6[m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "2[m]*3"),
            eval_as_string(&mut env, "6[m]")
        );
        // unit reduction
        assert_eq!(
            eval_as_string(&mut env, "6[m^2]/3[s]"),
            eval_as_string(&mut env, "6[m*m]/3[s]")
        );
        assert_eq!(
            eval_as_string(&mut env, "18[m^3]/2[s]"),
            eval_as_string(&mut env, "18[m*m*m]/2[s]")
        );
        assert_eq!(
            eval_as_string(&mut env, "6[g]/2[m/s]"),
            eval_as_string(&mut env, "3[g*s/m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "6[g/m]*2[s]"),
            eval_as_string(&mut env, "12[g*s/m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "2[g]*6[s/m]"),
            eval_as_string(&mut env, "12[g*s/m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "6[g/m]/2[s]"),
            eval_as_string(&mut env, "3[g/m/s]")
        );
        // unit expand
        // TODO: to be added more conversion
        assert_eq!(
            eval_as_string(&mut env, "6[km]"),
            eval_as_string(&mut env, "6000.0[m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "6000[mm]"),
            eval_as_string(&mut env, "6.0[m]")
        );
        assert_eq!(
            eval_as_string(&mut env, "0.001[1/m]"),
            eval_as_string(&mut env, "1.0[1/km]"),
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
