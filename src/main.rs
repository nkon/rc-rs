#[derive(Debug, Clone)]
pub enum Token {
    Num(i64),
}

fn num(s: String) -> Token {
    Token::Num(s.parse().unwrap())
}

#[derive(Debug, Clone)]
pub struct Node {
    pub entry: Token,
    pub child: Vec<Node>,    // child[0]: LHS, child[1]: RHS
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
    let mut n = Node::new();
    n.entry = num(s);

    n
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
