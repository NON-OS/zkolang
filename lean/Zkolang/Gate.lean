/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Logic

/-!
The arithmetic gate gadgets, the atoms of a binary circuit. A half adder sums two bits into a
sum and a carry; a full adder folds in an incoming carry. Soundness is that the carry and sum
are the binary addition of the inputs: `2 * carry + sum` equals the integer sum of the bits,
and both outputs are themselves bits, so the gates chain into a ripple adder. The gadgets are
the boolean gadgets of `Logic`, so this is those gates read as arithmetic.
-/

namespace Zkolang.Gate

open Zkolang.Logic

def halfSum (a b : Int) : Int := XOR a b
def halfCarry (a b : Int) : Int := AND a b
def fullSum (a b c : Int) : Int := XOR (XOR a b) c
def fullCarry (a b c : Int) : Int := MAJ a b c

/-- The half adder is correct: carry and sum are the two-bit binary sum of the input bits. -/
theorem half_adder (a b : Int) (ha : a = 0 ∨ a = 1) (hb : b = 0 ∨ b = 1) :
    2 * halfCarry a b + halfSum a b = a + b := by
  rcases ha with h | h <;> rcases hb with h' | h' <;> subst_vars <;> decide

/-- The full adder is correct: carry and sum are the two-bit binary sum of the three bits. -/
theorem full_adder (a b c : Int)
    (ha : a = 0 ∨ a = 1) (hb : b = 0 ∨ b = 1) (hc : c = 0 ∨ c = 1) :
    2 * fullCarry a b c + fullSum a b c = a + b + c := by
  rcases ha with h | h <;> rcases hb with h' | h' <;> rcases hc with h'' | h'' <;>
    subst_vars <;> decide

/-- The half adder's outputs are bits, so it chains. -/
theorem half_outputs_bits (a b : Int) (ha : a = 0 ∨ a = 1) (hb : b = 0 ∨ b = 1) :
    (halfSum a b = 0 ∨ halfSum a b = 1) ∧ (halfCarry a b = 0 ∨ halfCarry a b = 1) := by
  rcases ha with h | h <;> rcases hb with h' | h' <;> subst_vars <;> decide

/-- The full adder's outputs are bits, so a ripple carry stays in the boolean domain. -/
theorem full_outputs_bits (a b c : Int)
    (ha : a = 0 ∨ a = 1) (hb : b = 0 ∨ b = 1) (hc : c = 0 ∨ c = 1) :
    (fullSum a b c = 0 ∨ fullSum a b c = 1) ∧ (fullCarry a b c = 0 ∨ fullCarry a b c = 1) := by
  rcases ha with h | h <;> rcases hb with h' | h' <;> rcases hc with h'' | h'' <;>
    subst_vars <;> decide

end Zkolang.Gate
