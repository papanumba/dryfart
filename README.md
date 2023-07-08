# DryFart
A programming language as Dry as a Fart

## Description
DryFart is a halfway-esoteric toy language, interpreted with InFarter. Its
features range from the ugly syntax of some esoteric languages to the
uselessness of toy languages, altogether with the slow performance of
interpreted languages.

The documentation about the language will be in the `docs` folder.
[Start here](/docs/start.md).

## Build
By now, there's only one program in the repo, but soon will be more.
InFarter is written in Rust, so just go to the `infarter` directory and build
it with cargo:

```bash
cd infarter
cargo build --release
```

## Usage
Once InFarter is finished, one shall be able to use `infarter` to run a DryFart
script `x.df` from the command line by using:

```bash
infart x.df
```

## License
All code in this repo is licensed under
[GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html).
