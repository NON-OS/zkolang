/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Field

/-!
What the ledger's arguments share (`docs/audit/constraints.md`): the prime as a numeral,
that two canonical values equal in the field are the same integer, that a small negative
difference read canonically is large, and the powers of two the bounds are stated in.
-/

namespace Zkolang.Ledger
open Zkolang.Field

/-- The Goldilocks prime as a numeral, so `omega` can reason with it. -/
theorem p_eq : p = 18446744069414584321 := by decide

/-- Two values in `[0, p)` that are equal in the field are the same integer. -/
theorem eq_of_cong {a b : Int} (ha : 0 ≤ a) (ha' : a < p) (hb : 0 ≤ b) (hb' : b < p)
    (h : cong a b) : a = b := by
  unfold cong at h; rw [p_eq] at *; omega

/-- A value in `[0, p)` is its own canonical representative. -/
theorem canon_self {d : Int} (h : 0 ≤ d) (h' : d < p) : d % p = d := by
  rw [p_eq] at *; omega

/-- A difference at most `2^32` below zero is, read canonically, above `2^32`: a range
check of 32 bits or fewer refuses it. -/
theorem wrapped_large {d : Int} (h : -(2 ^ 32) ≤ d) (h' : d < 0) : 2 ^ 32 < d % p := by
  rw [p_eq]; omega

/-- Every power of two is positive. -/
theorem two_pow_pos : ∀ n : Nat, (0 : Int) < 2 ^ n
  | 0 => by decide
  | n + 1 => by have := two_pow_pos n; rw [Int.pow_succ]; omega

/-- Powers of two grow with the exponent. -/
theorem two_pow_le : ∀ n k : Nat, (2 : Int) ^ n ≤ 2 ^ (n + k)
  | _, 0 => Int.le_refl _
  | n, k + 1 => by
    have := two_pow_le n k
    have := two_pow_pos (n + k)
    rw [← Nat.add_assoc, Int.pow_succ]
    omega

/-- Below sixty-four bits, a power of two is at most `2^63`, far below `p`. -/
theorem two_pow_small {n : Nat} (h : n < 64) : (2 : Int) ^ n ≤ 2 ^ 63 := by
  have e : n + (63 - n) = 63 := by omega
  have := two_pow_le n (63 - n)
  rw [e] at this
  exact this

end Zkolang.Ledger
