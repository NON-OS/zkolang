/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

import Zkolang.Field

/-!
The grand-product accumulator's soundness: the running product an AIR carries computes
exactly the product of its per-row ratios, so its start-and-end boundary `z_0 = z_n = 1` is
exactly the statement that the whole product is one. This is the mechanism the permutation
argument rests on, and it is the outer's degree ceiling and its double-spend line: the same
accumulator binds the nullifier columns, so its soundness is what makes a spent note
unspendable twice.

Proven here is the telescoping: given the per-row constraint `z_{i+1} = z_i * r_i`, the
accumulator equals `z_0` times the product of the ratios, by induction on the rows, with no
hypothesis on the ratios at all. The boundary then reads off directly: an accumulator that
starts and ends at one forces the product of its ratios to be one. What remains, named not
assumed, is the Schwartz-Zippel bridge from `Field`'s challenges: a product of one over the
random `beta, gamma` forces the multiset of wired cells to match except with probability
bounded by the degree over the field size. That probabilistic step is the classical
argument; the algebraic identity it multiplies against is proven below. Over the integers,
read into Goldilocks by `transfer`.
-/

namespace Zkolang.GrandProduct

open Zkolang.Field

/-- The product of a list of ratios, folded from one. -/
def prod : List Int → Int
  | [] => 1
  | r :: rs => r * prod rs

/-- The accumulator's final value: from `z0`, each row multiplies in its ratio. The direct
recursion the AIR's row constraint `z_{i+1} = z_i * r_i` composes to. -/
def final (z0 : Int) : List Int → Int
  | [] => z0
  | r :: rs => final (z0 * r) rs

/-- Telescoping: the accumulator's final value is the start times the product of every
ratio, with no condition on the ratios. This is the whole content of "the running product
computes the product". -/
theorem final_eq_prod (z0 : Int) (rs : List Int) : final z0 rs = z0 * prod rs := by
  induction rs generalizing z0 with
  | nil => simp [final, prod]
  | cons r rs ih =>
    rw [final, prod, ih (z0 * r)]
    exact Int.mul_assoc z0 r (prod rs)

/-- Soundness of the boundary: an accumulator that starts at one and ends at one forces the
product of its ratios to be one. This is exactly the AIR's `z_0 = 1` boundary together with
its `z_n = 1` boundary, and it is the algebraic core the permutation check enforces. -/
theorem boundary_forces_prod_one (rs : List Int) (h : final 1 rs = 1) : prod rs = 1 := by
  have := final_eq_prod 1 rs
  rw [h] at this
  omega

/-- The same conclusion read into the field: the ratio product is one in Goldilocks whenever
the accumulator's boundary holds. -/
theorem boundary_in_field (rs : List Int) (h : final 1 rs = 1) : cong (prod rs) 1 :=
  transfer (boundary_forces_prod_one rs h)

/-- A single misbinding cannot hide: if one ratio is replaced so the product changes, the
boundary can no longer close. Contrapositive of the soundness, the form the forgery argument
uses: a tampered wiring whose product is not one is rejected by the boundary. -/
theorem tamper_breaks_boundary (rs : List Int) (h : prod rs ≠ 1) : final 1 rs ≠ 1 := by
  intro hf
  exact h (boundary_forces_prod_one rs hf)

end Zkolang.GrandProduct
