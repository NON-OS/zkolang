/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The equality-family gadgets. Equality is a field primitive returning a bit; here it is
modelled as the indicator of equality, and the derived tests are its combinations. Soundness
is that each test is one exactly on the condition it names and stays a bit, and that the
product test detects a nonzero pair exactly because the domain has no zero divisors, the
property `both_nonzero` relies on. The domain fact is proven for the integers, which the
field the compiler targets shares.
-/

namespace Zkolang.Cmp

def eq (a b : Int) : Int := if a = b then 1 else 0
def isZero (x : Int) : Int := eq x 0
def isNonzero (x : Int) : Int := 1 - isZero x
def isDistinct (a b : Int) : Int := 1 - eq a b
def bothNonzero (a b : Int) : Int := isNonzero (a * b)

/-- The equality test is a bit. -/
theorem eq_isBit (a b : Int) : eq a b = 0 ∨ eq a b = 1 := by
  unfold eq; by_cases h : a = b
  · rw [if_pos h]; right; rfl
  · rw [if_neg h]; left; rfl

/-- The equality test is one exactly when its operands are equal. -/
theorem eq_iff (a b : Int) : eq a b = 1 ↔ a = b := by
  unfold eq; by_cases h : a = b <;> simp [h]

/-- The zero test is one exactly at zero. -/
theorem isZero_iff (x : Int) : isZero x = 1 ↔ x = 0 := by
  unfold isZero; exact eq_iff x 0

/-- The distinctness test is one exactly when the operands differ. -/
theorem isDistinct_iff (a b : Int) : isDistinct a b = 1 ↔ a ≠ b := by
  unfold isDistinct eq; by_cases h : a = b <;> simp [h]

/-- The nonzero test is one exactly away from zero. -/
theorem isNonzero_iff (x : Int) : isNonzero x = 1 ↔ x ≠ 0 := by
  unfold isNonzero isZero eq; by_cases h : x = 0 <;> simp [h]

/-- The pair test is one exactly when neither operand is zero. This is where the domain
matters: the product is zero exactly when a factor is, so the single product test decides
both, which is why the gadget is sound over a field with no zero divisors. -/
theorem bothNonzero_iff (a b : Int) : bothNonzero a b = 1 ↔ a ≠ 0 ∧ b ≠ 0 := by
  unfold bothNonzero
  rw [isNonzero_iff]
  constructor
  · intro h
    exact ⟨fun ha => h (by rw [ha, Int.zero_mul]), fun hb => h (by rw [hb, Int.mul_zero])⟩
  · rintro ⟨ha, hb⟩ h
    rcases Int.mul_eq_zero.mp h with h0 | h0
    · exact ha h0
    · exact hb h0

end Zkolang.Cmp
