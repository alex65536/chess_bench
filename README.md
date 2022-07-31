# Tests and benchmarks for chess implementations in Rust

This repository contains tests and benchmarks to compare various chess implementations against each other. More precisely, the following things are implemented:

Tests:

- [Perft](https://www.chessprogramming.org/Perft)
- [Hperft](#hperft) (see below)
- Reimplementation of SoFCheck's [selftest](https://github.com/alex65536/sofcheck/tree/master/selftest)

Benchmarks:

- [Perft](https://www.chessprogramming.org/Perft)
- [Hperft](#hperft) (see below)

The following implementations are tested now, with different support level:

| Implementation | perft | hperft | selftest |
|---------------:|:-----:|:------:|:--------:|
| [chess]        | ✔️ | ✔️ | ✔️ |
| [owlchess]     | ✔️ | ✔️ | ✔️ |
| [shakmaty]     | ✔️ | ✔️ | ❌ |
| [cozy-chess]   | ✔️ | ✔️ | ❌ |
| [pleco]        | ✔️ | ✔️ | ❌ |

[chess]: https://github.com/jordanbray/chess
[owlchess]: https://github.com/alex65536/owlchess
[shakmaty]: https://github.com/niklasf/shakmaty
[cozy-chess]: https://github.com/analog-hors/cozy-chess/
[pleco]: https://github.com/sfleischman105/Pleco

## Running tests

Just do

```
$ cargo test
```

## Running benchmarks

You will need Python 3 to do this.

First, install the pre-requisites:

```
$ pip install matplotlib
$ cargo install cargo-criterion
```

Then, do the following:

```
$ cd run_perft
$ ./run.py
```

This will run the benchmarks and build nice plots into `run_perft/perft.svg` and `run_perft/hperft.svg` (for Perft and Hperft, respectively).

You can also run benchmarks via raw `cargo criterion`, but in this case you won't obtain plots comparing different implementations.

## Extending

You can easily add your chess implementation (if it's written in Rust, of course).

1. Add your implementation to `src/impls`. See [existing implementations](src/impls/chess.rs) as an example. You need to implement `Test` for selftest and `Perft` for Perft/Hperft.
2. To add your implementation to Perft/Hperft tests and benchmarks, modify [`impls::all_perft`](src/impls/mod.rs#L7).
3. To add your implementation to selftest, modify [`run`](src/bin/selftest.rs#L52) function in the binary and add a new test into [`tests/selftest.rs`](tests/selftest.rs#L23-L26).
4. If your chess implementation exists as a crate on [crates.io](https://crates.io), then feel free to submit a PR :)
5. Enjoy ;)
