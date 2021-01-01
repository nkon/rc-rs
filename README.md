[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Table of Contents
-----------------

* [Features](#features)
* [Screen capture](#screen-capture)
* [Install](#install)
    * [Build from source](#build-from-source)
* [Download](#download)
* [Future](#future)
* [Design Notes](#design-notes)

Created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)

[README-j.md](README-j.md)

rc - simple terminal calculator
==============================

```
$ rc
Ctrl-c to exit
rc>     # prompt. '#': comment
rc> 1+2*3
7
rc> 1/(2*pi*(3k//4.7k)*0.22u)   # cut off frequency of CR LPF. "//" means parallel. T/G/M/k/m/u/n/p: SI postfix.
395.0654615756267
rc> E12(1234)                   # round to E12 series
1200
rc> format 16 sep4              # output_format. command does not use "(",")",",".
format radix = 16 separate = 4
rc> 0xdead_beef - 0xcafe_babe
0x13af_0431
rc > format 10 sep3
rc> i^i
0.20787957635076193+0i
rc> exp(i*pi)                   # Euler`s equation
-1+0.00000000000000012246467991473532i
rc> a=2                         # user defined variable
rc> a*3
6
rc> defun add _1 + _2           # user define function, _1,_2,...,_9 are parameters
rc> add(10,add(2,a))            # recursive user defined function call
14
rc> constant                    # list constants. cmd, variable, func, user_func also work.
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
rc> ans*7                       # ans is the variable of last answer
35
rc> exit                        # exit REPL, Ctrl-c to exit as well.
$

$ rc 1+2+3                      # command line expression
6
$

$ rc
Ctrl-c or "exit()" to exit
rc> history_max 10              # `history_max` > 0 => save/load history from/to `~/.rc.history`
history_max 10                  # write `history_max 100` to `~/.rc_rc`
rc> sin(1)
0.8414709848078965
rc> cos(2)
-0.4161468365471424
rc> exp(3)
20.085536923187664
rc> history                     # show history
3 sin(1)
2 cos(2)
1 exp(3)

rc> history 2                   # re-execute from history
cos(2)
-0.4161468365471424
rc> 
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
        - unit conversion: inch2mm/feet2mm/oz2g/...
    + user defined variable/function
* REPL
    + Line Edit/History
    + Script mode(input from stdin/command line argument, output to stdout)
    + Initialize file (`~/.rc_rc`)
    + Comment `#...`
    + format sep4 radix16 -> 0x200_1fee
    + Highlight parentheses
* Install
    + Download binary for Linux/Windows/Mac

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

Windows, Linux, macOS binaries are available from [Release](https://github.com/nkon/rc-rs/releases) page.

## Future

* Online help
* Graph
* L10N using gettext-rs
* Solver

## Design Notes

[NOTE.md](NOTE.md)
