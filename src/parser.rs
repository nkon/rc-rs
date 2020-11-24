use super::*;
use std::fmt;

// <expr>    ::= <mul> ( '+' <mul> | '-' <mul> )*
// <mul>     ::= <unary> ( '*' <unary> | '/' <unary>)*
// <unary>   ::= <primary> | '-' <primary> | '+' <primary>
// <primary> ::= <num> | '(' <expr> ')' | <var> | <func> '(' <expr>* ',' ')'
// <num>     ::= <num> | <num> <postfix>

#[derive(Clone, Copy, PartialEq)]
pub enum NodeType {
    None,
    Num,   // value <- value
    FNum,  // fvalue <- value
    Unary, // op <- operator, child[0] <- operand
    BinOp, // op <- operator, child[0] <- lhs, child[1] <- rhs
    Var,   // op <- Token::Ident(ident)
    Func,  // op <- Token::Ident(ident), child[] <- parameter
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::None => write!(f, "None"),
            NodeType::Num => write!(f, "Num"),
            NodeType::FNum => write!(f, "FNum"),
            NodeType::Unary => write!(f, "Unary"),
            NodeType::BinOp => write!(f, "BinOp"),
            NodeType::Var => write!(f, "Var"),
            NodeType::Func => write!(f, "Func"),
        }
    }
}

// TODO: change from struct to Enum to maximize Rust power
pub struct Node {
    pub ty: NodeType,
    pub value: i128,
    pub fvalue: f64,
    pub op: Token,
    pub child: Vec<Node>, // child[0]: LHS, child[1]: RHS
}

impl Node {
    pub fn new() -> Node {
        Node {
            ty: NodeType::None,
            value: 0,
            fvalue: 0.0,
            child: Vec::new(),
            op: Token::Op(' '),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ty {
            NodeType::None => write!(f, "None"),
            NodeType::Num => write!(f, "Num({})", self.value),
            NodeType::FNum => write!(f, "FNum({})", self.fvalue),
            NodeType::Unary => write!(f, "Unary({:?} {:?})", self.op, self.child[0]),
            NodeType::BinOp => write!(f, "BinOp({:?} {:?})", self.op, self.child),
            NodeType::Var => write!(f, "Var({:?})", self.op),
            NodeType::Func => write!(f, "Func({:?} {:?})", self.op, self.child),
        }
    }
}

fn num(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), String> {
    if env.is_debug() {
        eprintln!("num {:?} {}\r", tok, i);
    }
    if tok.len() <= i {
        return Err(format!(
            "Error: unexpected end of input: {} {}",
            file!(),
            line!()
        ));
    }
    let mut node = Node::new();
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
                node.ty = NodeType::FNum;
                node.fvalue = n as f64 * f_postfix;
                Ok((node, i + 2))
            } else {
                node.ty = NodeType::Num;
                node.value = n;
                Ok((node, i + 1))
            }
        }
        Token::FNum(n) => {
            node.ty = NodeType::FNum;
            if has_postfix {
                node.fvalue = n * f_postfix;
                Ok((node, i + 2))
            } else {
                node.fvalue = n;
                Ok((node, i + 1))
            }
        }
        _ => Ok((node, i)),
    }
}

fn primary(env: &mut Env, tok: &[Token], index: usize) -> Result<(Node, usize), String> {
    let mut i = index;
    if env.is_debug() {
        eprintln!("primary {:?} {}\r", tok, i);
    }
    if tok.len() <= i {
        return Err(format!(
            "Error: unexpected end of input: {} {}",
            file!(),
            line!()
        ));
    }
    match &tok[i] {
        Token::Op('(') => {
            if let Ok((expr, i)) = expr(env, tok, i + 1) {
                if tok[i] != Token::Op(')') {
                    println!("')' not found.");
                }
                Ok((expr, i + 1))
            } else {
                Err(format!("Error: primary() {} {}", file!(), line!()))
            }
        }
        Token::Ident(id) => {
            let mut ret_node = Node::new();
            if let Some(_constant) = env.is_const(id.as_str()) {
                ret_node.ty = NodeType::Var;
                ret_node.op = Token::Ident(id.clone());
                return Ok((ret_node, i + 1));
            }
            if let Some(_func_tupple) = env.is_func(id.as_str()) {
                // TODO: parameter number check
                ret_node.ty = NodeType::Func;
                ret_node.op = Token::Ident(id.clone());
                if (i + 1) < tok.len() || tok[i + 1] == Token::Op('(') {
                    i += 2;
                    while i < tok.len() {
                        if tok[i] == Token::Op(')') {
                            return Ok((ret_node, i + 1));
                        } else if tok[i] == Token::Op(',') {
                            i += 1;
                            continue;
                        } else if let Ok((t, j)) = expr(env, tok, i) {
                            i = j;
                            ret_node.child.push(t);
                        } else {
                            return Err(format!("Error: primary() {} {}", file!(), line!()));
                        }
                    }
                }
                return Ok((ret_node, i + 1));
            }
            Ok((ret_node, i))
        }
        _ => num(env, tok, i),
    }
}

fn unary(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), String> {
    if env.is_debug() {
        eprintln!("unary {:?} {}\r", tok, i);
    }
    if tok.len() <= i {
        return Err(format!(
            "Error: unexpected end of input: {} {}",
            file!(),
            line!()
        ));
    }
    match tok[i] {
        Token::Op('-') | Token::Op('+') => {
            let mut node = Node::new();
            node.ty = NodeType::Unary;
            node.op = tok[i].clone();
            if let Ok((rhs, i)) = primary(env, tok, i + 1) {
                node.child.push(rhs);
                Ok((node, i))
            } else {
                Err(format!("Error: unary(): {} {}", file!(), line!()))
            }
        }
        _ => primary(env, tok, i),
    }
}

fn mul(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), String> {
    if env.is_debug() {
        eprintln!("mul {:?} {}\r", tok, i);
    }
    if tok.len() <= i {
        return Err(format!(
            "Error: unexpected end of input: {} {}",
            file!(),
            line!()
        ));
    }
    if let Ok((mut lhs, mut i)) = unary(env, tok, i) {
        loop {
            if tok.len() <= i {
                return Ok((lhs, i));
            }
            match tok[i] {
                Token::Op('*') | Token::Op('/') | Token::Op('%') => {
                    let mut node = Node::new();
                    node.ty = NodeType::BinOp;
                    node.op = tok[i].clone();
                    if let Ok((rhs, j)) = unary(env, tok, i + 1) {
                        node.child.push(lhs);
                        node.child.push(rhs);
                        i = j;
                        lhs = node;
                    } else {
                        return Err(format!(
                            "Error: Operator '*' '/' '%' requires right side operand. {:?} {}",
                            tok, i
                        ));
                    }
                }
                _ => {
                    return Ok((lhs, i));
                }
            }
        }
    }
    Err(format!("Error: expr(): {} {}", file!(), line!()))
}

// TODO: Error handling in parser: Result<(Node, usize), String>
fn expr(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), String> {
    if env.is_debug() {
        eprintln!("expr {:?} {}\r", tok, i);
    }
    if tok.len() <= i {
        return Err(format!(
            "Error: unexpected end of input: {} {}",
            file!(),
            line!()
        ));
    }
    if let Ok((mut lhs, mut i)) = mul(env, tok, i) {
        loop {
            if tok.len() <= i {
                return Ok((lhs, i));
            }
            match tok[i] {
                Token::Op('+') | Token::Op('-') => {
                    let mut node = Node::new();
                    node.ty = NodeType::BinOp;
                    node.op = tok[i].clone();
                    if let Ok((rhs, j)) = mul(env, tok, i + 1) {
                        node.child.push(lhs);
                        node.child.push(rhs);
                        i = j;
                        lhs = node;
                    } else {
                        return Err(format!(
                            "Error: Operator'+'/'-' requires right side operand. {:?} {}",
                            tok, i
                        ));
                    }
                }
                _ => {
                    return Ok((lhs, i));
                }
            }
        }
    }
    Err(format!("Error: expr(): {} {}", file!(), line!()))
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
/// assert_eq!(format!("{:?}", parse(&mut env, &(lexer("1+2".to_string()).unwrap())).unwrap()),"BinOp(Op('+') [Num(1), Num(2)])");
/// assert_eq!(format!("{:?}", parse(&mut env, &(lexer("1-2".to_string()).unwrap())).unwrap()),"BinOp(Op('-') [Num(1), Num(2)])");
/// ```
// TODO: user define var
// TODO: user devine function
pub fn parse(env: &mut Env, tok: &[Token]) -> Result<Node, String> {
    match expr(env, &tok, 0) {
        Ok((node, i)) => {
            if i < tok.len() {
                Err(format!("parse error: token left: {:?} {}", tok, i))
            } else {
                Ok(node)
            }
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_as_string(env: &mut Env, input: &str) -> String {
        let n = parse(env, &(lexer(input.to_string())).unwrap()).unwrap();
        format!("{:?}", n)
    }

    #[test]
    fn test_parser() {
        let mut env = Env::new();
        env.built_in();

        assert_eq!(
            parse_as_string(&mut env, "1+2"),
            "BinOp(Op('+') [Num(1), Num(2)])"
        );
        assert_eq!(
            parse_as_string(&mut env, "1-2"),
            "BinOp(Op('-') [Num(1), Num(2)])"
        );
        assert_eq!(
            parse_as_string(&mut env, "1+-2"),
            "BinOp(Op('+') [Num(1), Unary(Op('-') Num(2))])"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*2"),
            "BinOp(Op('*') [Num(1), Num(2)])"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*2+3"),
            "BinOp(Op('+') [BinOp(Op('*') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*(2+3)"),
            "BinOp(Op('*') [Num(1), BinOp(Op('+') [Num(2), Num(3)])])"
        );
        assert_eq!(
            parse_as_string(&mut env, "1+2+3"),
            "BinOp(Op('+') [BinOp(Op('+') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            parse_as_string(&mut env, "(1+2)+3"),
            "BinOp(Op('+') [BinOp(Op('+') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            parse_as_string(&mut env, "1*2*3"),
            "BinOp(Op('*') [BinOp(Op('*') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            parse_as_string(&mut env, "(1*2)*3"),
            "BinOp(Op('*') [BinOp(Op('*') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            parse_as_string(&mut env, "-(2+3)"),
            "Unary(Op('-') BinOp(Op('+') [Num(2), Num(3)]))"
        );
        assert_eq!(parse_as_string(&mut env, "pi"), "Var(Ident(\"pi\"))");
        assert_eq!(
            parse_as_string(&mut env, "2.0*pi"),
            "BinOp(Op('*') [FNum(2), Var(Ident(\"pi\"))])"
        );
        assert_eq!(parse_as_string(&mut env, "2k"), "FNum(2000)");
        assert_eq!(
            parse_as_string(&mut env, "2u*pi"),
            "BinOp(Op('*') [FNum(0.000002), Var(Ident(\"pi\"))])"
        );
        assert_eq!(
            parse_as_string(&mut env, "2*sin(0.5*pi)"),
            "BinOp(Op('*') [Num(2), Func(Ident(\"sin\") [BinOp(Op('*') [FNum(0.5), Var(Ident(\"pi\"))])])])"
        );
        assert_eq!(
            parse_as_string(&mut env, "2*sin(1, 2)"),
            "BinOp(Op('*') [Num(2), Func(Ident(\"sin\") [Num(1), Num(2)])])"
        );
    }
}
