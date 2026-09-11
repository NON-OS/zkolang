/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

import Zkolang.Field

/-!
The bottom index bit, pinned. A Merkle path proves a leaf's position through its
directions, and a nullifier hashes that position as a scalar. Every direction but
the bottom one rides the trace and is bound to the scalar. The bottom direction
lives only in which half of the initial state the leaf occupies, so the scalar was
free to disagree with it in that one bit and retire the note under the sibling
position, a second nullifier for one note.

The fix witnesses the bottom direction as a bit `d` and a canonical leaf
`select d a b = (1-d)*a + d*b` chosen from the two halves, then binds `d` to the
scalar's low bit. These theorems say the constraint leaves `d` no freedom: when
the canonical leaf the fold authenticates lands on one half and the halves differ,
`d` is forced to name that half. The membership's root pins which half the real
leaf occupies, so `d`, and through it the scalar's low bit, is the authenticated
bottom direction and nothing else. Proven over the integers; the transfer
principle of `Field` carries every identity into Goldilocks.
-/

namespace Zkolang.IndexBit

open Zkolang.Field

/-- The half a bit selects: the low half at zero, the high half at one. -/
def select (d a b : Int) : Int := (1 - d) * a + d * b

/-- A bit is a root of `x(1-x)`. -/
def isBit (d : Int) : Prop := d * (1 - d) = 0

/-- A bit is zero or one, since the integers have no zero divisors. -/
theorem bit_zero_or_one (d : Int) (h : isBit d) : d = 0 ∨ d = 1 := by
  have h' : d * (1 - d) = 0 := h
  rcases Int.mul_eq_zero.mp h' with hd | hd
  · exact Or.inl hd
  · exact Or.inr (by omega)

/-- Selecting the low half at a bit gives the low half, and the high half at one. -/
theorem select_bit (d a b : Int) (h : isBit d) :
    (d = 0 ∧ select d a b = a) ∨ (d = 1 ∧ select d a b = b) := by
  rcases bit_zero_or_one d h with h0 | h1
  · exact Or.inl ⟨h0, by subst h0; simp [select]⟩
  · exact Or.inr ⟨h1, by subst h1; simp [select]⟩

/-- Soundness, low. If the selection is the low half and the halves differ, the bit
is zero: the prover cannot claim the high half while selecting the low one. -/
theorem select_pins_low (d a b : Int) (h : isBit d) (hsel : select d a b = a)
    (hne : a ≠ b) : d = 0 := by
  rcases select_bit d a b h with ⟨h0, _⟩ | ⟨_, hb⟩
  · exact h0
  · exact absurd (hsel.symm.trans hb) hne

/-- Soundness, high. If the selection is the high half and the halves differ, the
bit is one. -/
theorem select_pins_high (d a b : Int) (h : isBit d) (hsel : select d a b = b)
    (hne : a ≠ b) : d = 1 := by
  rcases select_bit d a b h with ⟨_, ha⟩ | ⟨h1, _⟩
  · exact absurd (ha.symm.trans hsel) hne
  · exact h1

/-- The two initial-state halves under a real bottom direction `r`: the leaf `L` in
the half `r` names, the sibling `S` in the other. This is the injection the
membership performs. -/
def lowHalf (r L S : Int) : Int := select r L S -- r=0: L (low), r=1: S
def highHalf (r L S : Int) : Int := select r S L -- r=0: S (high), r=1: L

/-- The whole pin. The canonical leaf the fold authenticates is `L`; the real
direction `r` placed `L` in half `r` with the sibling `S` in the other, and a real
leaf differs from its sibling. Then the witnessed bit `d` equals `r`. Binding `d`
to the recovered scalar's low bit therefore ties that bit to the authenticated
bottom direction: the position a nullifier hashes cannot differ from the one the
path proved, and one note yields one nullifier. -/
theorem bottom_bit_pinned (d r L S : Int)
    (hd : isBit d) (hr : isBit r) (hne : L ≠ S)
    (hsel : select d (lowHalf r L S) (highHalf r L S) = L) : d = r := by
  rcases bit_zero_or_one r hr with hr0 | hr1
  · -- r = 0: the leaf is the low half, the sibling the high half.
    subst hr0
    have hlo : lowHalf 0 L S = L := by simp [lowHalf, select]
    have hhi : highHalf 0 L S = S := by simp [highHalf, select]
    rw [hlo, hhi] at hsel
    exact select_pins_low d L S hd hsel hne
  · -- r = 1: the leaf is the high half, the sibling the low half.
    subst hr1
    have hlo : lowHalf 1 L S = S := by simp [lowHalf, select]
    have hhi : highHalf 1 L S = L := by simp [highHalf, select]
    rw [hlo, hhi] at hsel
    exact select_pins_high d S L hd hsel (fun h => hne h.symm)

/-- The pin descends to Goldilocks: the same equality holds in the field, so the
constraint the circuit checks carries the soundness the integers prove. -/
theorem bottom_bit_pinned_in_field (d r L S : Int)
    (hd : isBit d) (hr : isBit r) (hne : L ≠ S)
    (hsel : select d (lowHalf r L S) (highHalf r L S) = L) : cong d r :=
  transfer (bottom_bit_pinned d r L S hd hr hne hsel)

end Zkolang.IndexBit
