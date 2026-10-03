<!-- NONOS. AGPL-3.0-or-later. -->

# Changelog

The changes of each release of the compiler, newest first. A release's section is its
notes on GitHub: the release workflow publishes the section whose heading is the tag's
version. The language edition is versioned apart from the compiler.

## 0.1.0

The first release of edition 2026, a typed language whose programs compile to a register
machine whose runs are proven with a STARK. Edition 2025 programs still compile, with the
fixes below.

### The language

- Types: `field`, `bool`, `u8` to `u64`, `i8` to `i64`, `usize`, tuples, arrays, structs,
  enums, type aliases, and generics over types and constants, with `impl` blocks,
  methods and `Self`.
- `match` checked for exhaustiveness (E0400), with the missing case written as a pattern,
  and unreachable arms reported (W0004).
- Secret flow: `public` and `secret` labels, and a value computed from a secret reaching
  `main`'s result only through `declassify` (E0600).
- Failure of a run on a false `assert`, an overflow, a division by zero, an index out of
  bounds, an oversized shift, a failing `checked_from` and a `while` past its `limit`;
  a failed run has no proof.
- Modules in their own files, and packages with a `zkolang.toml` and path dependencies.
- `SPEC.md`, the normative reference.

### The compiler

- A front end that recovers from errors and reports each mistake once, with the names
  closest to one that does not resolve.
- A typed IR with a reference interpreter, and an SSA IR over the field with constant
  folding, common subexpressions, dead code and range checks dropped where implied.
- Gadget expansion, register allocation, and a check of the emitted machine program
  against the SSA; each run of a compiled program beside the reference interpreter, a
  disagreement reported as a compiler bug.
- Proofs that blind every trace column from a fresh seed.
- The constraint ledger, `docs/audit/constraints.md`: every constraint the compiler writes,
  why it is sound, and the test that rejects a run breaking it.

### The standard library

- `std`, written in zKølang: the prelude with `Option` and `Result`, `std::array`,
  `std::cmp`, `std::poly`, `std::curve`, `std::hash` and `std::merkle`, and its generated
  reference, `docs/stdlib.md`.

### Tools

- `zkolang run`, `check` (with `--cost`, `--declassify` and `--json`), `test`, `abi`,
  `doc`, `fmt`, `explain`, `key` and `fee` for edition 2026.
- `zkolang check` of a crate with no `fn main` checks it as a library.
- Cost warnings W0100, W0101 and W0102, and the cost report of `check --cost`.

### Assurance

- Lean 4 proofs of the constraint ledger's guards, bit decomposition, field bits, division
  and bounds checks, beside the standard gadgets; `lean/Axioms.lean` lists every theorem
  with its axioms.
- Four fuzz targets: the front end, building and running, the formatter, and the edition
  2025 compiler.
- Every program in the README checked by a test.
- CI with pinned actions, read-only tokens, a pinned Rust, and a gate on the repository's
  rules.

### Edition 2025

- The audit of the 2025 compiler, `docs/audit/current-compiler.md`, each fix with a test,
  and every program in the tree pinned to its compiled form.
- `docs/migration-2026.md`, each of its pairs proven to give the same outputs in both
  editions.

### Changed

- The prover, `nonos-stark`, comes from the STARKs repository at a pinned commit, and the
  vendored copies are removed. Its default build keeps 24 bytes of each Merkle digest, so
  registration roots and verifier keys differ from those of earlier builds.
- The note commitment of `circuits/shield/note_commit.zkl` follows the STARKs layout,
  `cm = compress([value_lo, value_hi, asset, NOTE_DOMAIN], compress(spend_pk, blinding))`.
