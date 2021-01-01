[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Table of Contents
-----------------

* [主な機能と特徴](#主な機能と特徴)
* [Screen capture](#screen-capture)
* [Install](#install)
    * [Build from source](#build-from-source)
* [Download](#download)
* [将来の予定](#将来の予定)
* [Design Notes](#design-notes)

Created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)

[README.md](README.md)

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
rc> format 16 sep4              # 出力フォーマットを16進、4桁区切りに変更。コマンドは()も,も無し
format(radix = 16, separate = 4)
rc> 0xdead_beef - 0xcafe_babe   # 0x...は16進、0b....は2進
0x13af_0431
rc > format 10 sep3
rc> i^i                         # 複素数演算。i,j は虚数単位
0.20787957635076193+0i
rc> exp(i*pi)                   # オイラーの等式
-1+0.00000000000000012246467991473532i
rc> a=2                         # ユーザ定義変数
rc> a*3
6
rc> defun add _1 + _2           # ユーザ定義関数。_1,_2,...,_9 は引数。defunは関数定義のコマンド
rc> add(10,add(2,a))            # 再帰呼出しも可能
14
rc> constant                    # 定数. cmd, variable, func, user_func なども
e = 2.718281828459045
pi = 3.141592653589793
eps = 0.0000000000000002220446049250313
i = 0+1i
j = 0+1i
inch2mm = 25.4
feet2mm = 304.8
oz2g = 28.3495
rc> 2+3
5
rc> ans*7                       # 最後の計算結果は ans に保存される
35
rc> exit                        # 終了。Ctrl-c も終了
$

$ rc 1+2+3                      # コマンド引数に式を書いてもOK
6
$

$ rc
Ctrl-c or "exit()" to exit
rc> history_max 10              # `history_max` > 0のときヒストリーが `~/.rc.history`に保存される
history_max 10
rc> sin(1)
0.8414709848078965
rc> cos(2)
-0.4161468365471424
rc> exp(3)
20.085536923187664
rc> history                     # ヒストリーの表示
3 sin(1)
2 cos(2)
1 exp(3)

rc> history 2                   # ヒストリーからの再実行
cos(2)
-0.4161468365471424
rc> 
```

`rc`はRustで作られていて、Windows, Linux（Raspberry Piを含む）の上で動作確認されています。
ビルドすればMacでもたぶん動きます。

## 主な機能と特徴

* 計算機
    + 演算子の優先順位、括弧()
    + 整数、浮動小数点数、複素数
    + k/M/G/T/m/u/n/p ... SI suffix
    + 2進(0b....), 10進, 16進(0x....)
        - '_' を桁区切りとして使える `123_000_000`
    + 組込み関数・定数
        - 算術関数: sin/cos/abs/...今後拡充予定
        - エンジニアリング関数: E12/並列抵抗演算子(`//`)/...
        - 単位変換定数: inch2mm/feet2mm/oz2g/...
    + ユーザ定義変数・関数
* ユーザインターフェイス
    + 行編集、ヒストリー
    + スクリプトモード(input from stdin/command line argument, output to stdout)
    + ユーザ初期化ファイル(`~/.rc_rc`)
    + コメント `#...`
    + 出力フォーマット format sep4 radix16 -> 0x200_1fee
    + 対応する括弧のハイライト
* インストール
    + Linux/Windows/Mac のバイナリがダウンロード可能
    + RaspberryPiはソースからビルド


## Screen capture

![screen capture](images/screencam-002.gif)

## Install

### Build from source

```
$ git clone https://github.com/nkon/rc-rs.git
$ cd rc-rs
$ cargo install --path .                       ## installed to ~/.cargo/bin/rc
```


## Download

Windows, Linux, macOS用のバイナリが[Release](https://github.com/nkon/rc-rs/releases)ページからダウンロードできます。


## 将来の予定

* Online help
* Graph
* L10N using gettext-rs
* Solver

## Design Notes

[NOTE.md](NOTE.md)
