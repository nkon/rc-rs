#[derive(Debug, Clone)]
enum Token {
    Num(i64),
}

fn num(s: String) -> Token {
    Token::Num(s.parse().unwrap())
}

fn main() {
    println!("Hello, world!");
    println!("1 -> {:?}", num("1".to_string()));
    println!("0 -> {:?}", num("0".to_string()));
    println!("-1 -> {:?}", num("-1".to_string()));
    println!(
        "9223372036854775807 -> {:?}",
        num("9223372036854775807".to_string())
    );
    println!(
        "-9223372036854775808 -> {:?}",
        num("-9223372036854775808".to_string())
    );
}
