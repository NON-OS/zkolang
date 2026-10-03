# Security

## Reporting a vulnerability

Report privately to ekisanon@proton.me. Please do not open a public issue for a
suspected soundness, privacy, or key-handling flaw. A working proof-of-concept, or
the exact test that should reject and does not, is the most useful thing you can
send. You will get an acknowledgement, and a fix or an explanation of why the
behaviour is intended.

## What is proven, machine-checked

These are Lean 4 theorems over the core library, no Mathlib, no `sorry`, and no
`native_decide` (the axiom audit in CI enforces the last two). Each rests only on
`propext`, `Classical.choice`, and `Quot.sound`; the Goldilocks primality
certificate rests on no axioms at all.

- **No double-spend.** A note yields one nullifier; a replay finds it already
  present and is refused (`Nullifier.no_double_spend`). The bottom index bit, the
  one a scalar could disagree with, is pinned to the authenticated position
  (`IndexBit.bottom_bit_pinned`).
- **No inflation.** Value is conserved as an integer, not merely mod p, so no
  amount is minted by field wraparound (`Transfer.no_inflation_in_field`).
- **Proof-system soundness core.** The grand-product accumulator, the
  boundary-quotient check, the FRI fold decomposition, and the s-box split are
  each proven sound.
- **The compiler's constraint ledger.** The guards, bit decomposition, field
  bits, division, and the bounds checks of an index and a shift
  (`lean/Zkolang/Ledger`). `lean/Axioms.lean` lists every theorem with the axioms
  it rests on.

## What is enforced by the test and CI gates

- **The compiled program and the reference run agree.** A run of an edition 2026
  program runs the compiled machine program beside the reference interpreter of
  the typed program, and a disagreement is reported as a compiler bug instead of
  being proven. The fuzz target `build_run` searches for one on every change.
- **The constraint ledger.** `docs/audit/constraints.md` lists every constraint
  the edition 2026 compiler writes, its soundness argument, checked in Lean under
  `lean/Zkolang/Ledger`, and a test that rejects a run breaking it. Over the
  compiled programs of the suite, every advice value is checked to be pinned.
- **The deployed note commitment.** `note_commit_deployed_tests` holds the note
  commitment written in the language equal to the digest the STARKs repository
  pins as deployed, and `shield_key_kat` holds the key hierarchy vector byte for
  byte to the one that repository emits.
- **Fuzzing.** Four cargo-fuzz targets, the front end, building and running, the
  formatter and the edition 2025 compiler, run on every change and nightly.
- **Supply chain and secrets.** `cargo-deny` (advisories, licenses, bans,
  sources) and a full-history secret scan run on every change and weekly.

The prover, `nonos-stark`, and the shield circuits built on it are tested in the
STARKs repository, which owns that code; this repository pins the commit it
builds against.

## Privacy, stated precisely

The commitment scheme hides amounts, the sender-receiver link, and the note graph:
notes are blinded before they are committed, and a nullifier reveals no note, so
the ledger discloses none of it. This is the privacy that is there from day one,
and it requires the wallet to draw fresh random blinding for every note.

The proof is **not** claimed to be fully zero-knowledge. An edition 2026 proof
made by `zkolang run` blinds every trace column with a random polynomial expanded
from a seed read fresh from the system, one random coefficient for each point the
proof opens the column at, so the trace values the proof reveals are jointly
uniform (`nonos_zkolang/src/driver/prover.rs`; the identities the blinding rests
on are `Blinding.invisible_on_domain` and `Blinding.shift_off_domain` in Lean).
A zero-knowledge argument that also covers the composition polynomial and the FRI
layers is not made here. Edition 2025 proofs from `zkolang run` are not blinded:
there a `secret` input is a private witness the statement leaves out, and the
query openings can show trace values. `nonos_zkolang/docs/07-reference.md` states
the same boundary.

## Parameters and deployment

A zKølang program is proven at the point of `nonos_zkolang/src/driver/params.rs`:
32 queries, a 16-bit grind and rate 1/16. The STARKs repository states the same
point as `inner` in `stark_proofs/src/shield_params.rs`, at 144 conjectured and
80 provable bits, with the argument in its `docs/12-soundness.md`; the points the
shield settles at are stated there too. A change to these numbers is a reviewed
change.
