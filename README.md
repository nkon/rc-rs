rc - simple terminal calulator
==============================

```
$ rc
rc>    # prompt.
rc> 1+2*3
7
rc> 
```


I used `iMemo` on Windows for schientific caluclation.
`rc` is a tool to replace `iMemo`.

https://www.vector.co.jp/soft/win95/personal/se213459.html

## feature

* Calculator
    + Arithmetic operations including multiple parentheses
    + support integer, bignum and float
    + k/M/G/T/m/u/n/p ... suffix
    + binary(0b....), decimal, hexdecimal(0x....) format.
        - '_' ... separator
    + built-in functions
    + user defined variable/function
* REPL
    + history
    + hilight parlens
    + Online help
* Install
    + Single binary
    + Support Linux/Windows/Mac
## imprement

* Fully hand written parser
    + Training

## future

* Line Edit/History
* REPL/CLI/Script
* Graph
* Single binary release
    + cross build for Linux, Windows, Mac
* Use L10N using gettext-rs
