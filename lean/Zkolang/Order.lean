/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Logic
import Zkolang.Compare

/-!
The ordering gadgets, built by selecting on the comparison. `min` and `max` are a branchless
select over `a < b`, so their soundness is the composition of the comparison soundness and
the multiplexer: on operands the range proofs pin to sixteen bits, `min` returns the smaller
value and `max` the larger. This is two proven gadgets composing without either losing its
guarantee.
-/

namespace Zkolang.Order

open Zkolang.Logic Zkolang.Compare

/-- `min(a, b)`: select `a` when `a < b`, otherwise `b`. -/
def min' (a b : Int) : Int := MUX (lt a b) a b

/-- `max(a, b)`: select `b` when `a < b`, otherwise `a`. -/
def max' (a b : Int) : Int := MUX (lt a b) b a

/-- `min` selects the operand the order names. -/
theorem min_eq (a b : Int) (ha : 0 ≤ a) (ha' : a < 65536) (hb : 0 ≤ b) (hb' : b < 65536) :
    min' a b = if a < b then a else b := by
  unfold min'
  rw [lt_sound a b ha ha' hb hb']
  by_cases h : a < b
  · simp only [if_pos h]; exact mux_one a b
  · simp only [if_neg h]; exact mux_zero a b

/-- `max` selects the other way. -/
theorem max_eq (a b : Int) (ha : 0 ≤ a) (ha' : a < 65536) (hb : 0 ≤ b) (hb' : b < 65536) :
    max' a b = if a < b then b else a := by
  unfold max'
  rw [lt_sound a b ha ha' hb hb']
  by_cases h : a < b
  · simp only [if_pos h]; exact mux_one b a
  · simp only [if_neg h]; exact mux_zero b a

/-- `min` is a lower bound of both operands: it really is the smaller. -/
theorem min_le (a b : Int) (ha : 0 ≤ a) (ha' : a < 65536) (hb : 0 ≤ b) (hb' : b < 65536) :
    min' a b ≤ a ∧ min' a b ≤ b := by
  rw [min_eq a b ha ha' hb hb']
  by_cases h : a < b
  · simp only [if_pos h]; omega
  · simp only [if_neg h]; omega

/-- `max` is an upper bound of both operands: it really is the larger. -/
theorem max_ge (a b : Int) (ha : 0 ≤ a) (ha' : a < 65536) (hb : 0 ≤ b) (hb' : b < 65536) :
    a ≤ max' a b ∧ b ≤ max' a b := by
  rw [max_eq a b ha ha' hb hb']
  by_cases h : a < b
  · simp only [if_pos h]; omega
  · simp only [if_neg h]; omega

/-- `min` returns one of its operands, never a third value. -/
theorem min_mem (a b : Int) (ha : 0 ≤ a) (ha' : a < 65536) (hb : 0 ≤ b) (hb' : b < 65536) :
    min' a b = a ∨ min' a b = b := by
  rw [min_eq a b ha ha' hb hb']
  by_cases h : a < b
  · left; simp only [if_pos h]
  · right; simp only [if_neg h]

end Zkolang.Order
