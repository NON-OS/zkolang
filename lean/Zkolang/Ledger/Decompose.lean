/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Ledger.Range

/-!
Bit decomposition, the gadget behind `Bit(v, k, n)` and `RangeCheck(v, n)` for `n < 64`
(constraint ledger, section 3; `gadget/decompose.rs`). The advice is `n` bits, each held to
`0` or `1` by an `AssertBool` row, and one row asserts that their weighted sum equals `v`.
-/

namespace Zkolang.Ledger.Decompose
open Zkolang.Field Zkolang.Ledger

/-- The value of little-endian bits: `b₀ + 2·b₁ + 4·b₂ + …`. -/
def val : List Int → Int
  | [] => 0
  | b :: bs => b + 2 * val bs

/-- Every entry is a bit, as the `AssertBool` rows require. -/
def bits : List Int → Prop
  | [] => True
  | b :: bs => (b = 0 ∨ b = 1) ∧ bits bs

/-- `n` bits are worth at least zero and less than `2^n`. -/
theorem val_range : ∀ bs : List Int, bits bs → 0 ≤ val bs ∧ val bs < 2 ^ bs.length
  | [], _ => by simp [val]
  | b :: bs, ⟨hb, h⟩ => by
    have ih := val_range bs h
    simp only [val, List.length_cons, Int.pow_succ]
    omega

/-- Two bit strings of one length and one value are the same bits. -/
theorem val_inj : ∀ bs cs : List Int, bits bs → bits cs → bs.length = cs.length →
    val bs = val cs → bs = cs
  | [], [], _, _, _, _ => rfl
  | [], _ :: _, _, _, hl, _ => by simp at hl
  | _ :: _, [], _, _, hl, _ => by simp at hl
  | b :: bs, c :: cs, ⟨hb, h⟩, ⟨hc, h'⟩, hl, hv => by
    simp only [val] at hv
    have hbc : b = c := by omega
    subst hbc
    rw [val_inj bs cs h h' (by simpa using hl) (by omega)]

/-- Soundness: when `n < 64` bits equal `v` in the field and `v` is canonical, their sum
is `v` as an integer, so `v` is below `2^n`. The sum is below `2^63 < p`, so equality in
the field is equality of integers. -/
theorem decompose_sound (bs : List Int) (v : Int) (hb : bits bs) (hn : bs.length < 64)
    (hv : 0 ≤ v) (hv' : v < p) (h : cong (val bs) v) : val bs = v ∧ v < 2 ^ bs.length := by
  have hr := val_range bs hb
  have hs := two_pow_small hn
  have e := eq_of_cong hr.1 (by rw [p_eq]; omega) hv hv' h
  exact ⟨e, e ▸ hr.2⟩

/-- Uniqueness: any two decompositions of one canonical `v` into `n < 64` bits are the
same bits, so the advice the gadget reads is pinned. -/
theorem decompose_unique (bs cs : List Int) (v : Int) (hb : bits bs) (hc : bits cs)
    (hl : bs.length = cs.length) (hn : bs.length < 64) (hv : 0 ≤ v) (hv' : v < p)
    (h : cong (val bs) v) (h' : cong (val cs) v) : bs = cs := by
  have e := (decompose_sound bs v hb hn hv hv' h).1
  have e' := (decompose_sound cs v hc (hl ▸ hn) hv hv' h').1
  exact val_inj bs cs hb hc hl (by rw [e, e'])

end Zkolang.Ledger.Decompose
