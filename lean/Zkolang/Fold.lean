/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

import Zkolang.Quotient

/-!
The FRI folding decomposition, the algebraic foundation of the low-degree test. FRI halves
a polynomial's degree each round by splitting it into even and odd coefficients and
recombining them at a challenge: `f(x) = E(x^2) + x * O(x^2)`, where `E` collects the
even-indexed coefficients and `O` the odd. The verifier reads `f(x)` and `f(-x)`, recovers
`E(x^2)` and `x * O(x^2)` from their sum and difference, and forms the next layer
`E(y) + beta * O(y)` at `y = x^2`; each round the degree bound halves, and after enough
rounds a constant remains iff the original was low degree.

This module proves the decomposition itself, the identity every round invokes, by induction
on the coefficient list over the core library. From it follow the sum-and-difference
identities the verifier actually computes: `f(x) + f(-x) = 2 * E(x^2)` is even in `x`, and
`f(x) - f(-x) = 2x * O(x^2)`, so the fold is a well-defined function of `x^2`. What is not
here, named as always, is that the round map applied `log` times collapses a low-degree
codeword to a constant with soundness set by the query count, which is the FRI soundness
theorem over `Field`'s challenges. The per-round algebra it iterates is proven below.
-/

namespace Zkolang.Fold

open Zkolang.Quotient

/- The even-indexed coefficients, and the odd-indexed, by mutual recursion down the list. -/
mutual
  def evens : List Int → List Int
    | [] => []
    | c :: cs => c :: odds cs
  def odds : List Int → List Int
    | [] => []
    | _ :: cs => evens cs
end

/-- The decomposition: a polynomial is its even part at `x^2` plus `x` times its odd part at
`x^2`. This is the identity every FRI round rests on. -/
theorem decompose (p : List Int) (x : Int) :
    eval p x = eval (evens p) (x * x) + x * eval (odds p) (x * x) := by
  induction p with
  | nil => simp [eval, evens, odds]
  | cons c cs ih =>
    simp only [eval, evens, odds]
    rw [ih, Int.mul_add, ← Int.mul_assoc]
    omega

/-- The verifier's even combination: `f(x) + f(-x)` is twice the even part at `x^2`, so it
does not depend on the sign of `x`. -/
theorem sum_even (p : List Int) (x : Int) :
    eval p x + eval p (-x) = 2 * eval (evens p) (x * x) := by
  have hx := decompose p x
  have hnx := decompose p (-x)
  rw [show (-x) * (-x) = x * x from Int.neg_mul_neg x x, Int.neg_mul] at hnx
  omega

/-- The verifier's odd combination: `f(x) - f(-x)` is `2x` times the odd part at `x^2`. -/
theorem diff_odd (p : List Int) (x : Int) :
    eval p x - eval p (-x) = 2 * x * eval (odds p) (x * x) := by
  have hx := decompose p x
  have hnx := decompose p (-x)
  rw [show (-x) * (-x) = x * x from Int.neg_mul_neg x x, Int.neg_mul] at hnx
  rw [hx, hnx, Int.mul_assoc]
  omega

end Zkolang.Fold
