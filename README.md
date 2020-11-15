rc - simple terminal calulator

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

* Arithmetic operations including multiple parentheses
* support integer, bignum and float
* k/m/u... suffix
* binary, decimal, hexdecimal format.
* some functions

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
