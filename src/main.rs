use std::fmt;
use std::iter::Peekable;

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Num(u64),
    Op(char),
}

fn tok_num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
    let mut n = c.to_string().parse::<u64>().unwrap();
    while let Some(&c) = iter.peek() {
        if c == '0'
            || c == '1'
            || c == '2'
            || c == '3'
            || c == '4'
            || c == '5'
            || c == '6'
            || c == '7'
            || c == '8'
            || c == '9'
        {
            n = n * 10 + c.to_string().parse::<u64>().unwrap();
            iter.next();
        } else {
            return n;
        };
    }
    n
}

fn lexer(s: String) -> Vec<Token> {
    let mut ret = Vec::new();

    let mut iter = s.chars().peekable();
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                iter.next();
                let n = tok_num(c, &mut iter);
                ret.push(Token::Num(n));
            }
            '+' | '-' => {
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

pub enum NodeType {
    None,
    Num,
    Unary,
    BinOp,
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

#[derive(Debug)]
pub struct Node {
    pub ty: NodeType,
    pub value: u64,
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

pub fn num(tok: &Vec<Token>, i: usize) -> (Node, usize) {
    println!("num, i={}", i);
    let mut node = Node::new();
    match tok[i] {
        Token::Num(n) => {
            node.ty = NodeType::Num;
            node.value = n;
        }
        _ => {}
    }
    println!("num: value={}, i={}", node.value, i + 1);
    (node, i + 1)
}

pub fn unary(tok: &Vec<Token>, i: usize) -> (Node, usize) {
    println!("unary, i={}", i);
    let mut node = Node::new();
    match tok[i] {
        Token::Op('-') | Token::Op('+') => {
            node.ty = NodeType::Unary;
            node.op = tok[i];
            let (rhs, i) = num(tok, i + 1);
            node.child.push(rhs);
            return (node, i);
        }
        _ => {
            return (node, i);
        }
    }
}

pub fn expr(tok: &Vec<Token>, i: usize) -> (Node, usize) {
    println!("expr, i={}", i);
    let (lhs, i) = unary(tok, i);
    if tok.len() >= i {
        return (lhs, i);
    }
    let mut node = Node::new();
    match tok[i] {
        Token::Op('+') | Token::Op('-') => {
            node.ty = NodeType::BinOp;
            node.op = tok[i];
            let (rhs, i) = unary(tok, i + 1);
            node.child.push(lhs);
            node.child.push(rhs);
            return (node, i + 1);
        }
        _ => {
            return (lhs, i);
        }
    }
}

pub fn parse(s: String) -> Node {
    let tokens = lexer(s);
    let i = 0;
    let (node, _) = expr(&tokens, i);

    node
}

fn main() {
    println!("lexer");
    println!("1 -> {:?}", lexer("1".to_string()));
    println!("10 1 -> {:?}", lexer("10 1".to_string()));
    println!("1+1 -> {:?}", lexer("1+1".to_string()));
    println!("1-1 -> {:?}", lexer("1-1".to_string()));
    println!("-1 -> {:?}", lexer("-1".to_string()));
    println!("");
    println!("parser");
    println!("1 -> {:?}", parse("1".to_string()));
    println!("0 -> {:?}", parse("0".to_string()));
    println!("-1 -> {:?}", parse("-1".to_string()));
    println!(
        "9223372036854775807 -> {:?}",
        parse("9223372036854775807".to_string())
    );
    println!(
        "-9223372036854775808 -> {:?}",
        parse("-9223372036854775808".to_string())
    );
}
