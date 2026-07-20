/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The bit recomposition of the standard library and the soundness of a range proof. A
value constrained to equal the weighted sum of eight bits can only be a byte, a number
in the range zero through two hundred fifty five. This is what keeps a shielded amount
non negative and bounded: an out of range value has no bit witness.
-/

namespace Zkolang.Bits

def compose2 (b0 b1 : Int) : Int := b0 + 2 * b1
def compose4 (b0 b1 b2 b3 : Int) : Int := b0 + 2 * b1 + 4 * b2 + 8 * b3
def compose8 (b0 b1 b2 b3 b4 b5 b6 b7 : Int) : Int :=
  b0 + 2 * b1 + 4 * b2 + 8 * b3 + 16 * b4 + 32 * b5 + 64 * b6 + 128 * b7

/-- A byte range proof is sound: if each summand is a bit, the composed value lies in
the byte range. The hypotheses are the bit bounds the `assert bit(x)` constraints
establish. -/
theorem compose8_range
    (b0 b1 b2 b3 b4 b5 b6 b7 : Int)
    (h0 : 0 ≤ b0) (h0' : b0 ≤ 1) (h1 : 0 ≤ b1) (h1' : b1 ≤ 1)
    (h2 : 0 ≤ b2) (h2' : b2 ≤ 1) (h3 : 0 ≤ b3) (h3' : b3 ≤ 1)
    (h4 : 0 ≤ b4) (h4' : b4 ≤ 1) (h5 : 0 ≤ b5) (h5' : b5 ≤ 1)
    (h6 : 0 ≤ b6) (h6' : b6 ≤ 1) (h7 : 0 ≤ b7) (h7' : b7 ≤ 1) :
    0 ≤ compose8 b0 b1 b2 b3 b4 b5 b6 b7 ∧
      compose8 b0 b1 b2 b3 b4 b5 b6 b7 ≤ 255 := by
  unfold compose8; omega

/-- Two nibble decompositions of the same value are equal: the bits are determined by
the value, so a range proof binds a unique witness. -/
theorem compose4_injective
    (a0 a1 a2 a3 c0 c1 c2 c3 : Int)
    (ha0 : 0 ≤ a0) (ha0' : a0 ≤ 1) (hc0 : 0 ≤ c0) (hc0' : c0 ≤ 1)
    (ha1 : 0 ≤ a1) (ha1' : a1 ≤ 1) (hc1 : 0 ≤ c1) (hc1' : c1 ≤ 1)
    (ha2 : 0 ≤ a2) (ha2' : a2 ≤ 1) (hc2 : 0 ≤ c2) (hc2' : c2 ≤ 1)
    (_ha3 : 0 ≤ a3) (_ha3' : a3 ≤ 1) (_hc3 : 0 ≤ c3) (_hc3' : c3 ≤ 1)
    (heq : compose4 a0 a1 a2 a3 = compose4 c0 c1 c2 c3) :
    a0 = c0 ∧ a1 = c1 ∧ a2 = c2 ∧ a3 = c3 := by
  unfold compose4 at heq; omega

end Zkolang.Bits
