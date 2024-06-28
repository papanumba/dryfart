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

DryFart is a toy language that I've been developing while learning about compilers, languages, Rust, and more. Its features range from the ugly syntax of some esoteric languages to the uselessness of toy languages, together with the slow<sup><a name="footnote1">1</a></sup> performance of dynamically typed, interpreted languages.

The project contains:
- `InFarter`: tree-walk interpreter and bytecode compiler, written in Rust.
- `FlatVM`: stack-based bytecode VM, written in C++ and a bit of C. I started it by following the one from [Crafting Interpreters](https://craftinginterpreters.com/a-bytecode-virtual-machine.html), but then I had to change things and rewrote most of it in C++.
- `DFartEd`: a very small editor with syntax highlighting, written in Python. Still has a lot of bugs

The documentation about the language will be in the `doc` folder: [Start here](/doc/index.md).

<sup>[1](#myfootnote1)</sup> The tree-walk interpreter **is** slow, but the VM is comparable to CPython and sometimes a bit faster.

## Build

### InFarter

Since it's written in Rust, `cargo` is the best way for building.

```bash
cd infarter
cargo build --release
```

Then, the binary will be located at `./target/release/`.

### FlatVM

Currently uses GNU Make, so follow:

``` bash
cd flatvm
make release
```

### DFartEd

You'll need python3 and PyQt5. It is important that InFarter and FlatVM are built, on release mode, so that DFartEd can find their path.

## Usage

Suppose you have `example.df`:
- To run it on the interpreter, run `./infarter example.df`
- To compile it to bytecode, run `./infarter t example.df`. Then a file `example.dfc` will be created in the same folder as `example.df`. To compile it with optimizations, change `t` into `to`.
- To run the bytecode, run `./flatvm example.dfc`.
- To disassemble the bytecode, run `./flatvm d example.dfc`.

## License

This project is licensed under [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html).

