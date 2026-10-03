<!-- NONOS. AGPL-3.0-or-later. -->

# Fuzzing

Four targets, built with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) on the
nightly the formatter is pinned to. Each holds a property over any input, not only
over inputs that parse.

| Target | What must hold |
|---|---|
| `front_end` | the edition 2026 lexer and parser return on any text, and every diagnostic points inside it with a primary label |
| `build_run` | any text is built as an edition 2026 program without a panic, and a program that builds runs with its compiled machine program and its reference run in agreement |
| `fmt` | the formatter never changes a token or a comment, and a formatted file formats to itself |
| `edition_2025` | the edition 2025 compiler and evaluator take any text without a panic |

A target reads its input in one of two modes, chosen by the first byte: as UTF-8 text,
or as a string of the language's words, so a run reaches the parser's depths quickly.

```sh
cargo install cargo-fuzz --locked
cd fuzz
python3 seed.py --root .. --out corpus
cargo +nightly-2026-01-16 fuzz run -O build_run corpus/build_run -- -max_total_time=600
```

`seed.py` writes the repository's own programs, in the text mode, into each target's
corpus. A failing input lands in `artifacts/<target>/`; `cargo fuzz run <target> <file>`
replays it.
