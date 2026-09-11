# The pipeline

Every change runs through these gates before it can land. Each is a workflow under
`.github/workflows`, and the point of writing them down here is that the guarantees
are legible in one place rather than scattered across YAML.

## Correctness

- **rust** (`rust.yml`): `cargo fmt --check` and `cargo clippy -D warnings` on the
  language crates, then `cargo test --workspace --release`. The step assembly, a
  multi-gigabyte trace, runs single-threaded so it does not collide with the rest.
- **lean** (`lean.yml`): `lake build` compiles every proof; a grep gate rejects
  `sorry` and `admit`; and the **axiom audit** runs `Zkolang/Audit.lean`, failing
  if any load-bearing theorem depends on `ofReduceBool` (which `native_decide`
  would introduce) or `sorryAx`. The trust base is checked, not claimed.

## Money-safety

The forgery suite lives inside the workspace tests: every binding in the shield
circuit carries a forgery that violates exactly it, and each must reject. The
recursion family does the same for the aggregation proof. `inventory` fails if a
binding is added without a forgery, so coverage cannot quietly regress.

## Security level

`shield_params::tests` asserts the deployment point clears a 128-bit soundness
floor and that the development point stays strictly weaker. Changing the money rate
below the line turns the suite red here, not on chain.

## Determinism

- **verify** (`verify.yml`): the prover's serialized output is held byte-for-byte
  against a frozen digest. A prover change that moves the bytes is caught, and a
  digest that is meant to move is re-baked deliberately.

## Supply chain and secrets

- **security** (`security.yml`): `cargo-deny` checks advisories, the license
  allowlist in `deny.toml`, banned crates, and trusted sources; `gitleaks` scans
  the full history for committed keys or tokens. Runs on every change and weekly,
  so a newly published advisory is caught with no code change.

## Hygiene

- **hygiene** (`hygiene.yml`): the tree carries no authorship trace, host secret,
  or committed coordination note, and new commit messages read as human. This
  keeps a public repo public-safe by construction rather than by review memory.

## Reproducibility

`flake.nix` pins the whole toolchain, Rust and the Lean manager, through
`flake.lock`. `nix develop` gives the exact environment the gates run against, and
`nix flake check` builds the formatting check from a fixed world. A verifier whose
byte digest must reproduce is built from a pinned world or it is not reproducible.

## Perf

- **measure** (`measure.yml`): prove and verify timings and proof sizes, tracked
  so a regression in cost is visible.
