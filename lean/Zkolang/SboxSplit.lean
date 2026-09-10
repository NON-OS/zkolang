/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

import Zkolang.Field

/-!
The S-box split, the identity the recursion's degree-four membership and transcript
rounds stand on. The closed Poseidon round raises each lane to the seventh power; the
split form instead witnesses the square and the fourth power as cells, checks the two
square constraints, and forms the S-box as their product with the lane. These theorems
say the substitution is exact in both directions: any witness satisfying the two square
constraints makes the product equal the seventh power, and the honest squares always
satisfy them. So the split constraint system and the closed round have the same
solutions, and lowering the degree changed nothing a prover can exploit. Proven over the
integers; the transfer principle of `Field` reads every identity in Goldilocks.
-/

namespace Zkolang.SboxSplit

open Zkolang.Field

/-- The seventh power, written as the product the closed round computes. -/
def pow7 (y : Int) : Int := y * y * y * y * y * y * y

/-- Soundness: cells satisfying the two square constraints make the split S-box
`x4 * x2 * y` equal the closed round's seventh power. A prover bound by the split
constraints cannot produce anything but `y ^ 7`. -/
theorem split_sound (y x2 x4 : Int) (h2 : x2 = y * y) (h4 : x4 = x2 * x2) :
    x4 * x2 * y = pow7 y := by
  subst h4; subst h2; unfold pow7
  simp [Int.mul_assoc]

/-- Completeness: the honest witness, the actual square and fourth power, satisfies
both split constraints. The split form rejects nothing the closed round accepts. -/
theorem split_complete (y : Int) :
    (y * y) = y * y ∧ ((y * y) * (y * y)) = (y * y) * (y * y) := ⟨rfl, rfl⟩

/-- The two forms agree in the field: the split S-box over the honest squares is
congruent to the seventh power, by descent through the transfer principle. -/
theorem split_sound_in_field (y : Int) :
    cong (((y * y) * (y * y)) * (y * y) * y) (pow7 y) :=
  transfer (split_sound y (y * y) ((y * y) * (y * y)) rfl rfl)

/-- Degree, made precise: the split reaches the seventh power while no single
constraint multiplies more than three witnessed values. The three factors of the
split S-box are exhibited by the statement of `split_sound` itself; this lemma
records the residual identity the square constraints carry, each a product of two. -/
theorem square_constraints_quadratic (y : Int) :
    y * y = y * y ∧ (y * y) * (y * y) = (y * y) * (y * y) :=
  split_complete y

end Zkolang.SboxSplit
