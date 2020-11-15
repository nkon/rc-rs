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

# 9698ed3cd38eca4973203654ff1f099f336f39f7

* 文字列を、`fn lexer(s: String) -> Vec<Token>`でトークンに分解し、`pub fn parse(s: String) -> Node`で再帰降順によりパースできた。
* Debug printのため、 `enum NodeType`に `fmt::Debug`を実装した。
* Lexerは、参考ページを参照して、`peek()`で先読みするようにしたが、全入力をバッファーに割り当てて`Vec`のインデックスで操作したほうが良いかもしれない。
* Parserは、全トークン列を`Vec`に割り当てて、インデックスで（イテレータを使わずに）アクセスしている。読み込んだ結果とトークン数を返すために、返り値はタプルとなっている。
* この段階ではエラー処理はしていない。将来的には`Result<T,E>`を使ったエラー処理が必要となるだろう。

# f3872e3c339f150e7670f7a42ab9b809d1e4dce4

* `pub fn parse(tok: &Vec<Token>) -> Node `が返したASTを`pub fn eval(n: &Node) -> i64 `が計算できるようになった。
* Debug printを見やすくした。

