# 参考文献

https://adriann.github.io/rust_parser.html


# Node, Token

`Token`は`enum`として定義する。

```rust
#[derive(Debug, Clone)]
pub enum Token {
    Num(u64),
    Op(char),
}
```

`fn lexer(s: String) -> Vec<Token>`


`Node`は`struct`として定義する。

```rust
#[derive(Debug)]
pub struct Node {
    pub ty: NodeType,
    pub value: u64,
    pub child: Vec<Node>, // child[0]: LHS, child[1]: RHS
}
```

`pub fn parse(s: String) -> Node`

