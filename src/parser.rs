use super::*;

// <expr>    ::= <mul> ( '+' <mul> | '-' <mul> )*
// <mul>     ::= <unary> ( '*' <unary> | '/' <unary>)*
// <unary>   ::= <primary> | '-' <primary> | '+' <primary>
// <primary> ::= <num> | '(' <expr> ')' | <var> | <func> '(' <expr>* ',' ')'
// <num>     ::= <num> | <num> <postfix>

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    None,
    Num(i128),
    FNum(f64),
    Unary(Token, Box<Node>),
    BinOp(Token, Box<Node>, Box<Node>),
    Var(Token),
    Func(Token, Vec<Node>),
}

fn num(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("num {:?} {}\r", tok, i);
    }
    if tok.len() <= i {
        return Err(MyError::ParseError(format!(
            "unexpected end of input: {} {}: {:?}",
            file!(),
            line!(),
            tok
        )));
    }
    let mut f_postfix = 1.0;
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
                _ => {}
            }
        }
    }
    match tok[i] {
        Token::Num(n) => {
            if has_postfix {
                Ok((Node::FNum(n as f64 * f_postfix), i + 2))
            } else {
                Ok((Node::Num(n), i + 1))
            }
        }
        Token::FNum(n) => {
            if has_postfix {
                Ok((Node::FNum(n * f_postfix), i + 2))
            } else {
                Ok((Node::FNum(n), i + 1))
            }
        }
        _ => Ok((Node::None, i)),
    }
}

fn primary(env: &mut Env, tok: &[Token], index: usize) -> Result<(Node, usize), MyError> {
    let mut i = index;
    if env.is_debug() {
        eprintln!("primary {:?} {}\r", tok, i);
    }
    if tok.len() <= i {
        return Err(MyError::ParseError(format!(
            "unexpected end of input: {} {}: {:?}",
            file!(),
            line!(),
            tok
        )));
    }
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
            } else if let Some(func_tupple) = env.is_func(id.as_str()) {
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
                            if func_tupple.1 != 0 && func_tupple.1 != params.len() {
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
            } else if let Some(cmd_tupple) = env.is_cmd(id.as_str()) {
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
                            if cmd_tupple.1 != 0 && cmd_tupple.1 != params.len() {
                                return Err(MyError::ParseError(format!(
                                    "command parameter number: {:?} {}",
                                    tok, i
                                )));
                            }
                            cmd_tupple.0(env, &params);
                            return Ok((Node::None, i + 1));
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
    if tok.len() <= i {
        return Err(MyError::ParseError(format!(
            "unexpected end of input: {} {}",
            file!(),
            line!()
        )));
    }
    let tok_orig = tok[i].clone();
    match tok[i] {
        Token::Op(TokenOp::Minus) | Token::Op(TokenOp::Plus) => {
            let (rhs, i) = primary(env, tok, i + 1)?;
            Ok((Node::Unary(tok_orig, Box::new(rhs)), i))
        }
        _ => primary(env, tok, i),
    }
}

fn mul(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("mul {:?} {}\r", tok, i);
    }
    if tok.len() <= i {
        return Err(MyError::ParseError(format!(
            "unexpected end of input: {} {}",
            file!(),
            line!()
        )));
    }
    let (mut lhs, mut i) = unary(env, tok, i)?;
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
                if let Ok((rhs, j)) = unary(env, tok, i + 1) {
                    i = j;
                    lhs = Node::BinOp(tok_orig, Box::new(lhs), Box::new(rhs))
                } else {
                    return Err(MyError::ParseError(format!(
                        "Operator '*' '/' '%' requires right side operand. {:?} {}",
                        tok, i
                    )));
                }
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
    if tok.len() <= i {
        return Err(MyError::ParseError(format!(
            "unexpected end of input: {} {}",
            file!(),
            line!()
        )));
    }
    let (mut lhs, mut i) = mul(env, tok, i)?;
    loop {
        if tok.len() <= i {
            return Ok((lhs, i));
        }
        let tok_orig = tok[i].clone();
        match tok[i] {
            Token::Op(TokenOp::Plus) | Token::Op(TokenOp::Minus) => {
                if let Ok((rhs, j)) = mul(env, tok, i + 1) {
                    i = j;
                    lhs = Node::BinOp(tok_orig, Box::new(lhs), Box::new(rhs))
                } else {
                    return Err(MyError::ParseError(format!(
                        "Operator'+'/'-' requires right side operand. {:?} {}",
                        tok, i
                    )));
                }
            }
            _ => {
                return Ok((lhs, i));
            }
        }
    }
}

/// Input: `&Vec<Token>`   output of `lexer()`
/// Output: `Node()`       AST as the paser result
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
// TODO: user define var
// TODO: user devine function
// TODO: multiple expression
pub fn parse(env: &mut Env, tok: &[Token]) -> Result<Node, MyError> {
    let (node, i) = expr(env, &tok, 0)?;
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

        if let Ok(_) = parse(&mut env, &(lexer("ssss".to_string())).unwrap()) {
            assert!(false);
        }
    }
}
