use std::fmt;
use std::iter::Peekable;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Num(u64),
    Op(char),
}

fn tok_num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
    let mut n = c.to_string().parse::<u64>().unwrap();
    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                n = n * 10 + c.to_string().parse::<u64>().unwrap();
                iter.next();
            }
            _ => {
                return n;
            }
        }
    }
    n
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
        }
    }
    return 0;
}

fn main() {
    println!("lexer");
    println!("1 -> {:?}", lexer("1".to_string()));
    println!("10 1 -> {:?}", lexer("10 1".to_string()));
    println!("1+1 -> {:?}", lexer("1+1".to_string()));
    println!("1-1 -> {:?}", lexer("1-1".to_string()));
    println!("-1 -> {:?}", lexer("-1".to_string()));
    println!("+-*/%()^100 -> {:?}", lexer("+-*/%()^-100".to_string()));
    println!("");
    println!("parser");
    println!("1 -> {:?}", parse(&lexer("1".to_string())));
    println!("0 -> {:?}", parse(&lexer("0".to_string())));
    println!("-1 -> {:?}", parse(&lexer("-1".to_string())));
    println!(
        "9223372036854775807 -> {:?}",
        parse(&lexer("9223372036854775807".to_string()))
    );
    println!(
        "-9223372036854775808 -> {:?}",
        parse(&lexer("-9223372036854775808".to_string()))
    );
    println!("1+2 -> {:?}", parse(&lexer("1+2".to_string())));
    println!("1-2 -> {:?}", parse(&lexer("1-2".to_string())));
    println!("1+-2 -> {:?}", parse(&lexer("1+-2".to_string())));
    println!("1*2 -> {:?}", parse(&lexer("1*2".to_string())));
    println!("1*2+3 -> {:?}", parse(&lexer("1*2+3".to_string())));
    println!("1+2*3 -> {:?}", parse(&lexer("1+2*3".to_string())));
    println!("1*(2+3) -> {:?}", parse(&lexer("1*(2+3)".to_string())));
    println!("(1+2)*3 -> {:?}", parse(&lexer("(1+2)*3".to_string())));
    println!("1+2+3 -> {:?}", parse(&lexer("1+2+3".to_string())));
    println!("1*2*3 -> {:?}", parse(&lexer("1*2*3".to_string())));
    println!("");
    println!("eval");
    println!("1 -> {:?}", eval(&parse(&lexer("1".to_string()))));
    println!("0 -> {:?}", eval(&parse(&lexer("0".to_string()))));
    println!("-1 -> {:?}", eval(&parse(&lexer("-1".to_string()))));
    println!(
        "9223372036854775807 -> {:?}",
        eval(&parse(&lexer("9223372036854775807".to_string())))
    );
    println!(
        "-9223372036854775807 -> {:?}",
        eval(&parse(&lexer("-9223372036854775807".to_string())))
    );
    println!("1+2 -> {:?}", eval(&parse(&lexer("1+2".to_string()))));
    println!("1-2 -> {:?}", eval(&parse(&lexer("1-2".to_string()))));
    println!("1+-2 -> {:?}", eval(&parse(&lexer("1+-2".to_string()))));
    println!("1*2 -> {:?}", eval(&parse(&lexer("1*2".to_string()))));
    println!("1*2+3 -> {:?}", eval(&parse(&lexer("1*2+3".to_string()))));
    println!("1+2*3 -> {:?}", eval(&parse(&lexer("1+2*3".to_string()))));
    println!(
        "1*(2+3) -> {:?}",
        eval(&parse(&lexer("1*(2+3)".to_string())))
    );
    println!(
        "(1+2)*3 -> {:?}",
        eval(&parse(&lexer("(1+2)*3".to_string())))
    );
    println!("1+2+3 -> {:?}", eval(&parse(&lexer("1+2+3".to_string()))));
    println!("1*2*3 -> {:?}", eval(&parse(&lexer("1*2*3".to_string()))));
}
