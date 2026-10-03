/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Ledger.Range

/-!
Division, `Quot(a, b, n)` and `Rem(a, b, n)` for `n ≤ 32` (constraint ledger, section 3;
`gadget/divide.rs`). The advice is `q` and `r`. Each of `a`, `b`, `q`, `r` and `b − r − 1`
is range-checked below `2^n ≤ 2^32`, the last read canonically, and one row asserts
`q · b + r = a`.
-/

namespace Zkolang.Ledger.Divide
open Zkolang.Field Zkolang.Ledger

/-- The check of `b − r − 1` holds only when `r < b`, so `b` is at least one. -/
theorem remainder_below (b r : Int) (hb : 0 ≤ b ∧ b < 2 ^ 32) (hr : 0 ≤ r ∧ r < 2 ^ 32)
    (hd : (b - r - 1) % p < 2 ^ 32) : r < b := by
  by_cases h : r < b
  · exact h
  · have := wrapped_large (d := b - r - 1) (by omega) (by omega)
    omega

/-- Soundness: `q` and `r` are the quotient and the remainder of `a` by `b`. With `q` and
`b` below `2^32`, `q · b + r` stays below `p`, so the field equation is an integer one, and
integer division makes `q` and `r` unique. -/
theorem divide_sound (a b q r : Int) (ha : 0 ≤ a ∧ a < 2 ^ 32) (hb : 0 ≤ b ∧ b < 2 ^ 32)
    (hq : 0 ≤ q ∧ q < 2 ^ 32) (hr : 0 ≤ r ∧ r < 2 ^ 32) (hd : (b - r - 1) % p < 2 ^ 32)
    (h : cong (q * b + r) a) : a / b = q ∧ a % b = r := by
  have hrb := remainder_below b r hb hr hd
  have hqb : q * b ≤ (2 ^ 32 - 1) * (2 ^ 32 - 1) :=
    Int.mul_le_mul (by omega) (by omega) hb.1 (by omega)
  have hqb0 : 0 ≤ q * b := Int.mul_nonneg hq.1 hb.1
  have e : q * b + r = a :=
    eq_of_cong (by omega) (by rw [p_eq]; omega) ha.1 (by rw [p_eq]; omega) h
  exact (Int.ediv_emod_unique (by omega)).mpr ⟨by rw [Int.mul_comm]; omega, hr.1, hrb⟩

end Zkolang.Ledger.Divide
