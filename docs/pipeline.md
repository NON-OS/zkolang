# The pipeline

Every change runs through these gates before it can land. Each is a workflow under
`.github/workflows`, and the point of writing them down here is that the guarantees
are legible in one place rather than scattered across YAML.

## Correctness

- **rust** (`rust.yml`): `cargo fmt --check` and `cargo clippy -D warnings` on the
  language crates, their API reference built with rustdoc warnings denied, then
  `cargo test --workspace --release`.
- **proofs** (`proofs.yml`): nightly, the same tests with the ignored ones included.
- **zkl** (`zkl.yml`): every circuit, example and standard-library program compiles,
  and the shield circuits prove and refuse as their tests require.
- **lean** (`lean.yml`): `lake build` compiles every proof; a grep gate rejects
  `sorry` and `admit`; and the **axiom audit** runs `Zkolang/Audit.lean`, failing
  if any load-bearing theorem depends on `ofReduceBool` (which `native_decide`
  would introduce) or `sorryAx`. The trust base is checked, not claimed.
- **verify** (`verify.yml`): prints the axioms every public theorem rests on and
  fails if any rests on `sorryAx`.

## Fuzzing

- **fuzz** (`fuzz.yml`, targets under `fuzz/`): four cargo-fuzz targets, each seeded
  with the repository's own programs by `fuzz/seed.py`. `front_end`: the edition 2026
  lexer and parser take any text, and every diagnostic points inside it. `build_run`:
  any text is built, and a program that builds runs with its compiled machine program
  and its reference run in agreement. `fmt`: the formatter keeps every token and
  comment, and a formatted file formats to itself. `edition_2025`: the 2025 compiler
  and evaluator take any text without a panic. Two minutes a target on a change,
  thirty a night; a failing input is kept as an artifact.

## The prover

nonos-stark, the STARK the language proves with, comes from the STARKs repository at
the commit the manifests pin. That repository builds and tests it in its own CI, along
with the shield circuits built on it and the soundness points of
`stark_proofs/src/shield_params.rs`. Here the pin is held from the language side:
`note_commit_deployed_tests` requires the note commitment circuit to compute the
digest that repository pins as deployed, `shield_key_kat` requires the key hierarchy
vector to be the one it emits, and `vkey_tests` pins a verifier key under the pinned
build.

## Supply chain and secrets

- **security** (`security.yml`): `cargo-deny` checks advisories, the license
  allowlist in `deny.toml`, banned crates, and trusted sources; `gitleaks` scans
  the full history for committed keys or tokens. Runs on every change and weekly,
  so a newly published advisory is caught with no code change.

## Hygiene

- **hygiene** (`hygiene.yml`): the tree carries no authorship trace, host secret,
  or committed coordination note, and new commit messages read as human. This
  keeps a public repo public-safe by construction rather than by review memory.
  `scripts/check_rules.py` then holds what the change adds to the repository's
  rules: a code file of at most 75 lines, comments in Rust as blocks, no `allow`,
  only declarations in a `mod.rs`, no em-dash or banned word, and commit subjects
  of at most 72 characters. It reports what the change introduces, nothing older.

## The workflows themselves

Every workflow reads the repository and writes nothing back (`permissions:
contents: read`), every job has a time limit, and every action is pinned to a
commit, with its release tag beside it. Dependabot (`.github/dependabot.yml`)
proposes the new commit when an action releases.

## Reproducibility

`flake.nix` pins the whole toolchain, Rust and the Lean manager, through
`flake.lock`. `nix develop` gives the exact environment the gates run against, and
`nix flake check` builds the formatting check from a fixed world. A verifier whose
byte digest must reproduce is built from a pinned world or it is not reproducible.
