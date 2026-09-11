/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

import Zkolang.Field

/-!
The boundary-quotient soundness the composition and DEEP checks rest on. A STARK enforces a
boundary "the trace polynomial `f` takes value `e` at the domain point `a`" by having the
prover witness a quotient polynomial `q` and checking `f(x) - e = (x - a) * q(x)`; FRI then
proves `q` is genuinely low degree. This module proves the algebraic half: whenever that
relation holds as a polynomial identity, the boundary is forced, because `(x - a)` vanishes
at `a` and evaluation carries the product through. The remaining half, that agreement of two
low-degree polynomials at one random out-of-domain point implies the identity everywhere, is
the Schwartz-Zippel bound over `Field`'s challenges, named here and not discharged: it needs
the univariate root count, the same class of bridge as Fermat and Lucas. Coefficient lists
and Horner evaluation, core library only, read into Goldilocks by `transfer`.
-/

namespace Zkolang.Quotient

open Zkolang.Field

/-- Horner evaluation of a coefficient list, low degree first:
`eval [c0, c1, c2] x = c0 + c1 x + c2 x^2`. -/
def eval : List Int → Int → Int
  | [], _ => 0
  | c :: cs, x => c + x * eval cs x

/-- Multiplying a polynomial by `x` is prepending a zero coefficient, and evaluation reads
that as a factor of `x`. The one structural fact the soundness turns on. -/
theorem eval_mulX (q : List Int) (x : Int) : eval (0 :: q) x = x * eval q x := by
  simp [eval]

/-- Boundary-quotient soundness. Given the polynomial identity the OOD check plus FRI
establish, `f(x) - e = x * q(x) - a * q(x)` for every `x` (this is `f - e = (x - a) * q`
written through `eval_mulX`), the boundary holds: `f(a) = e`. The `(x - a)` factor kills the
quotient exactly at `a`, where the two `a * q(a)` terms cancel. -/
theorem boundary_quotient_sound (f q : List Int) (a e : Int)
    (hid : ∀ x, eval f x - e = eval (0 :: q) x - a * eval q x) :
    eval f a = e := by
  have hz := hid a
  rw [eval_mulX] at hz
  omega

/-- The boundary read into the field: the constrained value equals its expected value in
Goldilocks whenever the quotient identity holds. -/
theorem boundary_in_field (f q : List Int) (a e : Int)
    (hid : ∀ x, eval f x - e = eval (0 :: q) x - a * eval q x) :
    cong (eval f a) e :=
  transfer (boundary_quotient_sound f q a e hid)

/-- A false boundary cannot be quotiented: if the claimed value is wrong, no witness `q`
satisfies the identity, so the check rejects. The contrapositive the forgery argument uses. -/
theorem false_boundary_has_no_quotient (f q : List Int) (a e : Int) (h : eval f a ≠ e) :
    ¬ (∀ x, eval f x - e = eval (0 :: q) x - a * eval q x) := by
  intro hid
  exact h (boundary_quotient_sound f q a e hid)

end Zkolang.Quotient
