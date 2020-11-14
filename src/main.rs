#[derive(Debug, Clone)]
pub enum Token {
    Num(i64),
    Op(u16),
}

fn num(s: String) -> Token {
    Token::Num(s.parse().unwrap())
}

fn tokens(s: String) -> Vec<Token> {
    let mut ret = Vec::new();
    ret.push(num(s));

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
    
    let mut tk = tokens(s);

    for token in tk.pop() {
        match token {
            Token::Num(n) => node.entry = Token::Num(n),
            _ => {}
        }
    }

    node
}

fn main() {
    println!("Hello, world!");
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
