[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

![README-j.md](README-j.md)

Table of Contents
-----------------

* [Features](#features)
* [Screen capture](#screen-capture)
* [Install](#install)
    * [Build from source](#build-from-source)
    * [Build with musl(static linked binary)](#build-with-muslstatic-linked-binary)
* [Future](#future)
* [Design Notes](#design-notes)

Created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)

rc - simple terminal calculator
==============================

```
$ rc
Ctrl-c to exit
rc>     # prompt. '#': comment
rc> 1+2*3
7
rc> 1/(2*pi*(3k//4.7k)*0.22u)    # cut off frequency of CR LPF. "//" means parallel. T/G/M/k/m/u/n/p: SI postfix.
395.0654615756267
rc> format(16, sep4)            # output_format
format(radix = 16, separate = 4)
rc> 0xdead_beef - 0xcafe_babe
0x13af_0431
rc > format(10, sep3)
rc> i^i
0.20787957635076193+0i
rc> exp(i*pi)                   # Euler`s equation
-1+0.00000000000000012246467991473532i
rc> a=2                         # user defined variable
rc> a*3
6
rc> defun(add, _1 + _2)         # user define function, _1,_2,...,_9 are parameters
rc> add(10,add(2,a))            # recursive user defined function call
14
rc> exit()                      # exit REPL, Ctrl-c to exit as well.
$
```

* `rc` similar to `bc`, which is a famous command line calculator.
* `rc` is designed for scientific/engineering calculation.
* `rc` runs on terminal on Windows/Linux/Mac/Raspberry Pi.
* `rc` is an example of implementation of parser written by rust.

## Features

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

#### Build with musl(static linked binary)

In case of x86_64-linux.

```
$ rustup target add x86_64-unknown-linux-musl
$ rustup show   ## list of installed toolchain -> check: x86_64-unknown-linux-musl is exist.
$ cargo build --release --target=x86_64-unknown-linux-musl
$ ldd target/x86_64-unknown-linux-musl/release/rc
    not a dynamic executable
```

## Download

Static linked executables for some platforms are available in [download/](download/) directory.

## Future

* Highlight parentheses
* Online help
* Graph
* L10N using gettext-rs

## Design Notes

[NOTE.md](NOTE.md)
