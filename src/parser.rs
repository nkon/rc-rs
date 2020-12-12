use super::*;

// <assign>  ::= <var> '=' <expr>
// <expr>    ::= <mul> ( '+' <mul> | '-' <mul> )*
// <mul>     ::= <exp> ( '*' <exp> | '/' <exp>)*
// <exp>     ::= <unary> '^' <exp> | <unary>
// <unary>   ::= <primary> | '-' <primary> | '+' <primary>
// <primary> ::= <num> | '(' <expr> ')' | <var> | <func> '(' <expr>* ',' ')'
// <num>     ::= <num> | <num> <postfix>

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    None,
    Num(i128),
    FNum(f64),
    CNum(Complex64),
    Unary(Token, Box<Node>),
    BinOp(Token, Box<Node>, Box<Node>),
    Var(Token),
    Func(Token, Vec<Node>),
    Command(Token, Vec<Token>, String),
}

macro_rules! tok_check_index {
    ($tok:expr, $i:expr) => {
        if $tok.len() <= $i {
            return Err(MyError::ParseError(format!(
                "unexpected end of input: {} {}: {:?}",
                file!(),
                line!(),
                $tok
            )));
        }
    };
}

fn num(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("num {:?} {}\r", tok, i);
    }
    tok_check_index!(tok, i);

    let mut f_postfix = 1.0;
    let mut is_complex = false;
    let mut has_postfix = false;
    if (i + 1) < tok.len() {
        if let Token::Ident(id) = &tok[i + 1] {
            match id.as_ref() {
                "k" => {
                    f_postfix = 1000.0;
                    has_postfix = true;
                }
                "M" => {
                    f_postfix = 1_000_000.0;
                    has_postfix = true;
                }
                "G" => {
                    f_postfix = 1_000_000_000.0;
                    has_postfix = true;
                }
                "T" => {
                    f_postfix = 1_000_000_000_000.0;
                    has_postfix = true;
                }
                "m" => {
                    f_postfix = 0.001;
                    has_postfix = true;
                }
                "u" => {
                    f_postfix = 0.000_001;
                    has_postfix = true;
                }
                "n" => {
                    f_postfix = 0.000_000_001;
                    has_postfix = true;
                }
                "p" => {
                    f_postfix = 0.000_000_000_001;
                    has_postfix = true;
                }
                "i" | "j" => {
                    is_complex = true;
                }
                _ => {}
            }
        }
    }
    match tok[i] {
        Token::Num(n) => {
            if has_postfix {
                if is_complex {
                    Ok((Node::CNum(Complex64::new(0.0, n as f64 * f_postfix)), i + 3))
                } else {
                    Ok((Node::FNum(n as f64 * f_postfix), i + 2))
                }
            } else if is_complex {
                Ok((Node::CNum(Complex64::new(0.0, n as f64)), i + 2))
            } else {
                Ok((Node::Num(n), i + 1))
            }
        }
        Token::FNum(n) => {
            if has_postfix {
                if is_complex {
                    Ok((Node::CNum(Complex64::new(0.0, n * f_postfix)), i + 3))
                } else {
                    Ok((Node::FNum(n * f_postfix), i + 2))
                }
            } else if is_complex {
                Ok((Node::CNum(Complex64::new(0.0, n)), i + 2))
            } else {
                Ok((Node::FNum(n), i + 1))
            }
        }
        _ => Ok((Node::None, i)),
    }
}

// TODO: func, cmd, var -> separate function
fn primary(env: &mut Env, tok: &[Token], index: usize) -> Result<(Node, usize), MyError> {
    let mut i = index;
    if env.is_debug() {
        eprintln!("primary {:?} {}\r", tok, i);
    }
    tok_check_index!(tok, i);

    match &tok[i] {
        Token::Op(TokenOp::ParenLeft) => {
            let (ex, i) = expr(env, tok, i + 1)?;
            if tok[i] != Token::Op(TokenOp::ParenRight) {
                Err(MyError::ParseError(format!(
                    "')' not found: {:?} {}",
                    tok, i
                )))
            } else {
                Ok((ex, i + 1))
            }
        }
        Token::Ident(id) => {
            if let Some(_constant) = env.is_const(id.as_str()) {
                return Ok((Node::Var(Token::Ident(id.clone())), i + 1));
            } else if id == "i" || id == "j" {
                return Ok((Node::CNum(Complex64::new(0.0, 1.0)), i + 1));
            } else if let Some(func_tuple) = env.is_func(id.as_str()) {
                let mut params = Vec::new();
                if tok.len() <= (i + 1) {
                    return Err(MyError::ParseError(format!(
                        "function has no parameter: {:?} {}",
                        tok, i
                    )));
                } else if tok[i + 1] == Token::Op(TokenOp::ParenLeft) {
                    i += 2;
                    while i < tok.len() {
                        if tok[i] == Token::Op(TokenOp::ParenRight) {
                            if func_tuple.1 != 0 && func_tuple.1 != params.len() {
                                return Err(MyError::ParseError(format!(
                                    "function parameter number: {:?} {}",
                                    tok, i
                                )));
                            }
                            return Ok((Node::Func(Token::Ident(id.clone()), params), i + 1));
                        } else if tok[i] == Token::Op(TokenOp::Comma) {
                            i += 1;
                            continue;
                        } else if let Ok((t, j)) = expr(env, tok, i) {
                            i = j;
                            params.push(t);
                        } else {
                            return Err(MyError::ParseError(format!(
                                "function parameter: {:?} {}",
                                tok, i
                            )));
                        }
                    }
                    if tok.len() <= i {
                        return Err(MyError::ParseError(format!(
                            "function has no ')': {:?} {}",
                            tok, i
                        )));
                    }
                } else {
                    return Err(MyError::ParseError(format!(
                        "function has no '(': {:?} {}",
                        tok, i
                    )));
                }
            } else if let Some(cmd_tuple) = env.is_cmd(id.as_str()) {
                let mut params = Vec::new();
                if tok.len() <= (i + 1) {
                    return Err(MyError::ParseError(format!(
                        "command has no parameter: {:?} {}",
                        tok, i
                    )));
                } else if tok[i + 1] == Token::Op(TokenOp::ParenLeft) {
                    i += 2;
                    while i < tok.len() {
                        if tok[i] == Token::Op(TokenOp::ParenRight) {
                            if cmd_tuple.1 != 0 && cmd_tuple.1 != params.len() {
                                return Err(MyError::ParseError(format!(
                                    "command parameter number: {:?} {}",
                                    tok, i
                                )));
                            }
                            return Ok((
                                Node::Command(Token::Ident(id.clone()), params, "".to_string()),
                                i + 1,
                            ));
                        } else if tok[i] == Token::Op(TokenOp::Comma) {
                            i += 1;
                            continue;
                        } else {
                            params.push(tok[i].clone());
                            i += 1;
                            continue;
                        }
                    }
                    if tok.len() <= i {
                        return Err(MyError::ParseError(format!(
                            "command has no ')': {:?} {}",
                            tok, i
                        )));
                    }
                } else {
                    return Err(MyError::ParseError(format!(
                        "command has no '(': {:?} {}",
                        tok, i
                    )));
                }
            } else if env.is_variable(id).is_some() {
                return Ok((Node::Var(Token::Ident(id.clone())), i + 1));
            } else {
                env.new_variable(id.clone());
                return Ok((Node::Var(Token::Ident(id.clone())), i + 1));
            }
            Ok((Node::None, i))
        }
        _ => num(env, tok, i),
    }
}

fn unary(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("unary {:?} {}\r", tok, i);
    }
    tok_check_index!(tok, i);

    let tok_orig = tok[i].clone();
    match tok[i] {
        Token::Op(TokenOp::Minus) | Token::Op(TokenOp::Plus) => {
            let (rhs, i) = primary(env, tok, i + 1)?;
            Ok((Node::Unary(tok_orig, Box::new(rhs)), i))
        }
        _ => primary(env, tok, i),
    }
}

fn exp(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("exp {:?} {}\r", tok, i);
    }
    tok_check_index!(tok, i);

    let (lhs, mut i) = unary(env, tok, i)?;
    if tok.len() <= i {
        return Ok((lhs, i));
    }
    if tok[i] == Token::Op(TokenOp::Hat) {
        let (rhs, j) = exp(env, tok, i + 1)?;
        i = j;
        Ok((
            Node::BinOp(Token::Op(TokenOp::Hat), Box::new(lhs), Box::new(rhs)),
            i,
        ))
    } else {
        Ok((lhs, i))
    }
}

fn mul(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("mul {:?} {}\r", tok, i);
    }
    tok_check_index!(tok, i);

    let (mut lhs, mut i) = exp(env, tok, i)?;
    loop {
        if tok.len() <= i {
            return Ok((lhs, i));
        }
        let tok_orig = tok[i].clone();
        match tok[i] {
            Token::Op(TokenOp::Mul)
            | Token::Op(TokenOp::Div)
            | Token::Op(TokenOp::Mod)
            | Token::Op(TokenOp::Para) => {
                let (rhs, j) = exp(env, tok, i + 1)?;
                i = j;
                lhs = Node::BinOp(tok_orig, Box::new(lhs), Box::new(rhs))
            }
            _ => {
                return Ok((lhs, i));
            }
        }
    }
}

fn expr(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("expr {:?} {}\r", tok, i);
    }
    tok_check_index!(tok, i);

    let (mut lhs, mut i) = mul(env, tok, i)?;
    loop {
        if tok.len() <= i {
            return Ok((lhs, i));
        }
        let tok_orig = tok[i].clone();
        match tok[i] {
            Token::Op(TokenOp::Plus) | Token::Op(TokenOp::Minus) => {
                let (rhs, j) = mul(env, tok, i + 1)?;
                i = j;
                lhs = Node::BinOp(tok_orig, Box::new(lhs), Box::new(rhs));
            }
            _ => {
                return Ok((lhs, i));
            }
        }
    }
}

fn assign(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("assign {:?} {}\r", tok, i);
    }
    tok_check_index!(tok, i);

    let (lhs, i) = expr(env, tok, i)?;
    if i < tok.len() && tok[i] == Token::Op(TokenOp::Equal) {
        let (rhs, i) = expr(env, tok, i + 1)?;
        Ok((
            Node::BinOp(Token::Op(TokenOp::Equal), Box::new(lhs), Box::new(rhs)),
            i,
        ))
    } else {
        Ok((lhs, i))
    }
}

/// Input: `&Vec<Token>`   output of `lexer()`
/// Output: `Node()`       AST as the parser result
///
/// # Examples
/// ```
/// use rc::lexer;
/// use rc::Token;
/// use rc::parse;
/// use rc::Env;
/// let mut env = Env::new();
/// env.built_in();
/// assert_eq!(format!("{:?}", parse(&mut env, &(lexer("1+2".to_string()).unwrap())).unwrap()),"BinOp(Op(Plus), Num(1), Num(2))");
/// ```
// TODO: user define function
// TODO: multiple expression
pub fn parse(env: &mut Env, tok: &[Token]) -> Result<Node, MyError> {
    let (node, i) = assign(env, &tok, 0)?;
    if i < tok.len() {
        Err(MyError::ParseError(format!("token left: {:?} {}", tok, i)))
    } else {
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_as_string(env: &mut Env, input: &str) -> String {
        match parse(env, &(lexer(input.to_string())).unwrap()) {
            Ok(n) => {
                format!("{:?}", n)
            }
            Err(e) => {
                format!("{}", e)
            }
        }
    }

    #[test]
    fn test_parser() {
        let mut env = Env::new();
        env.built_in();

        assert_eq!(
            parse_as_string(&mut env, "1+2"),
            "BinOp(Op(Plus), Num(1), Num(2))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1-2"),
            "BinOp(Op(Minus), Num(1), Num(2))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1+-2"),
            "BinOp(Op(Plus), Num(1), Unary(Op(Minus), Num(2)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*2"),
            "BinOp(Op(Mul), Num(1), Num(2))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*2+3"),
            "BinOp(Op(Plus), BinOp(Op(Mul), Num(1), Num(2)), Num(3))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*(2+3)"),
            "BinOp(Op(Mul), Num(1), BinOp(Op(Plus), Num(2), Num(3)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1+2+3"),
            "BinOp(Op(Plus), BinOp(Op(Plus), Num(1), Num(2)), Num(3))"
        );
        assert_eq!(
            parse_as_string(&mut env, "(1+2)+3"),
            "BinOp(Op(Plus), BinOp(Op(Plus), Num(1), Num(2)), Num(3))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*2*3"),
            "BinOp(Op(Mul), BinOp(Op(Mul), Num(1), Num(2)), Num(3))"
        );
        assert_eq!(
            parse_as_string(&mut env, "(1*2)*3"),
            "BinOp(Op(Mul), BinOp(Op(Mul), Num(1), Num(2)), Num(3))"
        );
        assert_eq!(
            parse_as_string(&mut env, "-(2+3)"),
            "Unary(Op(Minus), BinOp(Op(Plus), Num(2), Num(3)))"
        );
        assert_eq!(parse_as_string(&mut env, "pi"), "Var(Ident(\"pi\"))");
        assert_eq!(
            parse_as_string(&mut env, "2.0*pi"),
            "BinOp(Op(Mul), FNum(2.0), Var(Ident(\"pi\")))"
        );
        assert_eq!(parse_as_string(&mut env, "2k"), "FNum(2000.0)");
        assert_eq!(
            parse_as_string(&mut env, "2u*pi"),
            "BinOp(Op(Mul), FNum(0.000002), Var(Ident(\"pi\")))"
        );
        assert_eq!(
            parse_as_string(&mut env, "2*sin(0.5*pi)"),
            "BinOp(Op(Mul), Num(2), Func(Ident(\"sin\"), [BinOp(Op(Mul), FNum(0.5), Var(Ident(\"pi\")))]))"
        );
        assert_eq!(
            parse_as_string(&mut env, "a=1"),
            "BinOp(Op(Equal), Var(Ident(\"a\")), Num(1))"
        );
        assert_eq!(
            parse_as_string(&mut env, "2^3"),
            "BinOp(Op(Hat), Num(2), Num(3))"
        );
        assert_eq!(
            parse_as_string(&mut env, "2^3^4"),
            "BinOp(Op(Hat), Num(2), BinOp(Op(Hat), Num(3), Num(4)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1+2i"),
            "BinOp(Op(Plus), Num(1), CNum(Complex { re: 0.0, im: 2.0 }))"
        );
        assert_eq!(
            parse_as_string(&mut env, "i"),
            "CNum(Complex { re: 0.0, im: 1.0 })"
        );
    }

    #[test]
    fn test_parser_error() {
        let mut env = Env::new();
        env.built_in();

        if let Ok(_) = parse(&mut env, &(lexer("2*sin(1, 2)".to_string())).unwrap()) {
            assert!(false);
        }
        if let Ok(_) = parse(&mut env, &(lexer("sin(".to_string())).unwrap()) {
            assert!(false);
        }
        if let Ok(_) = parse(&mut env, &(lexer("sin()".to_string())).unwrap()) {
            assert!(false);
        }
        if let Ok(_) = parse(&mut env, &(lexer("1+2+".to_string())).unwrap()) {
            assert!(false);
        }
        if let Ok(_) = parse(&mut env, &(lexer("sin".to_string())).unwrap()) {
            assert!(false);
        }
    }
}
