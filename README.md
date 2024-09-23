<div align="center">
  <h1 align="center">DryFart</h1>
  <h4>A Programming Language as Dry as a Fart</h4>
</div>
<div align="center">
  <a href="https://github.com/papanumba/dryfart/blob/main/doc/index.md"><img alt="Documentation" src="https://img.shields.io/badge/docs-meh-blue"></a>
  <a href="https://www.gnu.org/licenses/gpl-3.0.en.html"><img alt="License" src="https://img.shields.io/badge/license-GPL--3.0-green"></a>
  <a href="https://github.com/papanumba/dryfart"><img alt="Lines of Code" src="https://img.shields.io/badge/SLOC-~9k-0"></a>
  <a href="https://github.com/papanumba/dryfart"><img alt="yea" src="https://img.shields.io/badge/fart-dry-orange"></a>
</div>

:warning: wARNING: this project is still under construction. :construction:

## Description

DryFart is a toy language that I have been developing as I delve into the world of compilers, languages, Rust, and beyond. It incorporates features that mimic the irksome syntax of esoteric languages while keeping the intrinsic uselessness of toy languages. On top of that, it boasts the characteristically sluggish performance of dynamically-typed interpreted languages.

The project consists of:
- `InFarter`: a tree-walk interpreter and bytecode compiler written in Rust.
- `FlatVM`: a stack-based bytecode virtual machine written in C++ with some C bits stuffed in. Initially implemented by following the magnificient book [Crafting Interpreters](https://craftinginterpreters.com/a-bytecode-virtual-machine.html), but underwent noteworthy modifications and most of it ended up getting rewritten in C++.
- `DFartEd`: a tiny editor written in Python and Qt, with syntax highlighting as its sole and greatest feature.

The documentation about the language will be in the `doc` folder. You can [start here](/doc/index.md).

*The tree-walk interpreter **is** slow, but the VM is comparable to CPython and sometimes even faster.

## Build

### InFarter

As it's written in Rust, `cargo` is the go-to tool.

```bash
cd infarter
cargo build --release
```

Then, the binary will be located at `./target/release/`.

### FlatVM

Currently using GNU Make, so:

``` bash
cd flatvm
make release
```

### DFartEd

You'll need python3 and PyQt5. It is important that InFarter and FlatVM are built on release mode, so the `dfarted.py` can find their path.

Just `cd dfarted` and run `dfarted.py` either by `chmod u+x`'ing it or running it with `python3` (or `python`).

## Usage

Suppose you have a DryFart source `example.df`:
- To run it on the interpreter, run `./infarter example.df`
- To compile it to bytecode, run `./infarter t example.df`. Then a file `example.dfc` will be created in the same folder as `example.df`. To compile it with optimizations, change `t` into `to`.
- To run the bytecode, run `./flatvm example.dfc`.
- To disassemble the bytecode, run `./flatvm d example.dfc`.

With `infarter` being the binary, located in the current folder.

## License

This project is licensed under [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html).

All images and icons under [CC0](https://creativecommons.org/publicdomain/zero/1.0/?ref=chooser-v1).

