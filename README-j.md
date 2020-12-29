[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Table of Contents
-----------------

* [機能](#機能)
* [Screen capture](#screen-capture)
* [Install](#install)
    * [Build from source](#build-from-source)
* [Download](#download)
* [将来の予定](#将来の予定)
* [Design Notes](#design-notes)

Created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)

rc - simple terminal calculator
==============================

rcはターミナル上で動作する科学技術計算用の計算機です。
コマンドプロンプトに`rc`とタイプすると起動して、rcのプロンプト`rc> `を表示します。

使い方の例。

```
$ rc
Ctrl-c to exit
rc>                             # rc> はプロンプト。#から後はコメント
rc> 1+2*3                       # 足し算より掛け算が優先される
7
rc> 1/(2*pi*(3k//4.7k)*0.22u)   # CR-LPFのカットオフ周波数の計算。
                                # "//" は抵抗の並列を表す演算子。コンデンサなら直列。
                                # T/G/M/k/m/u/n/p のSIポストフィックスが使える
395.0654615756267
rc> E12(1234)                   # E12シリーズに丸める
1200
rc> format(16, sep4)            # 出力フォーマットを16進、4桁区切りに変更
format(radix = 16, separate = 4)
rc> 0xdead_beef - 0xcafe_babe   # 0x...は16進、0b....は2進
0x13af_0431
rc > format(10, sep3)
rc> i^i                         # 複素数演算。i,j は虚数単位
0.20787957635076193+0i
rc> exp(i*pi)                   # オイラーの等式
-1+0.00000000000000012246467991473532i
rc> a=2                         # ユーザ定義変数
rc> a*3
6
rc> defun(add, _1 + _2)         # ユーザ定義関数。_1,_2,...,_9 は引数
rc> add(10,add(2,a))            # 再帰呼出しも可能
14
rc> constant()                  # 定数. cmd(), variable(), func(), user_func() なども
e = 2.718281828459045
pi = 3.141592653589793
eps = 0.0000000000000002220446049250313
i = 0+1i
j = 0+1i
rc> 2+3
5
rc> ans*7                       # 最後の計算結果は ans に保存される
35
rc> exit()                      # 終了。Ctrl-c も終了
$
```

`rc`はRustで作られていて、Windows, Linux（Raspberry Piを含む）の上で動作確認されています。
ビルドすればMacでもたぶん動きます。

## 機能

* Calculator
    + Arithmetic operations including multiple parentheses
    + support integer and float
    + k/M/G/T/m/u/n/p ... suffix
    + binary(0b....), decimal, hexadecimal(0x....) format
        - '_' ... separator, i.e., `123_000_000`
    + built-in functions
        - Arithmetic: sin/cos/abs/...
        - Engineering: E12/parallel(`//`)/...
        - unit conversion: inch2mm/mm2inch/...
    + user defined variable/function  <- not yet
* REPL
    + Line Edit/History
    + Script mode(input from stdin, output to stdout)
    + Initialize file (`~/.rc_rc`)
    + Comment `#...`
    + input format(separator: 123_456, radix: 0x55aa)
    + output_format(sep4, radix16) -> 0x200_1fee
* Install
    + Statically linked single binary
    + Support Linux/Windows/Mac


## Screen capture


## Install

### Build from source

```
$ git clone https://github.com/nkon/rc-rs.git
$ cargo install --path .                       ## installed to ~/.cargo/bin/rc
```


## Download

Windows, Linux, macOS binaries are available from [Release](https://github.com/nkon/rc-rs/releases) page.


## 将来の予定

* Highlight parentheses
* Online help
* Graph
* L10N using gettext-rs
* Solver

## Design Notes

[NOTE.md](NOTE.md)
