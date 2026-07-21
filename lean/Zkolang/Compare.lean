/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
Ordered comparison soundness. The `<` gadget compares two operands the compiler first range
proves to sixteen bits. It forms the shifted difference `t = a + 2^16 - b`, decomposes it
into seventeen bits, and takes the top bit, which is one exactly when `a ≥ b`; the result
for `a < b` is its complement. Because `t` lies below `2^17`, that top bit is the high part
`t / 2^16`. The theorem is that, on operands the range proofs pin to `[0, 2^16)`, the gadget
outputs one exactly when `a < b`. The range hypotheses are essential: an operand outside the
range could otherwise forge an order, which is why the gadget range proves both operands
first. Here `2^16` is written as the literal `65536`.
-/

namespace Zkolang.Compare

/-- The `2^16` offset the gadget adds so the difference stays non-negative. -/
def offset : Int := 65536

/-- The shifted difference the gadget decomposes, `a + 2^16 - b`. -/
def diff (a b : Int) : Int := a + offset - b

/-- The gadget's result for `a < b`: one minus the top bit of the shifted difference. The
top bit is the high part `t / 2^16`, since `t` is a seventeen-bit value. -/
def lt (a b : Int) : Int := 1 - diff a b / offset

/-- Soundness: on operands the range proofs constrain to sixteen bits, the comparison
gadget outputs one exactly when `a < b` and zero otherwise. -/
theorem lt_sound (a b : Int)
    (ha : 0 ≤ a) (ha' : a < 65536) (hb : 0 ≤ b) (hb' : b < 65536) :
    lt a b = if a < b then 1 else 0 := by
  unfold lt diff offset
  by_cases h : a < b
  · rw [if_pos h]; omega
  · rw [if_neg h]; omega

/-- The gadget's output is itself a bit, zero or one on any in-range operands, so a
comparison composes with the boolean gadgets. -/
theorem lt_isBit (a b : Int)
    (ha : 0 ≤ a) (ha' : a < 65536) (hb : 0 ≤ b) (hb' : b < 65536) :
    lt a b = 0 ∨ lt a b = 1 := by
  unfold lt diff offset
  omega

/-- The complement view: the top bit alone decides `a ≥ b`, which is what the gadget reads
before taking the complement. -/
theorem topbit_ge (a b : Int)
    (ha : 0 ≤ a) (ha' : a < 65536) (hb : 0 ≤ b) (hb' : b < 65536) :
    diff a b / offset = if b ≤ a then 1 else 0 := by
  unfold diff offset
  by_cases h : b ≤ a
  · rw [if_pos h]; omega
  · rw [if_neg h]; omega

end Zkolang.Compare
