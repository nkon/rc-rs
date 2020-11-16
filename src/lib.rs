use std::fmt;
use std::iter::Peekable;

mod readline;
pub use readline::readline;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Num(i128),
    FNum(f64),
    Op(char),
}

fn tok_num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> i128 {
    let mut n = c.to_string().parse::<i128>().unwrap();
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                n = n * 10 + c.to_string().parse::<i128>().unwrap();
                iter.next();
            }
            _ => {
                return n;
            }
        }
    }
    return n;
}

fn tok_get_num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> String {
    let mut ret = String::from(c);
    if ret == "-"
        || ret == "0"
        || ret == "1"
        || ret == "2"
        || ret == "3"
        || ret == "4"
        || ret == "5"
        || ret == "6"
        || ret == "7"
        || ret == "8"
        || ret == "9"
    {
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
    } else {
        return String::from(' ');
    }
}

fn tok_fnum<T: Iterator<Item = char>>(_c: char, iter: &mut Peekable<T>) -> f64 {
    // let mut mantissa = c.to_string();
    let mut mantissa = String::new();
    let mut exponent = String::new();
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' | '.' => {
                mantissa.push(c);
                iter.next();
            }
            'e' | 'E' => {
                iter.next();
                let &c = iter.peek().unwrap();
                exponent = tok_get_num(c, iter);
                break;
            }
            _ => {
                return mantissa.parse::<f64>().unwrap();
            }
        }
    }
    if exponent == "" {
        return mantissa.parse::<f64>().unwrap();
    } else {
        mantissa.push_str("e");
        mantissa.push_str(&exponent);
        return mantissa.parse::<f64>().unwrap();
    }
}

pub fn lexer(s: String) -> Vec<Token> {
    let mut ret = Vec::new();

    let mut iter = s.chars().peekable();
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                iter.next();
                let n = tok_num(c, &mut iter);
                ret.push(Token::Num(n));
            }
            'f' => {
                iter.next();
                let n = tok_fnum(c, &mut iter);
                ret.push(Token::FNum(n));
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

/*
<expr>    ::= <mul> ( '+' <mul> | '-' <mul> )*
<mul>     ::= <primary> ( '*' <primary> | '/' <primary>)*
<primary> ::= <unary> | '(' <expr> ')'
<unary>   ::= <num> | '-' <num> | '+' <num>
*/

#[derive(Clone, PartialEq)]
pub enum NodeType {
    None,
    Num,   // value <- value
    Unary, // op <- operator, child[0] <- operand
    BinOp, // op <- operator, child[0] <- lhs, child[1] <- rhs
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::None => write!(f, "None"),
            NodeType::Num => write!(f, "Num"),
            NodeType::Unary => write!(f, "Unary"),
            NodeType::BinOp => write!(f, "BinOp"),
        }
    }
}

pub struct Node {
    pub ty: NodeType,
    pub value: i128,
    pub op: Token,
    pub child: Vec<Node>, // child[0]: LHS, child[1]: RHS
}

impl Node {
    pub fn new() -> Node {
        Node {
            ty: NodeType::None,
            value: 0,
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

pub fn eval(n: &Node) -> i64 {
    if n.ty == NodeType::Num {
        return n.value as i64;
    } else if n.ty == NodeType::Unary {
        if n.op == Token::Op('-') {
            return -1 * eval(&n.child[0]) as i64;
        }
    } else if n.ty == NodeType::BinOp {
        if n.op == Token::Op('+') {
            return eval(&n.child[0]) + eval(&n.child[1]);
        } else if n.op == Token::Op('-') {
            return eval(&n.child[0]) - eval(&n.child[1]);
        } else if n.op == Token::Op('*') {
            return eval(&n.child[0]) * eval(&n.child[1]);
        } else if n.op == Token::Op('/') {
            return eval(&n.child[0]) / eval(&n.child[1]);
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        assert_eq!(lexer("1".to_string()), [Token::Num(1)]);
        assert_eq!(lexer("0".to_string()), [Token::Num(0)]);
        assert_eq!(lexer("10".to_string()), [Token::Num(10)]);
        assert_eq!(
            lexer("9223372036854775807".to_string()),
            [Token::Num(9223372036854775807)]
        );
        assert_eq!(
            lexer("18446744073709551615".to_string()),
            [Token::Num(18446744073709551615)]
        );
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
        assert_eq!(eval(&parse(&lexer("1+2".to_string()))), 3);
        assert_eq!(eval(&parse(&lexer("1+2*3".to_string()))), 7);
        assert_eq!(eval(&parse(&lexer("1*2+3".to_string()))), 5);
        assert_eq!(eval(&parse(&lexer("1+2+3".to_string()))), 6);
        assert_eq!(eval(&parse(&lexer("(1+2)*3".to_string()))), 9);
        assert_eq!(eval(&parse(&lexer("-2".to_string()))), -2);
        assert_eq!(
            eval(&parse(&lexer("-9223372036854775807".to_string()))),
            -9223372036854775807
        );
    }
}
