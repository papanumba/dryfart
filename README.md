<div align="center">
  <h1 align="center">DryFart</h1>
  <h4>A Programming Language as Dry as a Fart</h4>
</div>
<div align="center">
  <a href="https://github.com/papanumba/dryfart/blob/main/doc/index.md"><img alt="Documentation" src="https://img.shields.io/badge/docs-meh-blue"></a>
  <a href="https://www.gnu.org/licenses/gpl-3.0.en.html"><img alt="License" src="https://img.shields.io/badge/license-GPL--3.0-green"></a>
    <a href="https://github.com/papanumba/dryfart"><img alt="License" src="https://img.shields.io/badge/fart-dry-orange"></a>
</div>

:warning: wARNING: this project is still under construction. :construction:

## Description

DryFart is a partly-esoteric toy language that I've been developing while learn about compilers, languages, Rust and more. Its features range from the ugly syntax of some esoteric languages to the uselessness of toy languages, altogether with the slow<sup><a name="footnote1">1</a></sup> performance of dynamically-typed interpreted languages.

The project contains:
- `InFarter`: tree-walk interpreter and bytecode compiler, written in Rust.
- `FlatVM`: bytecode VM, written in C99. I started it by following the one from [Crafting Interpreters](https://craftinginterpreters.com/), but then reworked it and diverged a bit.
- `DFartEd`: a very small editor with syntax highlighting, written in Python3.

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

In its folder, here's a bash script for building, which can work both with `gcc` & `clang`, just change the `$CC` variable. Running it with no argument is the default and "release" build.

``` bash
cd flatvm
./build.sh
```

### DFartEd

You'll need python3 and PyQt5.

## Usage

Suppose you have `example.df`:
- To run it on the interpreter, run `./infarter example.df`
- To compile it to bytecode, run `./infarter t example.df`. Then a file `example.dfc` will be created in the same folder as `example.df`. To compile it with optimizations, change `t` into `to`.
- To run the bytecode, run `./flatvm example.dfc`.
- To disassemble the bytecode, run `./flatvm d example.dfc`.

## License

This project is licensed under [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html).

