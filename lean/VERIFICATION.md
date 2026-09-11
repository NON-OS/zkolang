# What the Lean development proves, and what it names

The zKølang Lean development is 24 modules and 118 theorems in Lean 4 over the core library
alone: no Mathlib, no `sorry`, no `native_decide`, no declared `axiom`, no `admit`. It is
checked by `lake build` in CI. This file states exactly what is machine-checked and, as
importantly, where the honest boundaries are: the facts that need machinery beyond a
core-library development are named at their use site and never stood in for by an axiom.

## The trust base

Every theorem here reduces in the Lean kernel. `#print axioms` on the load-bearing results
returns one of two answers:

- `does not depend on any axioms` (the Goldilocks primality certificate), or
- `[propext, Quot.sound]` (and occasionally `Classical.choice`), which are the three axioms
  of Lean's own type theory, present in essentially every Lean proof.

No proof here depends on `sorryAx` (an admitted goal) or `Lean.ofReduceBool` (the axiom
`native_decide` injects). That second point is a deliberate constraint: every computational
fact, including the sixty-four-bit modular exponentiations of the primality certificate, is
discharged by kernel `decide`, so the compiler is not in the trust base.

## The security guarantees, machine-checked

The two properties a private-value system must never violate are theorems, not test suites.

- **No double-spend** (`Nullifier.no_double_spend`). Freshness is exactly the nullifier's
  absence from the recorded set; admitting a fresh nullifier preserves the duplicate-free
  invariant; a replay finds its nullifier already present and is refused. A note spends at
  most once. This is the semantic content of the in-circuit binding proved by
  `GrandProduct`.
- **No inflation** (`Transfer.no_inflation_in_field`). The balance is a congruence mod `p`,
  so field conservation alone would let the amounts differ by a multiple of `p` and mint
  value. With the range proofs holding both sides below `p`, the canonical representative is
  each side itself and the congruence forces integer equality. Range checks and the balance
  constraint are one guarantee, and the wraparound vector is closed.

## The proof-system soundness core

- **Grand-product accumulator** (`GrandProduct.final_eq_prod`, `boundary_forces_prod_one`,
  `tamper_breaks_boundary`). The running product equals the start times the product of the
  per-row ratios, so the `z = 1` boundary at both ends forces the whole product to one, and
  a tampered wiring whose product is not one cannot close the boundary. This mechanism is
  the outer circuit's degree ceiling and the nullifier binding at once.
- **Boundary quotient** (`Quotient.boundary_quotient_sound`,
  `false_boundary_has_no_quotient`). The composition and DEEP checks enforce a boundary by
  witnessing a quotient `q` with `f(x) - e = (x - a) * q(x)`; the `(x - a)` factor cancels
  at `a`, so the identity forces `f(a) = e`, and a false boundary admits no quotient.
- **FRI folding** (`Fold.decompose`, `sum_even`, `diff_odd`). Every polynomial is
  `E(x^2) + x * O(x^2)`; the verifier's sum and difference of `f(x)` and `f(-x)` recover the
  even and odd parts at `x^2`, so each fold is a well-defined function of `x^2`.
- **S-box split** (`SboxSplit.split_sound`, `split_complete`) and its bridge to the running
  Rust (`stark_proofs` KAT `the_sbox_split_matches_the_lean_model`). The witnessed squares
  reproduce the seventh power exactly; the deployed field op is checked equal to the Lean
  model on random inputs, so the two meet in CI rather than in prose.
- **Goldilocks primality** (`Pratt`). A Pratt certificate for `2^64 - 2^32 + 1`: the
  factorization of `p - 1`, the witness `7` to the full order and each maximal proper
  divisor, and the primality of every factor to its square-root bound, all decided by the
  kernel with zero axioms. Closes the gap `Field` left under `recip`, `divide`, `ratio`.

## The named boundaries

These are the facts a core-library development cannot reach. Each is stated at its use site
and relied on openly; none is an `axiom`.

- **Fermat's little theorem** and the field inverse (`Field`): the multiplicative inverse of
  every nonzero element exists because `p` is prime. `Pratt` supplies the primality; the
  inverse's existence from it is the classical step.
- **The Lucas primality criterion** (`Pratt`): a valid Pratt certificate yields a prime.
- **The square-root criterion** (`Pratt`): an odd `n` with no divisor up to `sqrt n` is
  prime.
- **Schwartz-Zippel** (`GrandProduct`, `Quotient`, `Fold`): agreement of low-degree
  polynomials at a random out-of-domain point implies the polynomial identity everywhere,
  except with probability bounded by the degree over the field size. This is the single
  recurring bridge under grand-product soundness, the boundary quotient, and the FRI
  collapse. The per-instance algebra each invokes is proven; the probabilistic step is
  named. It is the one frontier whose discharge (the univariate root count) would move the
  most from named to proven.
