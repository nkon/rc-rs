rc - simple terminal calulator
==============================

```
$ rc
rc>    # prompt.
rc> 1+2*3
7
rc> 
```

* `rc` is better `bc`, which is famous command line caluclator.
* `rc` is designed for schientific/engineering caluclation.
* `rc` runs on terminal on Windows/Linux/Mac/Raspberry Pi.
* `rc` is written by rust. Example of parser by rust.

## feature

* Calculator
    + Arithmetic operations including multiple parentheses
    + support integer and float
    + k/M/G/T/m/u/n/p ... suffix
    + binary(0b....), decimal, hexdecimal(0x....) format.
        - '_' ... separator
    + built-in functions
        - Arithmetic: sin/cos/abs/...
        - Engineering: E12/para/...
        - unit conversion: inch2mm/mm2inch/...
    + user defined variable/function
* REPL
    + Line Edit/History
    + hilight parlens
    + Online help
    + Script mode(input from stdin, output to stdout)
* Install
    + Single binary
    + Support Linux/Windows/Mac


## screen capture


## Install

### build from source

```
$ git clone https://github.com/nkon/rc-rs.git
$ cargo install --path .                       ## installed to ~/.cargo/bin/rc
```

### binary download




## future

* REPL/CLI/Script
* Graph
* Single binary release
    + cross build for Linux, Windows, Mac
* L10N using gettext-rs
* Read .rc_rc as the init file, in which user defines constants and functions.
