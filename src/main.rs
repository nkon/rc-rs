use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Token {
    Num(u64),
    Op(u16),
}

fn num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
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
                let n = num(c, &mut iter);
                ret.push(Token::Num(n));
            }
            _ => {
                let _ = iter.next();
            }
        }
    }

    ret
}

#[derive(Debug, Clone)]
pub struct Node {
    pub entry: Token,
    pub child: Vec<Node>, // child[0]: LHS, child[1]: RHS
}

impl Node {
    pub fn new() -> Node {
        Node {
            entry: Token::Num(0),
            child: Vec::new(),
        }
    }
}

pub fn parse(s: String) -> Node {
    let mut node = Node::new();

    let mut tokens = lexer(s);

    for token in tokens.pop() {
        match token {
            Token::Num(n) => node.entry = Token::Num(n),
            _ => {}
        }
    }

    node
}

fn main() {
    println!("lexer");
    println!("1 -> {:?}", lexer("1".to_string()));
    println!("1 1 -> {:?}", lexer("1 1".to_string()));
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
