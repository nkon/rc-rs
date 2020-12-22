Design Note of `rc`
===================
Table of Contents
-----------------

* [目的](#目的)
* [Lexer](#lexer)
* [Parser](#parser)
    * [整数・浮動小数点数・複素数](#整数浮動小数点数複素数)
    * [Enum](#enum)
* [Evaluator___](#evaluator___)
    * [複素数演算](#複素数演算)
* [MyError](#myerror)
    * [thiserror](#thiserror)
* [Command___](#command___)
* [システム定義定数___](#システム定義定数___)
* [ユーザ定義変数___](#ユーザ定義変数___)
* [システム定義関数___](#システム定義関数___)
    * [exp()___](#exp___)
* [ユーザ定義関数___](#ユーザ定義関数___)
* [オブジェクト指向___](#オブジェクト指向___)
    * [ライフタイム___](#ライフタイム___)
* [ファイル分割](#ファイル分割)
* [端末制御](#端末制御)
* [コマンドラインオプション](#コマンドラインオプション)
    * [バージョン情報の自動取得](#バージョン情報の自動取得)
* [テスト](#テスト)
* [インクリメンタルな開発](#インクリメンタルな開発)
* [開発環境](#開発環境)
* [Static link](#static-link)
    * [Linux](#linux)
    * [Windows](#windows)

Created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)


## 目的

従来、PC上での電卓としてiMemoを使っていた。Windows上で動くフリーソフトで、その名のとおり複素数計算ができる、かなり高機能な関数電卓ソフトだ。数式を入力して科学技術計算ができるのが便利で起動も早い。しかし、英語Windows環境では文字化けしたり、Linux（PC, RaspberryPiなど）でも同様の電卓が使いたかったり（`bc`では力不足）するので、勉強も兼ねて自作しようというのが作成の動機。2020年の冬もコロナの影響で冬ごもりになる予定だし、プロジェクトとしてはちょうどよいだろう、というのも理由。開発言語は個人的な好みによりRust。

最低限必要な機能としては次のようなものだ。

* コマンドラインで気軽に起動してすぐに計算が実行できる。
* 数式どおりの入力が使える。科学技術・エンジニアリング関数を備え、複素数計算も可能。
* 10進だけでなく16進、2進も取り扱える。
* 簡単なコマンドライン編集が可能。
* WindowsとLinuxのターミナル上で動作する。

将来的にはフィルターなどのグラフ特性が書けるとありがたいが、実装はしばらく先になるだろう。

日常の設計に実践投入して不自由しないレベルになったので公開することにした。

この記事では設計や実装についての解説を行う。

ソフトの使い方については[README.md](README.md)や[README-j.md](README-j.md)を参照していただきたい。

## Lexer

構成としては、オーソドックスに、Lexer（トークナイザ）→Parser→Evaluatorの3層構造とする。

Lexerは1行入力として`String`を受け取り、それをトークン列に分解し`Result<Vec<Token>, String>`を返す。

`Token`は`enum`として実装する。それぞれの枝に値を持っている。

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(i128),
    FNum(f64),
    Op(TokenOp),
    Ident(String),
}
```

Lexerの特徴として、`-100`を、単項演算子`-`と整数リテラル（`100`）へ分解するようにしている。単項演算子として`-`を消費してしまうほうが簡単な実装になるからだ。デメリットとしては、たとえば`u8`で−128を表すことができなくなる。

入力は`Vec<char>`に変換してインデックスでアクセスする。Iteratorを使わない。この方が型修飾を減らしてCっぽく実装できる。トークナイザは文字配列と現在注目している文字列のインデックスを受け取り、解析した結果のトークンと更新されたインデックスの2つを返す必要がある。複数の値を返す時には、参照引数の値を変更する（`&mut`で引数を受け取って変更する）のではなく、複数の値を返す。このスタイルはRust API Guidelineの[C-NO-OUT](https://sinkuu.github.io/api-guidelines/predictability.html#c-no-out)にも定められている。

```rust
fn tok_ident(chars: &[char], index: usize) -> (Token, usize) {
    let mut i = index;
    let mut ret = String::new();
    while i < chars.len() {
        match chars[i] {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                ret.push(chars[i]);
                i += 1;
            }
            _ => {
                return (Token::Ident(ret), i);
            }
        }
    }
    (Token::Ident(ret), i)
}
```

## Parser

ParserとEvaluatorは再帰がすべて。手書きの再帰降順パーサ（Recursive descending parser）を用いてトークン列をASTに変換する。EvaluatorはASTを再帰的に辿って式の値を決定する。文法と評価ツリーのトラバースがしっかりと設計できていれば、自分の予想外のことまでうまくいく。単純な再帰ではなく、複数の関数の間を循環するような再帰になっているので、どこに再帰させるかを間違えないようにだけ注意が必要。

Parserは`Vec<Token>`を受け取り、`Result<(Node, usize), MyError>`を返す。`Node`は下に示す`Enum`である。必要な場合には子要素（Nodeへのポインターなど）を持ち、ツリーを構成する。Lexerとおなじく、入力はIteratorではなく配列とインデックスでアクセスする。トークンを食べたあとの新しいインデックスはLexerと同様にタプルとして返す。

昔、Cでインタプリタを作った時は、構造体ではなく共用体を用いてメモリを節約した。RustのEnumは、たぶんそれを自動でやってくれる。

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    None,
    Num(i128),
    FNum(f64),
    CNum(Complex64),
    Unary(Token, Box<Node>),                 // TokenOp, Operand
    BinOp(Token, Box<Node>, Box<Node>),      // TokenOp, LHS, RHS
    Var(Token),                              // Token::Ident
    Func(Token, Vec<Node>),                  // Token::Ident, args...
    Command(Token, Vec<Token>, String),      // Token::Ident, args..., result-holder
}
```

### 整数・浮動小数点数・複素数

`rc`は科学技術計算用計算機なので、整数、浮動小数点数、複素数を扱うことができる。実装は、上の`Node`で示したように`Enum`を用いている。それぞれの枝の中に`i128`,`f64`,`Complex64`として格納する。多倍長整数（BigNum）や有理数は未実装。必要に応じて整数→浮動小数点数→複素数へ格上げして計算を実行する。標準ライブラリのように`Trait`をうまく使えば自然な表現を実現できそうだが、今は単純に場合分けによる実装としている。

### Enum

当初、Cでの類似の実装経験から、Node構造体などをStructで表し、Node.Typeといったメンバー変数で処理を分岐するように書いていた。その後、Rustらしく、タグでの分岐をEnumの型タグによる分岐に修正した。処理漏れもなくなるし、余計な初期化や分岐も無くなった。短くて情報量が多い、なんというか「力強い」コードになる。Enumの威力を実感した。

## Evaluator___

Parserが返したASTを再帰的にevalすることで計算結果を得る。

### 複素数演算

複素演算を実装するには、プログラミングだけでなく、複素演算自体の知識も必要となる。ただし、今回の実装では精度を追い求めることはせず、ライブラリ関数を全面的に信用してそれを利用することにする。

rustでは`use num_complex::Complex64;`とすれば、`(re: f64, im: f64)`と言う形の複素数、およびそれらを引数にとる複素関数を使うことができる。

複素数演算の基本はべき乗関数（`power()`）である。これはライブラリにあるので、引数の型（正整数、整数、浮動小数点数、複素数）に会わせて呼んでやるだけで良い。`TokenOp::Hat`がべき乗演算子（`^`）なので、次のようになる。場合分けが煩雑だが仕方がない。

```rust
    if let Node::BinOp(tok, lhs, rhs) = n {
        let lhs = eval(env, lhs)?;
        let rhs = eval(env, rhs)?;
        match tok {
// .....................中略.......................
            Token::Op(TokenOp::Hat) => {
                if let Node::Num(nr) = rhs {
                    if let Node::Num(nl) = lhs {
                        if nr > 0 {
                            return Ok(Node::Num(nl.pow(nr as u32)));
                        } else {
                            return Ok(Node::FNum((nl as f64).powi(nr as i32)));
                        }
                    } else if let Node::FNum(nl) = lhs {
                        return Ok(Node::FNum(nl.powi(nr as i32)));
                    } else if let Node::CNum(nl) = lhs {
                        return Ok(Node::CNum(nl.powi(nr as i32)));
                    }
                } else if let Node::FNum(nr) = rhs {
                    if let Node::Num(nl) = lhs {
                        return Ok(Node::FNum((nl as f64).powf(nr)));
                    } else if let Node::FNum(nl) = lhs {
                        return Ok(Node::FNum(nl.powf(nr)));
                    } else if let Node::CNum(nl) = lhs {
                        return Ok(Node::CNum(nl.powf(nr)));
                    }
                } else if let Node::CNum(nr) = rhs {
                    if let Node::Num(nl) = lhs {
                        return Ok(Node::CNum(Complex64::new(nl as f64, 0.0).powc(nr)));
                    } else if let Node::FNum(nl) = lhs {
                        return Ok(Node::CNum(Complex64::new(nl, 0.0).powc(nr)));
                    } else if let Node::CNum(nl) = lhs {
                        return Ok(Node::CNum(nl.powc(nr)));
                    }
                }
                return Ok(Node::Num(0));
            }
        }
    }
```


浮動小数点のべき乗が計算できるようになれば、世界でもっとも美しいと言われているオイラーの等式（Euler's Identity）を計算してみたくなるだろう。

```
rc> exp(i * pi)
-1+0.00000000000000012246467991473532i
```

虚数部に多少の計算誤差はあるが、`-1`という結果を得られた。

もうひとつ計算してみたいものが、`i^i`だ。虚数単位の`i`を、虚数単位乗したもの。一目見たところ複雑に見えるが、答えは実数になる。

`exp(log(z)) = z`と定義することができるので、`a^b = exp(b * log(a))` と定義することができる。`log(i)`の主値が`(1/2) * pi * i`なので`i * log(i)`の値の1つは`-(1/2) * pi`。よって`i^i = exp(-pi/2)`が解の1つである。これは実数。実際に計算してみた。

```
rc> i^i
0.20787957635076193+0i
```

虚数の虚数乗が実数というだけでなく、字面としても`i^i`は顔文字見たいでおもしろいし、高校数学の知識で計算することができる。読み方も「あいのあいじょう」となかなか良い。


## MyError

Rustではエラー処理に`Result<T,E>`を使う。

最初の実装では`Result<T, String>`が簡便でよいだろう。しかし、それではRustの力を十分に活用できていない。

エラー処理のためにEdition2018では`?`構文が導入されている。それを有効に使うためには`String`でエラーを返すのではなく、独自エラー型を導入しておくほうが便利だ。2020年現在、Rustのエラー処理の状況は変化が進行中だ。現時点でもっとも有力な方法は「MyErrorを定義してthiserrorで実装を付ける、それ以外はanyhowでエラーを返す」のようだ。`try!`を使う方法は現在は推奨されていない。

独自エラーを`enum MyError`で定義して`Error`トレイトを継承しておく。下位ライブラリが返すエラーを`From`で変換して`MyError`の一種にする。そうすると、自分のコード内ではとにかく`Result<T,MyError>`を返すことができる。そのように返す型が統一されていれば、`?`でエラーチェックをして、ショートカットリターンが可能だ。

### thiserror

現時点で`thiserror`の日本語による詳しい説明はあまり見当たらない。

`thiserror`は自作のエラーにトレイト実装（`Error`, `Display`, `From`など）を簡単に付けるためのクレートである。`Cargo.toml`に次のように書くことで使えるようになる。

```
[dependencies]
thiserror = "1.0"
```

そのうえで、`use thiserror::Error;`と書けば次のようなマクロが使える。

```rust
#[derive(Error, Debug)]
pub enum MyError {
    #[error("lexer error: {1} {0}")]
    LexerIntError(String, #[source] std::num::ParseIntError),
    #[error("lexer error: {1} {0}")]
    LexerFloatError(String, #[source] std::num::ParseFloatError),
    #[error("parser error: {0}")]
    ParseError(String),
}
```

このように、自分で定義したエラー型に対して、`Display`トレイトを`#[error("...")]`で簡便に定義できることが特徴。フォーマット書式は`fmt!`に準ずる。エラー型は`Enum`でも`Struct`でも良い。

MyErrorを発生させるときは次のようになるだろう。上は、文字列→数値変換で、変換エラーが出た時に、独自の注釈が付いたエラーを返す場合。

```rust
match i128::from_str_radix(&mantissa, radix) {
    Ok(int) => Ok((Token::Num(int), i)),
    Err(e) => Err(MyError::LexerIntError(mantissa, e)),
}
```

次は再帰降順パーサで下位呼び出しがエラーした場合の処置方法。最初の`mul()`呼び出しは左辺（LHS:left hand side）側をパーズ。ここは再帰的に呼ぶだけなので、エラーが起きた場合は`?`でそのまま帰る。ふたつ目の`mul()`呼び出しは右辺のどこかの再帰呼び出しでエラーが起きた場合。これも、このまま上に伝えて帰る。このとき、`i`の範囲が`tok`のインデックスをはみ出していないかをチェックせずに呼び出しているが、呼びだされている`mul`の先頭で（`expr()`の先頭の`tok_check_index!()`のように）チェックしているのでこれはこれで良い。

```rust
fn expr(env: &mut Env, tok: &[Token], i: usize) -> Result<(Node, usize), MyError> {
    if env.is_debug() {
        eprintln!("expr {:?} {}\r", tok, i);
    }
    tok_check_index!(tok, i);

    let (mut lhs, mut i) = mul(env, tok, i)?;
    loop {
        if tok.len() <= i {
            return Ok((lhs, i));
        }
        let tok_orig = tok[i].clone();
        match tok[i] {
            Token::Op(TokenOp::Plus) | Token::Op(TokenOp::Minus) => {
                let (rhs, j) = mul(env, tok, i + 1)?;
                i = j;
                lhs = Node::BinOp(tok_orig, Box::new(lhs), Box::new(rhs));
            }
            _ => {
                return Ok((lhs, i));
            }
        }
    }
}
```

もし、ライブラリが発生するエラーをそのまま使う場合は、最初の例とは異なり、次のように`From`トレイトも自動生成できる。ライブラリが返してきた`std::num::ParseFloatError`を、自動生成した`From`トレイトによって`MyError`に変換して`Result<>`で包んで返す。今回の場合は自分でエラー情報を付加したいので、このようにはしていない。

```rust
#[derive(Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    LexerFloatError(#[from] std::num::ParseFloatError),
}
```

```rust
fn tok_num(chars: &[char], index: usize) -> Result<(Token, usize), MyError> {
    let mut mantissa = String::new();

    /// いろいろな処理

    Ok((Token::FNum(mantissa.parse::<f64>()?), i))
}
```

自前のエラー型を定義して、標準のエラー型からの`From`を定義することのメリットは`?`が使えること。つまり、エラー処理を呼び出し側に放り投げる形のショートカットリターンが使えること。つまり、関数を`Result<T,MyError>`を返すように定義しておけば、ライブラリがエラーを発生した場合はライブラリのエラーから`MyError`に`From`によって変換してリータンできる。もちろん、自前のエラーは`MyError`なので、それもリターンできる。いずれも、呼び出し側でエラー処理を行わなければならない。


## Command___

計算の実行では無く、アプリの動作を変更したいときなどに「コマンド」が用意されている。たとえばデバッグ設定の変更や出力フォーマットの変更などだ。

## システム定義定数___



## ユーザ定義変数___



## システム定義関数___


### `exp()`___

マクロとしての実装

## ユーザ定義関数___

引数をマクロ的に展開。
スタック不要で再帰呼出しが可能。引数の数が限られる。


## オブジェクト指向___

今回は、インタプリタ全体の動作を決定する情報を `struct Env`にまとめ、`&mut env`として必要な関数には引数として渡すようにした。これを、`&mut self`として渡すとオブジェクト指向になる。やってることは同じだが、見た目と書く手間のバランスだ。

### ライフタイム___

## ファイル分割

`main.rs`にはオプション処理など必要最低限のことだけ行って、あとは、`lib.rs`および、そこから呼び出されるライブラリに制御を移す。Lib crateではユニットテストが使えるので、それを最大限有効活用するため。


## 端末制御

別のプロジェクトでは [`termion`](https://docs.rs/termion/1.5.5/termion/)を使っていた。しかしLinuxとmacOSだけでWindowsのサポートはない。`rc`では、Windowsとのクロスプラットフォーム性を重視して[`crossterm`](https://github.com/crossterm-rs/crossterm)を用いるようにした。依存するクレートは増えるがLinuxでもWindowsでも動作する。さらに、こちらのほうがより多機能だ。書き方は異なるが使い方は似ている。

当然であるが、内部のデータ構造の編集操作と、表示部分やイベントハンドリングを分けておくのがコツ。移植性のためだけでなく、編集操作を分けておくとユニットテストもやりやすい。


RAWモードに入る。

```rust
    enable_raw_mode().unwrap();
```

エスケープシーケンスの出力。outputというストリーム（Write Traitを持つ）に対して`queue!`でエスケープ文字をバッファー出力し、最終的に`flush()`する。

```rust
fn result_print<W>(output: &mut W, s: &str)
where
    W: Write,
{
    queue!(
        output,
        style::SetAttribute(style::Attribute::Bold),
        style::SetForegroundColor(style::Color::Yellow),
        style::Print(s),
        style::SetAttribute(style::Attribute::Reset),
    )
    .unwrap();
    output.flush().unwrap();
}
```

キー入力は`read()`で読み取り、`Event::Key(keyev)`で`match`させる。マウス入力もキャッチできる。

```rust
    loop {
        let event = read().unwrap();
        // println!("Event::{:?}\r", event);

        if let Event::Key(keyev) = event {
```

## コマンドラインオプション

コマンドラインオプションを処理するために使われるライブラリとして[`getopts`](https://docs.rs/getopts/0.2.21/getopts/)と[`clap`](https://docs.rs/clap/2.33.3/clap/)が広く知られている。今回はそれほど複雑でないので[`getopts`](https://docs.rs/getopts/0.2.21/getopts/)を用いた。

`let mut opts = Options::new();`でオプションオブジェクトを作り、`opts.optflag()`などのメソッドでオプションを定義していく。その後、`opts.parse(&args[1..])`で引数を解析させ、`match`でオプションが指定されていた時の処理を実装していく。

### バージョン情報の自動取得

通常`--version`などのオプションでプログラムのバージョンを表示できるようにする。それらを自動的に手際よく処理する方法がある。`cargo`ではビルド時にいろいろな環境変数が設定されるが、`Cargo.toml`の中で定義した`semver`が`CARGO_PKG_VERSION`という環境変数にセットされる。それを用いれば、コード中にバージョンを手動で定義しなくても`Cargo.toml`のバージョンを引っ張ってくることができる。ただし、`Cargo.toml`の`semver`も手動定義だ。gitのHash値をバージョン名に含めれば、それは自動的に一意となる。`build.rs`で`git rev-parse`などのコマンドを実行し、`git_commit_hash.txt`というファイルに保存しておく。コード中では`include!()`でそれを取り込めば、自動的に一意なバージョン識別子が得られる。

`build.rs`
```rust
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let commit_hash = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("failed to execute command \"git rev-parse HEAD\"");
    let dest_path = format!("{}/src/git_commit_hash.txt", env!("CARGO_MANIFEST_DIR"));
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(
        format!(
            "\"{}\"",
            String::from_utf8(commit_hash.stdout.to_vec())
                .unwrap()
                .replace("\n", "")
        )
        .as_bytes(),
    )
    .unwrap();
}
```

```rust
    if matches.opt_present("v") {
        let version = env!("CARGO_PKG_VERSION");
        let git_commit_hash = include!("git_commit_hash.txt");
        println!("{} {}-{}", program, version, git_commit_hash);
        std::process::exit(0);
    }
```

参照: [https://matsu7874.hatenablog.com/entry/2019/12/24/080000](https://matsu7874.hatenablog.com/entry/2019/12/24/080000)

[`git-version`](https://docs.rs/git-version/0.3.4/git_version/)というクレートも同様の目的のようだ。



## テスト

電卓も言語処理系だ。入力があって出力がある。定めた仕様どおりにコードを書くと、とりあえずは動く。しかし、言語処理系は構文要素の組み合わせが無限にある。ちょっと動くようになったら自分でドッグフードを食べてみるが、いろいろなケースでバグに出会う。そういうのはすぐにテストケースにまとめて、それが正常実行できるようにデバッグを行う。テストケースが溜まっていき、ワンコマンドで回帰テストを実行する。

テストがあるからこそ、チャレンジングなリファクタリングも実行できる。後半になればなるほど、開発スピードが上がる。

VS-Codeと組み合わせた場合、`#[test]`を付けた関数はクリック可能なヒントが付く。それをクリックすれば、そのテストだけを実行できる。

![images/screenshot-001.png](images/screenshot-001.png)

## インクリメンタルな開発

これも毎回書いているが、インクリメンタルな開発が推奨される。

たとえば、電卓に定数"pi"の機能を実装するとき。「将来的にユーザ定義変数も追加したいな」とか思い描いて、最初から辞書検索機能を実装するのは悪手である。まず"pi"だけが確実に機能する実装を作り、テストを行う。そこから、ユーザ定義変数などの追加機能を実装するのだ。

当然、高機能な電卓ではユーザ定義の変数、関数の機能が求められる。場合によってはスコープなどで入れ子になったネームスペースなども必要になるだろう。そこで要求仕様書にそう書くとしよう。すると、入れ子になったネームスペースを参照した、変数・関数辞書ができるまで、最初の変数・関数のテストが実行されない。それまでに要する開発工数、テスト作成工数は増大する。一方、実装最初のパーサのテストは、固定の"pi"が解釈できさえすば実行できる。最初の間に合わせの実装から、最終的な実装への書き換えの手間が発生するが、インクリメンタルに開発することでスコープを小さく保つことのメリットは十分に見合う。

また、仕様書には想定される実装に基づいたデータ構造やモジュール構造に依存した記述がなされる。それらは、実装・テストによる確認を経ていないため、最適解ではなかったり、設計書の記述時点で間違っていたりする。ソフトウエアは複雑なので、実装・テストによる確認を経ずに、頭の中だけで仕様を書き下して、それがバグ無く一発で動くことは極めて困難だ。

これが「仕様定義」→「実装」のウォーターフォール開発がソフトウエアに不適合である主要な理由だ。

経験のある開発者は、たとえ壮大な仕様書を渡されたとしてもその意を汲み取りながら、ステップバイステップに分解して、最終的には要求を満たすが仕様書よりベターな実装を仕上げるだろう。しかし、中級の開発者にそれを期待するのは難しい。まして、実装を知らない企画者の考えた「仕様」がどうなるか、考えただけでも恐ろしい。

## 開発環境

rustのコンパイラは厳しいが親切。コンパイル時にエラーが出まくるのは有名な話だが、どう直せばよいかも提示してくれる。初心者が遭遇するエラーは典型的なので、コンパイラが提示するヒントにしたがって修正していけば、たいてい解消される。VS Codeを使っていると、コンパイラを通さずとも赤線が引かれるので、注釈通りに修正していけばよい。コンパイラエラーだけでなく、clippyも適切な改善をアドバイスしてくれる。ひととおり修正が完了したら、コミットする前に`cargo fmt`と`cargo clippy`を実行する習慣をつけるとよいだろう。

グローバル変数が事実上使えない、ボローチェッカーが厳しいなどもある。これも、コンパイラの言う通りに修正することで、メモリーリークやデータ競合リスクがないコードを書くギブスみたいなものだ。慣れればそれらに引っかからないコードが書けるようになる。そのためには、自然とデータの寿命と所有者・だれが変更するか、に気を使わなければならない。それらは、当然Cなどでも、きちんと考えられているべき事項なのだ。

コンパイラがザルだと、全部自分でチェックしなければならない。ストレス。

今時の環境なので、テストやドキュメンテーションも言語設計レベルで統合されており、どのツールを使うかなどの迷いがない。フォーマットも宗派がない。そのような自転車置き場の議論がないこともRust開発環境の快適さだと思う。C言語の`{`,`}`のインデントの議論で丸一日潰れたことを思い出す。

VS Codeの環境がGitと密に結合していることも、テスト＆修正＆コミットのサイクルを細かく回すことに役立っている。従来の「保存」の代わりがコミットのようなものだ。そうなることで、自動セーブ機能がさらに合理的なものとなる。保存しないと、ビルドにも反映しないし、何かの拍子に編集が失われることは明らかに不便だ。保存したとしても、それはテストが通っていないのであれば、本当に「保存」する価値があるのだろうか。テストが通れば、その変更単位についてコミット・コメントが付く。理由・履歴・差分を参照しながら、いつでもその場所に戻れるのが「セーブ」ポイントというものだろう。ただし、これは個人開発の場合。グループ開発の場合にはPushする前にコミットを（他の人にもわかる程度には）きれいにしましょう。

Windows版の開発とVS-Code remoteを使ったWSL2上の開発が、ほぼ同じ環境で実施できるのも素晴らしい。

## Static link

### Linux

Linuxでは[MUSL](https://ja.wikipedia.org/wiki/Musl)がサポートされているので、外部ライブラリに依存していない場合はとくに、スタティックリンク・バイナリを作るのは簡単だ。`-musl`ターゲットの場合、スタティックリンク用の外部ライブラリが有れば、それらもスタティックリンクしてくれる。

```
$ rustup target add x86_64-unknown-linux-musl     ## ターゲットを追加
$ rustup show   ## インストール済のターゲットに x86_64-unknown-linux-musl があることを確認
$ cargo build --release --target=x86_64-unknown-linux-musl  ## ターゲットを指定してビルド
$ ldd target/x86_64-unknown-linux-musl/release/rc
    not a dynamic executable
```

ビルド済のバイナリが[download/](download/) からダウンロードできる。

### Windows

`.cargo/config`に次のように書いておけば、`x86_64-pc-windows-msvc`環境でstatic linkオプションを隣家に対して渡してくれる。
```
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```
ビルド済のバイナリが[download/](download/) からダウンロードできる。

