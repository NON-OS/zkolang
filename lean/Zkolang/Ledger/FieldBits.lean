/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Ledger.Range

/-!
Field bits, `FieldBit(v, k)` (constraint ledger, section 3; `gadget/field_bits.rs`).
Sixty-four advice bits, each `AssertBool`, are read as `lo`, bits 0 to 31, and `hi`, bits
32 to 63. The constraints are `lo + 2^32 · hi = v` and `(hi == 2^32 − 1) · lo = 0`.
-/

namespace Zkolang.Ledger.FieldBits
open Zkolang.Field Zkolang.Ledger

/-- Soundness: the bits are those of `v`'s canonical representative. `lo + 2^32 · hi` is
below `2^64 < 2p`, so it is `v` or `v + p`; `v + p` is at least `p`, which forces `hi` to
`2^32 − 1` and `lo` above zero, and the second constraint refuses that. -/
theorem field_bits_sound (lo hi v : Int) (hlo : 0 ≤ lo ∧ lo < 2 ^ 32)
    (hhi : 0 ≤ hi ∧ hi < 2 ^ 32) (hv : 0 ≤ v ∧ v < p) (h : cong (lo + 2 ^ 32 * hi) v)
    (htop : hi = 2 ^ 32 - 1 → lo = 0) : lo + 2 ^ 32 * hi = v := by
  unfold cong at h
  rw [p_eq] at h hv
  omega

/-- Completeness: the bits of a canonical `v` keep both constraints. The largest canonical
value, `p − 1`, has `hi = 2^32 − 1` and `lo = 0`, and passes. -/
theorem field_bits_complete (v : Int) (hv : 0 ≤ v ∧ v < p) :
    (v / 2 ^ 32 = 2 ^ 32 - 1 → v % 2 ^ 32 = 0) ∧ v % 2 ^ 32 + 2 ^ 32 * (v / 2 ^ 32) = v := by
  rw [p_eq] at hv
  omega

end Zkolang.Ledger.FieldBits
