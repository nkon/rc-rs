use super::*;
use std::collections::HashMap;

// <assign>  ::= <var> '=' <expr>
// <expr>    ::= <mul> ( '+' <mul> | '-' <mul> )*
// <mul>     ::= <exp> ( '*' <exp> | '/' <exp>)*
// <exp>     ::= <unary> '^' <exp> | <unary>
// <unary>   ::= <primary> | '-' <primary> | '+' <primary>
// <primary> ::= <num> | '(' <expr> ')' | <var> | <func> '(' <expr>* ',' ')'
// <num>     ::= <num> | <num> <postfix> | <num> <units> | <num> <postfix> <units>
// <units>   ::= '[' <expr> ']'

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    None,
    Units(Box<Node>),
    UnitsFraction(HashMap<String, i32>, HashMap<String, i32>), // Numerator, Denominator ("m" => 2), ("g" => 1)
    Num(i128, Box<Node>),                                      // Num, Units
    FNum(f64, Box<Node>),
    CNum(Complex64, Box<Node>),
    Unary(Token, Box<Node>),            // TokenOp, Operand
    BinOp(Token, Box<Node>, Box<Node>), // TokenOp, LHS, RHS
    Var(Token),                         // Token::Ident
    Func(Token, Vec<Node>),             // Token::Ident, args...
    Command(Token, Vec<Token>, String), // Token::Ident, args..., result-holder
}

fn tok_check_index(tok: &[Token], i: usize) -> Result<(), MyError> {
    if tok.len() <= i {
        Err(MyError::ParseError(format!(
            "unexpected end of input: {:?}",
            tok
        )))
    } else {
        Ok(())
    }
}

fn units(env: &mut Env, tok: &[Token], index: usize) -> Result<(Node, usize), MyError> {
    let mut i = index;
    if env.is_debug() {
        eprintln!("units {:?} {}\r", tok, i);
    }

    if (i + 1) < tok.len() {
        // check token over run
        match &tok[i + 1] {
            Token::Op(TokenOp::SqBracketLeft) => {
                i += 2;
                let mut node = Node::None;
                loop {
                    match &tok[i] {
                        Token::Op(TokenOp::SqBracketRight) => {
                            return Ok((Node::Units(Box::new(node)), i));
                        }
                        _ => {
                            let (new_node, new_index) = expr(env, tok, i)?;
                            i = new_index;
                            node = new_node;
                        }
                    }
                }
            }
            _ => {
                return Ok((Node::Units(Box::new(Node::None)), i));
            }
        }
    }
    Ok((Node::Units(Box::new(Node::None)), i))
}

fn postfix(env: &mut Env, tok: &[Token], index: usize) -> (bool, f64, bool, usize) {
    // has_postfix, scale, is_complex, new_index
    if env.is_debug() {
        eprintln!("postfix {:?} {}\r", tok, index);
    }

    if (index + 1) < tok.len() {
        // check token over run
        if let Token::Ident(id) = &tok[index + 1] {
            // check suffix
            match id.as_ref() {
                "k" => {
                    return (true, 1000.0, false, index + 1);
                }
                "M" => {
                    return (true, 1_000_000.0, false, index + 1);
                }
                "G" => {
                    return (true, 1_000_000_000.0, false, index + 1);
                }
                "T" => {
                    return (true, 1_000_000_000_000.0, false, index + 1);
                }
                "m" => {
                    return (true, 0.001, false, index + 1);
                }
                "u" => {
                    return (true, 0.000_001, false, index + 1);
                }
                "n" => {
                    return (true, 0.000_000_001, false, index + 1);
                }
                "p" => {
                    return (true, 0.000_000_000_001, false, index + 1);
                }
                "i" | "j" => {
                    return (true, 1.0, true, index + 1);
                }
                _ => {
                    return (false, 1.0, false, index);
                }
            }
        } else {
            return (false, 1.0, false, index);
        }
    }
    (false, 1.0, false, index)
}

fn num(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("num {:?} {}\r", tok, i);
    }
    tok_check_index(tok, i)?;

    match tok[i] {
        Token::Num(n) => {
            let (has_postfix, scale, is_complex, index) = postfix(env, tok, i);
            let (units, index) = units(env, tok, index)?;
            if has_postfix {
                if is_complex {
                    Ok((
                        Node::CNum(Complex64::new(0.0, n as f64 * scale), Box::new(units)),
                        index + 1,
                    ))
                } else {
                    Ok((Node::FNum(n as f64 * scale, Box::new(units)), index + 1))
                }
            } else if is_complex {
                Ok((
                    Node::CNum(Complex64::new(0.0, n as f64), Box::new(units)),
                    index + 1,
                ))
            } else {
                Ok((Node::Num(n, Box::new(units)), index + 1))
            }
        }
        Token::FNum(n) => {
            let (_, scale, is_complex, index) = postfix(env, tok, i);
            let (units, index) = units(env, tok, index)?;
            if is_complex {
                Ok((
                    Node::CNum(Complex64::new(0.0, n * scale), Box::new(units)),
                    index + 1,
                ))
            } else {
                Ok((Node::FNum(n * scale, Box::new(units)), index + 1))
            }
        }
        _ => Ok((Node::None, i)),
    }
}

fn func(
    env: &mut Env,
    id: &str,
    param_num: usize,
    tok: &[Token],
    index: usize,
) -> Result<(Node, usize), MyError> {
    let mut i = index;
    let mut params = Vec::new();
    if tok.len() <= (i + 1) {
        return Err(MyError::ParseError(format!(
            "function has no parameter: {:?} {}",
            tok, i
        )));
    }
    if tok[i + 1] == Token::Op(TokenOp::ParenLeft) {
        i += 2;
        while i < tok.len() {
            if tok[i] == Token::Op(TokenOp::ParenRight) {
                if param_num != 0 && param_num != params.len() {
                    return Err(MyError::ParseError(format!(
                        "function parameter number: {:?} {}",
                        tok, i
                    )));
                }
                return Ok((Node::Func(Token::Ident(id.to_owned()), params), i + 1));
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
    }
    Err(MyError::ParseError(format!(
        "function has no '(': {:?} {}",
        tok, i
    )))
}

fn cmd(_env: &Env, id: &str, tok: &[Token], index: usize) -> Result<(Node, usize), MyError> {
    let mut i = index;
    let mut params = Vec::new();
    i += 1;
    while i < tok.len() {
        params.push(tok[i].clone());
        i += 1;
        continue;
    }
    Ok((
        Node::Command(Token::Ident(id.to_owned()), params, "".to_owned()),
        i + 1,
    ))
}

fn primary(env: &mut Env, tok: &[Token], index: usize) -> Result<(Node, usize), MyError> {
    let i = index;
    if env.is_debug() {
        eprintln!("primary {:?} {}\r", tok, i);
    }
    tok_check_index(tok, i)?;

    match &tok[i] {
        Token::Op(TokenOp::ParenLeft) => {
            let (ex, i) = expr(env, tok, i + 1)?;
            tok_check_index(tok, i)?;
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
                Ok((Node::Var(Token::Ident(id.clone())), i + 1))
            } else if let Some(func_tuple) = env.is_func(id.as_str()) {
                func(env, id, func_tuple.1, tok, index)
            } else if let Some(_tokens) = env.is_user_func((*id).clone()) {
                func(env, &(*id).to_owned(), 0, tok, index)
            } else if let Some(_cmd_tuple) = env.is_cmd(id.as_str()) {
                cmd(env, id, tok, index)
            } else if env.is_variable(id).is_some() {
                Ok((Node::Var(Token::Ident(id.clone())), i + 1))
            } else {
                env.new_variable(id.clone());
                Ok((Node::Var(Token::Ident(id.clone())), i + 1))
            }
        }
        _ => num(env, tok, i),
    }
}

fn unary(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("unary {:?} {}\r", tok, i);
    }
    tok_check_index(tok, i)?;

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
    tok_check_index(tok, i)?;

    let (lhs, mut i) = unary(env, tok, i)?;
    if tok.len() <= i {
        return Ok((lhs, i));
    }
    if tok[i] == Token::Op(TokenOp::Caret) {
        let (rhs, j) = exp(env, tok, i + 1)?;
        i = j;
        Ok((
            Node::BinOp(Token::Op(TokenOp::Caret), Box::new(lhs), Box::new(rhs)),
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
    tok_check_index(tok, i)?;

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
    tok_check_index(tok, i)?;

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
    tok_check_index(tok, i)?;

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
/// assert_eq!(format!("{:?}", parse(&mut env, &(lexer("1+2".to_owned()).unwrap())).unwrap()),"BinOp(Op(Plus), Num(1, Units(None)), Num(2, Units(None)))");
/// ```
// TODO: multiple expression
pub fn parse(env: &mut Env, tok: &[Token]) -> Result<Node, MyError> {
    let (node, i) = assign(env, tok, 0)?;
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
        match parse(env, &(lexer(input.to_owned())).unwrap()) {
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
            "BinOp(Op(Plus), Num(1, Units(None)), Num(2, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1-2"),
            "BinOp(Op(Minus), Num(1, Units(None)), Num(2, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1+-2"),
            "BinOp(Op(Plus), Num(1, Units(None)), Unary(Op(Minus), Num(2, Units(None))))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*2"),
            "BinOp(Op(Mul), Num(1, Units(None)), Num(2, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*2+3"),
            "BinOp(Op(Plus), BinOp(Op(Mul), Num(1, Units(None)), Num(2, Units(None))), Num(3, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*(2+3)"),
            "BinOp(Op(Mul), Num(1, Units(None)), BinOp(Op(Plus), Num(2, Units(None)), Num(3, Units(None))))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1+2+3"),
            "BinOp(Op(Plus), BinOp(Op(Plus), Num(1, Units(None)), Num(2, Units(None))), Num(3, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "(1+2)+3"),
            "BinOp(Op(Plus), BinOp(Op(Plus), Num(1, Units(None)), Num(2, Units(None))), Num(3, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*2*3"),
            "BinOp(Op(Mul), BinOp(Op(Mul), Num(1, Units(None)), Num(2, Units(None))), Num(3, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "(1*2)*3"),
            "BinOp(Op(Mul), BinOp(Op(Mul), Num(1, Units(None)), Num(2, Units(None))), Num(3, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "-(2+3)"),
            "Unary(Op(Minus), BinOp(Op(Plus), Num(2, Units(None)), Num(3, Units(None))))"
        );
        assert_eq!(parse_as_string(&mut env, "pi"), "Var(Ident(\"pi\"))");
        assert_eq!(
            parse_as_string(&mut env, "2.0*pi"),
            "BinOp(Op(Mul), FNum(2.0, Units(None)), Var(Ident(\"pi\")))"
        );
        assert_eq!(parse_as_string(&mut env, "2k"), "FNum(2000.0, Units(None))");
        assert_eq!(
            parse_as_string(&mut env, "2u*pi"),
            "BinOp(Op(Mul), FNum(2e-6, Units(None)), Var(Ident(\"pi\")))"
        );
        assert_eq!(
            parse_as_string(&mut env, "2*sin(0.5*pi)"),
            "BinOp(Op(Mul), Num(2, Units(None)), Func(Ident(\"sin\"), [BinOp(Op(Mul), FNum(0.5, Units(None)), Var(Ident(\"pi\")))]))"
        );
        assert_eq!(
            parse_as_string(&mut env, "a=1"),
            "BinOp(Op(Equal), Var(Ident(\"a\")), Num(1, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "2^3"),
            "BinOp(Op(Caret), Num(2, Units(None)), Num(3, Units(None)))"
        );
        assert_eq!(
            parse_as_string(&mut env, "2^3^4"),
            "BinOp(Op(Caret), Num(2, Units(None)), BinOp(Op(Caret), Num(3, Units(None)), Num(4, Units(None))))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1+2i"),
            "BinOp(Op(Plus), Num(1, Units(None)), CNum(Complex { re: 0.0, im: 2.0 }, Units(None)))"
        );
        assert_eq!(parse_as_string(&mut env, "i"), "Var(Ident(\"i\"))");
        // 新しいテストケース
        assert_eq!(
            parse_as_string(&mut env, "3+4*2/(1-5)^2^3"),
            "BinOp(Op(Plus), Num(3, Units(None)), BinOp(Op(Div), BinOp(Op(Mul), Num(4, Units(None)), Num(2, Units(None))), BinOp(Op(Caret), BinOp(Op(Minus), Num(1, Units(None)), Num(5, Units(None))), BinOp(Op(Caret), Num(2, Units(None)), Num(3, Units(None))))))"
        );
    }

    #[test]
    fn test_parser_units() {
        let mut env = Env::new();
        env.built_in();
        // env.debug = true;
        assert_eq!(parse_as_string(&mut env, "1"), "Num(1, Units(None))");
        assert_eq!(
            parse_as_string(&mut env, "1[m]"),
            "Num(1, Units(Var(Ident(\"m\"))))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1[mm]"),
            "Num(1, Units(Var(Ident(\"mm\"))))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1[m/s]"),
            "Num(1, Units(BinOp(Op(Div), Var(Ident(\"m\")), Var(Ident(\"s\")))))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1[m*m/s]"),
            "Num(1, Units(BinOp(Op(Div), BinOp(Op(Mul), Var(Ident(\"m\")), Var(Ident(\"m\"))), Var(Ident(\"s\")))))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1k[m*m/s]"), 
            "FNum(1000.0, Units(BinOp(Op(Div), BinOp(Op(Mul), Var(Ident(\"m\")), Var(Ident(\"m\"))), Var(Ident(\"s\")))))"
        );
        // 新しいテストケース
        assert_eq!(
            parse_as_string(&mut env, "1[m^2]"),
            "Num(1, Units(BinOp(Op(Caret), Var(Ident(\"m\")), Num(2, Units(None)))))"
        );
        assert_eq!(
            parse_as_string(&mut env, "1[m^2/s^2]"),
            "Num(1, Units(BinOp(Op(Div), BinOp(Op(Caret), Var(Ident(\"m\")), Num(2, Units(None))), BinOp(Op(Caret), Var(Ident(\"s\")), Num(2, Units(None))))))"
        );
    }

    #[test]
    fn test_parser_command() {
        let mut env = Env::new();
        env.built_in();
        assert_eq!(
            parse_as_string(&mut env, "debug 1"),
            "Command(Ident(\"debug\"), [Num(1)], \"\")"
        );
        assert_eq!(
            parse_as_string(&mut env, "debug"),
            "Command(Ident(\"debug\"), [], \"\")"
        );
        assert_eq!(
            parse_as_string(&mut env, "constant"),
            "Command(Ident(\"constant\"), [], \"\")"
        );
        assert_eq!(
            parse_as_string(&mut env, "defun add  _1+_2 "),
            "Command(Ident(\"defun\"), [Ident(\"add\"), Ident(\"_1\"), Op(Plus), Ident(\"_2\")], \"\")"
        );
        assert_eq!(
            parse_as_string(&mut env, "format sep4 16"),
            "Command(Ident(\"format\"), [Ident(\"sep4\"), Num(16)], \"\")"
        );
    }

    #[test]
    fn test_parser_error() {
        let mut env = Env::new();
        env.built_in();

        if let Ok(_) = parse(&mut env, &(lexer("2*sin(1, 2)".to_owned())).unwrap()) {
            assert!(false);
        }
        if let Ok(_) = parse(&mut env, &(lexer("sin(".to_owned())).unwrap()) {
            assert!(false);
        }
        if let Ok(_) = parse(&mut env, &(lexer("sin()".to_owned())).unwrap()) {
            assert!(false);
        }
        if let Ok(_) = parse(&mut env, &(lexer("1+2+".to_owned())).unwrap()) {
            assert!(false);
        }
        if let Ok(_) = parse(&mut env, &(lexer("sin".to_owned())).unwrap()) {
            assert!(false);
        }
        if let Ok(_) = parse(&mut env, &(lexer("((())".to_owned())).unwrap()) {
            assert!(false);
        }
    }
}
