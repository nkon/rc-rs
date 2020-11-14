# 参考文献

https://adriann.github.io/rust_parser.html


# Node, Token

`Token`は`enum`として定義する。

```rust
#[derive(Debug, Clone)]
pub enum Token {
    Num(i64),
}
```

`Node`は`struct`として定義する。

```rust
#[derive(Debug, Clone)]
pub struct Node {
    pub entry: Token,
    pub child: Vec<Node>,    // child[0]: LHS, child[1]: RHS
}
```

