[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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

rc - simple terminal calulator
==============================

```
$ rc
rc>    # prompt.
rc> 1+2*3
7
rc> 
```

* `rc` simmilar to `bc`, which is a famous command line caluclator.
* `rc` is designed for schientific/engineering caluclation.
* `rc` runs on terminal on Windows/Linux/Mac/Raspberry Pi.
* `rc` is an example of imprementation of parser written by rust.

## Features

* Calculator
    + Arithmetic operations including multiple parentheses
    + support integer and float
    + k/M/G/T/m/u/n/p ... suffix
    + binary(0b....), decimal, hexdecimal(0x....) format
        - '_' ... separator, i.e., `123_000_000`
    + built-in functions
        - Arithmetic: sin/cos/abs/...
        - Engineering: E12/pararell(`//`)/...
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

Static linked executables for some platforms are aviailable in [download/](download/) directory.

## Future

* Hilight parlens
* Online help
* Graph
* L10N using gettext-rs

## Design Notes

[NOTE.md](NOTE.md)
