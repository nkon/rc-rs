use std::fmt;
use std::iter::Peekable;

mod readline;
pub use readline::readline;

// TODO: Separete lexter into `lexer.rs`.
// TODO: add Doc-test.

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Num(i128),
    FNum(f64),
    Op(char),
}

fn tok_get_num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> String {
    match c {
        '-' | '0'..='9' => {
            let mut ret = String::from(c);
            iter.next();
            while let Some(&c) = iter.peek() {
                match c {
                    '0'..='9' => {
                        ret.push(c);
                        iter.next();
                    }
                    _ => {
                        return ret;
                    }
                }
            }
            return ret;
        }
        _ => {
            return String::from('0');
        }
    }
}

fn tok_num_int<T: Iterator<Item = char>>(
    _c: char,
    iter: &mut Peekable<T>,
) -> Result<Token, String> {
    let mut radix = 10;
    let mut mantissa = String::from("0");
    let mut err_str = String::from("0");

    if let Some(&c) = iter.peek() {
        match c {
            'x' | 'X' => {
                radix = 16;
                iter.next();
                err_str.push(c);
            }
            'b' | 'B' => {
                radix = 2;
                iter.next();
                err_str.push(c);
            }
            '0'..='7' => {
                radix = 8;
            }
            _ => {
                return Ok(Token::Num(0));
            }
        }
    }
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' | 'a'..='f' | 'A'..='F' => {
                mantissa.push(c);
                err_str.push(c);
                iter.next();
            }
            '_' => {
                iter.next();
            }
            _ => {
                break;
            }
        }
    }
    match i128::from_str_radix(&mantissa, radix) {
        Ok(int) => {
            return Ok(Token::Num(int));
        }
        Err(e) => {
            return Err(format!("Error: Integer format: {} {}", e, err_str));
        }
    }
}

// TODO: return `Result<Token, String>` for error handling.
fn tok_num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> Token {
    let mut mantissa = String::from(c);
    let mut exponent = String::new();
    let mut has_dot = false;
    let mut has_exponent = false;
    if mantissa == "0" {
        match iter.peek() {
            Some(&c) => match c {
                '0'..='9' | 'a'..='f' | 'A'..='F' | 'x' | 'X' => match tok_num_int(c, iter) {
                    Ok(tok) => {
                        return tok;
                    }
                    Err(e) => {
                        panic!(e);
                    }
                },
                _ => {}
            },
            None => {
                return Token::Num(0);
            }
        }
    }
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                mantissa.push(c);
                iter.next();
            }
            '_' => {
                iter.next();
            }
            '.' => {
                mantissa.push(c);
                iter.next();
                has_dot = true;
            }
            'e' | 'E' => {
                iter.next();
                has_dot = true; // no dot but move to floating mode.
                has_exponent = true;
                let &c = iter.peek().unwrap();
                exponent = tok_get_num(c, iter);
                break;
            }
            _ => {
                break;
            }
        }
    }
    if !has_dot {
        return Token::Num(mantissa.parse::<i128>().unwrap());
    }
    if has_exponent {
        mantissa.push_str("e");
        mantissa.push_str(&exponent);
        return Token::FNum(mantissa.parse::<f64>().unwrap());
    }
    return Token::FNum(mantissa.parse::<f64>().unwrap());
}

// TODO: change from peekable iterator to Vec and index.
pub fn lexer(s: String) -> Vec<Token> {
    let mut ret = Vec::new();

    let mut iter = s.chars().peekable();
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                iter.next();
                let tk = tok_num(c, &mut iter);
                ret.push(tk);
            }
            '+' | '-' | '*' | '/' | '%' | '(' | ')' | '^' => {
                iter.next();
                ret.push(Token::Op(c));
            }
            _ => {
                let _ = iter.next();
            }
        }
    }

    ret
}

// BUG: eBNF should be below and code should be fixed
// <expr>    ::= <mul> ( '+' <mul> | '-' <mul> )*
// <mul>     ::= <unary> ( '*' <unary> | '/' <unary>)*
// <unary>   ::= <primary> | '-' <primary> | '+' <primary>
// <primary> ::= <num> | '(' <expr> ')'

/*
<expr>    ::= <mul> ( '+' <mul> | '-' <mul> )*
<mul>     ::= <primary> ( '*' <primary> | '/' <primary>)*
<primary> ::= <unary> | '(' <expr> ')'
<unary>   ::= <num> | '-' <num> | '+' <num>
*/

#[derive(Clone, Copy, PartialEq)]
pub enum NodeType {
    None,
    Num,   // value <- value
    FNum,  // fvalue <- value
    Unary, // op <- operator, child[0] <- operand
    BinOp, // op <- operator, child[0] <- lhs, child[1] <- rhs
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::None => write!(f, "None"),
            NodeType::Num => write!(f, "Num"),
            NodeType::FNum => write!(f, "FNum"),
            NodeType::Unary => write!(f, "Unary"),
            NodeType::BinOp => write!(f, "BinOp"),
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

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ty {
            NodeType::None => write!(f, "None"),
            NodeType::Num => write!(f, "Num({})", self.value),
            NodeType::FNum => write!(f, "FNum({})", self.fvalue),
            NodeType::Unary => write!(f, "Unary({:?} {:?})", self.op, self.child[0]),
            NodeType::BinOp => write!(f, "BinOp({:?} {:?})", self.op, self.child),
        }
    }
}

fn num(tok: &Vec<Token>, i: usize) -> (Node, usize) {
    // println!("num {:?} {}", tok, i);
    let mut node = Node::new();
    match tok[i] {
        Token::Num(n) => {
            node.ty = NodeType::Num;
            node.value = n;
            return (node, i + 1);
        }
        Token::FNum(n) => {
            node.ty = NodeType::FNum;
            node.fvalue = n;
            return (node, i + 1);
        }
        _ => {
            return (node, i);
        }
    }
}

fn unary(tok: &Vec<Token>, i: usize) -> (Node, usize) {
    // println!("unary {:?} {}", tok, i);
    match tok[i] {
        Token::Op('-') | Token::Op('+') => {
            let mut node = Node::new();
            node.ty = NodeType::Unary;
            node.op = tok[i];
            let (rhs, i) = num(tok, i + 1);
            node.child.push(rhs);
            return (node, i);
        }
        _ => {
            return num(tok, i);
        }
    }
}

fn primary(tok: &Vec<Token>, i: usize) -> (Node, usize) {
    // println!("primary {:?} {}", tok, i);
    match tok[i] {
        Token::Op('(') => {
            let (expr, i) = expr(tok, i + 1);
            return (expr, i + 1);
        }
        _ => {
            return unary(tok, i);
        }
    }
}

fn mul(tok: &Vec<Token>, i: usize) -> (Node, usize) {
    // println!("mul {:?} {}", tok, i);
    let (mut lhs, mut i) = primary(tok, i);
    loop {
        if tok.len() <= i {
            return (lhs, i);
        }
        match tok[i] {
            Token::Op('*') | Token::Op('/') | Token::Op('%') => {
                let mut node = Node::new();
                node.ty = NodeType::BinOp;
                node.op = tok[i];
                let (rhs, j) = primary(tok, i + 1);
                node.child.push(lhs);
                node.child.push(rhs);
                i = j;
                lhs = node;
            }
            _ => {
                return (lhs, i);
            }
        }
    }
}

fn expr(tok: &Vec<Token>, i: usize) -> (Node, usize) {
    // println!("expr {:?} {}", tok, i);
    let (mut lhs, mut i) = mul(tok, i);
    loop {
        if tok.len() <= i {
            return (lhs, i);
        }
        match tok[i] {
            Token::Op('+') | Token::Op('-') => {
                let mut node = Node::new();
                node.ty = NodeType::BinOp;
                node.op = tok[i];
                let (rhs, j) = mul(tok, i + 1);
                node.child.push(lhs);
                node.child.push(rhs);
                i = j;
                lhs = node;
            }
            _ => {
                return (lhs, i);
            }
        }
    }
}

pub fn parse(tok: &Vec<Token>) -> Node {
    let (node, _) = expr(&tok, 0);

    node
}

pub fn eval(n: &Node) -> Node {
    if n.ty == NodeType::Num {
        let mut ret_node = Node::new();
        ret_node.ty = NodeType::Num;
        ret_node.value = n.value;
        return ret_node;
    } else if n.ty == NodeType::FNum {
        let mut ret_node = Node::new();
        ret_node.ty = NodeType::FNum;
        ret_node.fvalue = n.fvalue;
        return ret_node;
    } else if n.ty == NodeType::Unary {
        if n.op == Token::Op('-') {
            let mut ret_node = Node::new();
            if n.child[0].ty == NodeType::Num {
                ret_node.ty = NodeType::Num;
                ret_node.value = -n.child[0].value;
                return ret_node;
            }
            if n.child[0].ty == NodeType::FNum {
                ret_node.ty = NodeType::FNum;
                ret_node.fvalue = -n.child[0].fvalue;
                return ret_node;
            }
        }
    } else if n.ty == NodeType::BinOp {
        let mut ret_node = Node::new();
        if n.op == Token::Op('+') {
            let lhs = eval(&n.child[0]);
            let rhs = eval(&n.child[1]);
            if lhs.ty == NodeType::Num && rhs.ty == NodeType::Num {
                ret_node.ty = NodeType::Num;
                ret_node.value = lhs.value + rhs.value;
                return ret_node;
            } else if lhs.ty == NodeType::FNum && rhs.ty == NodeType::FNum {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.fvalue + rhs.fvalue;
                return ret_node;
            } else if lhs.ty == NodeType::Num && rhs.ty == NodeType::FNum {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.value as f64 + rhs.fvalue;
                return ret_node;
            } else if lhs.ty == NodeType::FNum && rhs.ty == NodeType::Num {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.fvalue + rhs.value as f64;
                return ret_node;
            }
        } else if n.op == Token::Op('-') {
            let lhs = eval(&n.child[0]);
            let rhs = eval(&n.child[1]);
            if lhs.ty == NodeType::Num && rhs.ty == NodeType::Num {
                ret_node.ty = NodeType::Num;
                ret_node.value = lhs.value - rhs.value;
                return ret_node;
            } else if lhs.ty == NodeType::FNum && rhs.ty == NodeType::FNum {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.fvalue - rhs.fvalue;
                return ret_node;
            } else if lhs.ty == NodeType::Num && rhs.ty == NodeType::FNum {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.value as f64 - rhs.fvalue;
                return ret_node;
            } else if lhs.ty == NodeType::FNum && rhs.ty == NodeType::Num {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.fvalue - rhs.value as f64;
                return ret_node;
            }
        } else if n.op == Token::Op('*') {
            let lhs = eval(&n.child[0]);
            let rhs = eval(&n.child[1]);
            if lhs.ty == NodeType::Num && rhs.ty == NodeType::Num {
                ret_node.ty = NodeType::Num;
                ret_node.value = lhs.value * rhs.value;
                return ret_node;
            } else if lhs.ty == NodeType::FNum && rhs.ty == NodeType::FNum {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.fvalue * rhs.fvalue;
                return ret_node;
            } else if lhs.ty == NodeType::Num && rhs.ty == NodeType::FNum {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.value as f64 * rhs.fvalue;
                return ret_node;
            } else if lhs.ty == NodeType::FNum && rhs.ty == NodeType::Num {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.fvalue * rhs.value as f64;
                return ret_node;
            }
        } else if n.op == Token::Op('/') {
            let lhs = eval(&n.child[0]);
            let rhs = eval(&n.child[1]);
            if lhs.ty == NodeType::Num && rhs.ty == NodeType::Num {
                ret_node.ty = NodeType::Num;
                ret_node.value = lhs.value / rhs.value;
                return ret_node;
            } else if lhs.ty == NodeType::FNum && rhs.ty == NodeType::FNum {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.fvalue / rhs.fvalue;
                return ret_node;
            } else if lhs.ty == NodeType::Num && rhs.ty == NodeType::FNum {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.value as f64 / rhs.fvalue;
                return ret_node;
            } else if lhs.ty == NodeType::FNum && rhs.ty == NodeType::Num {
                ret_node.ty = NodeType::FNum;
                let lhs = eval(&n.child[0]);
                let rhs = eval(&n.child[1]);
                ret_node.fvalue = lhs.fvalue / rhs.value as f64;
                return ret_node;
            }
        }
    }
    let mut ret_node = Node::new();
    ret_node.ty = n.ty;
    ret_node.value = n.value;
    ret_node.fvalue = n.fvalue;
    return ret_node;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_format() {
        assert_eq!(lexer("1".to_string()), [Token::Num(1)]);
        assert_eq!(lexer("0".to_string()), [Token::Num(0)]);
        assert_eq!(lexer("10".to_string()), [Token::Num(10)]);
        assert_eq!(lexer("1.1".to_string()), [Token::FNum(1.1)]);
        assert_eq!(lexer("0.1".to_string()), [Token::FNum(0.1)]);
        assert_eq!(lexer("1.1E2".to_string()), [Token::FNum(110.0)]);
        assert_eq!(lexer("1.1E-2".to_string()), [Token::FNum(0.011)]);
        assert_eq!(lexer("100_000".to_string()), [Token::Num(100000)]);
        assert_eq!(lexer("0xa".to_string()), [Token::Num(10)]);
        assert_eq!(lexer("011".to_string()), [Token::Num(9)]);
        assert_eq!(lexer("0b11".to_string()), [Token::Num(3)]);
        assert_eq!(lexer("1e3".to_string()), [Token::FNum(1000.0)]);
        assert_eq!(
            lexer("9223372036854775807".to_string()),
            [Token::Num(9223372036854775807)]
        );
        assert_eq!(
            lexer("18446744073709551615".to_string()),
            [Token::Num(18446744073709551615)]
        );
    }

    #[test]
    #[should_panic(expected = "Error: Integer format: invalid digit found in string 0b12")]
    fn test_fmt1() {
        lexer("0b12".to_string());
    }
    #[test]
    #[should_panic(expected = "Error: Integer format: invalid digit found in string 018")]
    fn test_fmt2() {
        lexer("018".to_string());
    }

    #[test]
    fn test_lexer() {
        assert_eq!(
            lexer("1+2+3".to_string()),
            [
                Token::Num(1),
                Token::Op('+'),
                Token::Num(2),
                Token::Op('+'),
                Token::Num(3)
            ]
        );
        assert_eq!(
            lexer(" 1 + 2 + 3 ".to_string()),
            [
                Token::Num(1),
                Token::Op('+'),
                Token::Num(2),
                Token::Op('+'),
                Token::Num(3)
            ]
        );
        assert_eq!(
            lexer("1 2 34+-*/%()-^".to_string()),
            [
                Token::Num(1),
                Token::Num(2),
                Token::Num(34),
                Token::Op('+'),
                Token::Op('-'),
                Token::Op('*'),
                Token::Op('/'),
                Token::Op('%'),
                Token::Op('('),
                Token::Op(')'),
                Token::Op('-'),
                Token::Op('^')
            ]
        );
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            format!("{:?}", parse(&lexer("1+2".to_string()))),
            "BinOp(Op('+') [Num(1), Num(2)])"
        );
        assert_eq!(
            format!("{:?}", parse(&lexer("1-2".to_string()))),
            "BinOp(Op('-') [Num(1), Num(2)])"
        );
        assert_eq!(
            format!("{:?}", parse(&lexer("1+-2".to_string()))),
            "BinOp(Op('+') [Num(1), Unary(Op('-') Num(2))])"
        );
        assert_eq!(
            format!("{:?}", parse(&lexer("1*2".to_string()))),
            "BinOp(Op('*') [Num(1), Num(2)])"
        );
        assert_eq!(
            format!("{:?}", parse(&lexer("1*2+3".to_string()))),
            "BinOp(Op('+') [BinOp(Op('*') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            format!("{:?}", parse(&lexer("1*(2+3)".to_string()))),
            "BinOp(Op('*') [Num(1), BinOp(Op('+') [Num(2), Num(3)])])"
        );
        assert_eq!(
            format!("{:?}", parse(&lexer("1+2+3".to_string()))),
            "BinOp(Op('+') [BinOp(Op('+') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            format!("{:?}", parse(&lexer("(1+2)+3".to_string()))),
            "BinOp(Op('+') [BinOp(Op('+') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            format!("{:?}", parse(&lexer("1*2*3".to_string()))),
            "BinOp(Op('*') [BinOp(Op('*') [Num(1), Num(2)]), Num(3)])"
        );
        assert_eq!(
            format!("{:?}", parse(&lexer("(1*2)*3".to_string()))),
            "BinOp(Op('*') [BinOp(Op('*') [Num(1), Num(2)]), Num(3)])"
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(
            format!("{:?}", eval(&parse(&lexer("1+2".to_string())))),
            "Num(3)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&lexer("1+2*3".to_string())))),
            "Num(7)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&lexer("1*2+3".to_string())))),
            "Num(5)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&lexer("1+2+3".to_string())))),
            "Num(6)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&lexer("(1+2)*3".to_string())))),
            "Num(9)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&lexer("-2".to_string())))),
            "Num(-2)"
        );
        assert_eq!(
            format!(
                "{:?}",
                eval(&parse(&lexer("-9223372036854775807".to_string())))
            ),
            "Num(-9223372036854775807)"
        );
        assert_eq!(
            format!("{:?}", eval(&parse(&lexer("1.1+2.2".to_string())))),
            "FNum(3.3000000000000003)"
        );
    }
}
