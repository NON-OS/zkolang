/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
Zero-knowledge blinding. A hiding STARK replaces each trace column `f` with
`f + r * Z_H`, where `Z_H = x^t - 1` vanishes on the trace domain and `r` is the
prover's secret randomness. Two facts make this a zero-knowledge construction that
costs no soundness, and both are ring identities, proven here over the core library
so they hold in the field the prover computes in.

The first is completeness: on the trace domain, where `x^t = 1`, the vanishing
polynomial is zero, so the blinded column takes the same value as `f` at every row.
No constraint sees the blinding, and a satisfying trace stays satisfying, so a
blinded proof still verifies. The second is the hiding: off the domain, where the
FRI queries open, the value moves by exactly `r * (x^t - 1)`, a term the witness
does not determine, which is what randomizes the openings.

Field elements are the integers here; `xt` stands for the evaluated power `x^t`, so
`xt = 1` is the hypothesis that `x` is a `t`-th root of unity, one row of the trace
domain. `Zkolang.Field` carries the transfer principle under which these integer
identities are the field identities the prover relies on.
-/

namespace Zkolang.Blinding

/-- The vanishing polynomial of the trace domain, evaluated: `Z_H(x) = x^t - 1`. -/
def zh (xt : Int) : Int := xt - 1

/-- A column value after blinding: `f(x) + r(x) * Z_H(x)`. `f` and `r` are the
    evaluated column and the evaluated blinding, `xt` the evaluated `x^t`. -/
def blindEval (f r xt : Int) : Int := f + r * zh xt

/-- Completeness. On the trace domain, where `x^t = 1`, the blinded column equals
    the original at that row, so blinding is invisible to every constraint and a
    satisfying trace stays satisfying. This is why blinding costs no soundness. -/
theorem invisible_on_domain (f r xt : Int) (hdom : xt = 1) :
    blindEval f r xt = f := by
  unfold blindEval zh
  have hz : xt - 1 = 0 := by omega
  rw [hz, Int.mul_zero, Int.add_zero]

/-- Hiding. Off the domain the blinded value moves by exactly `r * (x^t - 1)`. That
    shift is a function of the secret `r`, not of the witness, so it is what
    randomizes the query openings and hides the trace. -/
theorem shift_off_domain (f r xt : Int) :
    blindEval f r xt - f = r * zh xt := by
  unfold blindEval
  omega

/-- A boundary constraint is preserved on the domain: if the column meets its bound
    value there, the blinded column meets it too. A direct corollary of invisibility,
    stated because the boundary quotients are where a blinded proof could have
    silently broken. -/
theorem preserves_boundary (r xt expected : Int) (hdom : xt = 1) :
    blindEval expected r xt = expected :=
  invisible_on_domain expected r xt hdom

end Zkolang.Blinding
