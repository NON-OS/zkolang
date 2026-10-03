/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Ledger.Range

/-!
The bounds checks of a dynamic index and of a shift (constraint ledger, section 2;
`lower/place_index.rs`, `lower/shift.rs`). For an array of length `n ≥ 1` the compiler
range-checks `(n − 1) − i` below `W = 2^bits(n − 1)`; for a shift of a `N`-bit value it
checks `(N − 1) − k` below `N`.
-/

namespace Zkolang.Ledger.Index
open Zkolang.Field Zkolang.Ledger

/-- The check is exact: for a length `1 ≤ n ≤ 2^32`, an index `i` of a `usize` and a bound
`n − 1 < W ≤ 2^32`, the canonical value of `(n − 1) − i` is below `W` exactly when
`i < n`. An index past the end makes the difference negative, and read canonically it is
above `p − 2^32`, far above `W`. -/
theorem index_exact (n i W : Int) (hn : 1 ≤ n ∧ n ≤ 2 ^ 32) (hi : 0 ≤ i ∧ i < 2 ^ 32)
    (hW : n - 1 < W ∧ W ≤ 2 ^ 32) : (n - 1 - i) % p < W ↔ i < n := by
  constructor
  · intro h
    by_cases h1 : i < n
    · exact h1
    · have := wrapped_large (d := n - 1 - i) (by omega) (by omega)
      omega
  · intro h
    rw [canon_self (by omega) (by rw [p_eq]; omega)]
    omega

/-- A shift of an `N`-bit value, `N` a power of two up to `2^32`, by an amount `k` below
`2^32`: `(N − 1) − k` is below `N` exactly when `k < N`. -/
theorem shift_exact (N k : Int) (hN : 1 ≤ N ∧ N ≤ 2 ^ 32) (hk : 0 ≤ k ∧ k < 2 ^ 32) :
    (N - 1 - k) % p < N ↔ k < N :=
  index_exact N k N hN hk ⟨by omega, hN.2⟩

end Zkolang.Ledger.Index
