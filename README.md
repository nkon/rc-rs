rc - simple terminal calulator
==============================

```
$ rc
rc>    # prompt.
rc> 1+2*3
7
rc> 
```

* `rc` simmilar to `bc`, which is famous command line caluclator.
* `rc` is designed for schientific/engineering caluclation.
* `rc` runs on terminal on Windows/Linux/Mac/Raspberry Pi.
* `rc` is an example of imprementation of parser written by rust.

## Features

* Calculator
    + Arithmetic operations including multiple parentheses
    + support integer and float
    + k/M/G/T/m/u/n/p ... suffix
    + binary(0b....), decimal, hexdecimal(0x....) format.
        - '_' ... separator
    + built-in functions
        - Arithmetic: sin/cos/abs/...
        - Engineering: E12/pararell(`//`)/...
        - unit conversion: inch2mm/mm2inch/...
    + user defined variable/function
* REPL
    + Line Edit/History
    + hilight parlens    <- not yet
    + Online help        <- not yet
    + Script mode(input from stdin, output to stdout)
    + initialize file (`~/.rc_rc`)
* Install
    + Statically linked single binary.
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

## Future

* REPL/CLI/Script
    + hilight parlens
    + Online help
* Graph
* L10N using gettext-rs
