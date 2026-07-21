/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The virtual machine gadgets that let a zKolang program verify a zKolang execution. A step of
the register machine selects one operation by a one-hot opcode and produces its result, the same
one-hot-times-op gating the step AIR uses. Soundness is that the gate is exact: on each opcode the
step result is that opcode's operation on the operands, and the one-hot constraint holds exactly
when the selectors name one operation. A trace of these gadgets, each pinned, is a proof that a run
followed the machine's rules, so this is the arithmetic under the language checking a run of itself.
-/

namespace Zkolang.Vm

def stepResult (isAdd isSub isMul a b : Int) : Int :=
  isAdd * (a + b) + isSub * (a - b) + isMul * (a * b)

def isOnehot3 (isAdd isSub isMul : Int) : Int :=
  isAdd + isSub + isMul - 1

/-- The add opcode gates the sum. -/
theorem step_add (a b : Int) : stepResult 1 0 0 a b = a + b := by
  unfold stepResult; simp

/-- The subtract opcode gates the difference. -/
theorem step_sub (a b : Int) : stepResult 0 1 0 a b = a - b := by
  unfold stepResult; simp

/-- The multiply opcode gates the product. -/
theorem step_mul (a b : Int) : stepResult 0 0 1 a b = a * b := by
  unfold stepResult; simp

/-- The one-hot constraint is zero exactly when the selectors sum to one, so a step names one
operation. Combined with each selector being a bit, that is one-hot. -/
theorem onehot3_iff (isAdd isSub isMul : Int) :
    isOnehot3 isAdd isSub isMul = 0 ↔ isAdd + isSub + isMul = 1 := by
  unfold isOnehot3; omega

end Zkolang.Vm
