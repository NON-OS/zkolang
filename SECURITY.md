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

## What is enforced by the test and CI gates

- **The forgery suite.** Every binding in the shield circuit has a forgery that
  violates exactly it while everything else stays honest, and the forgery must
  reject. Double-spend, mint, burn, cross-asset, ownership, and the recursion
  family are named required checks.
- **128-bit settlement soundness.** The deployment parameters clear a 128-bit
  floor, gated by a test so the money rate cannot silently regress. The
  development rate is deliberately weaker and labelled so.
- **Determinism.** The prover's output is held byte-for-byte against a frozen
  digest, so a prover change that moves the bytes is caught.
- **Supply chain and secrets.** `cargo-deny` (advisories, licenses, bans,
  sources) and a full-history secret scan run on every change and weekly.

## Privacy, stated precisely

The commitment scheme hides amounts, the sender-receiver link, and the note graph:
notes are blinded before they are committed, and a nullifier reveals no note, so
the ledger discloses none of it. This is the privacy that is there from day one,
and it requires the wallet to draw fresh random blinding for every note.

The proof itself is **not** full zero-knowledge. The STARK is not hiding, so a
determined verifier could learn trace values from the query openings. Hiding the
witness inside the proof is a further hardening, noted here rather than claimed.
`nonos_zkolang/docs/07-reference.md` and the paper state the same boundary.

## Parameters and deployment

The two soundness points live in `stark_proofs/src/shield_params.rs`: the
development rate (fast, for tests and gates) and the deployment rate (rate 1/16,
32 queries, 16 grind bits, ~128-bit). The registered verifier keys and the
on-chain verifier hold the deployment point. A change to these numbers is a
reviewed change and is gated by the security-level test.
