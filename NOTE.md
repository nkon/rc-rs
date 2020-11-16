Design Note of `rc`
===================

* 入力としては、Readerからの文字列を受け取る。それをlexerでトークン列に分解し`Vec<Token>`とする。Parserは`Vec<Token>`を受け取り、インデックスを移動させることで（イテレータを使わずに）パーズして、ASTを返す。Parserは`Vec<Token>`とパースする箇所のインデックスを受け取り、ASTと読み進めた後のインデックスをタプルとして返す。インデックスをポインターとして受け取ってそれを変更するようにはなっていない。それぞれの文法要素に対応するパーザ関数があり、再帰下降でパースしてゆく。ASTをevalすることで計算結果を得る。

Tokenはenumとして実装し、それぞれの枝に値を持っている。

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Num(u64),
    Op(char),
}
```

ASTはstrutとして実装し、`NodeType`がノードの種類を表し、他のメンバーには、それぞれの要素に必要な値が入っている。

```rust
#[derive(Clone, PartialEq)]
pub enum NodeType {
    None,
    Num,   // value <- value
    Unary, // op <- operator, child[0] <- operand
    BinOp, // op <- operator, child[0] <- lhs, child[1] <- rhs
}

pub struct Node {
    pub ty: NodeType,
    pub value: u64,
    pub op: Token,
    pub child: Vec<Node>, // child[0]: LHS, child[1]: RHS
}
```

* エラー処理


* rustのコンパイラは厳しいが親切。コンパイル時にエラーが出まくるのは有名な話だが、どう直せばよいかも提示してくれる。初心者が遭遇するエラーは典型的なので、コンパイラがヒントとして提示するように直していけば、たいてい解消される。VS Codeを使っていると、コンパイラを通さずとも赤線が引かれるので、注釈通りに修正していけばよい。

