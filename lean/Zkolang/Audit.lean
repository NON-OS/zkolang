/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

import Zkolang

/-!
The axiom audit. `#print axioms` reports, for each theorem, the axioms its proof
actually depends on. A clean proof over the core library rests only on `propext`,
`Classical.choice`, and `Quot.sound`; anything else is a leak in the trust base.
`native_decide` would add `Lean.ofReduceBool`, trusting the compiler's evaluator;
a `sorry` would add `sorryAx`. Neither may appear.

The CI axiom gate runs this file and fails if the output names `ofReduceBool` or
`sorryAx`, so the "no native_decide, no sorry" claim in `VERIFICATION.md` is
enforced by the build rather than by discipline. The theorems below are the
load-bearing ones: the two money guarantees, the index-bit pin, the proof-system
soundness core, and the primality certificate that has to be axiom-free by hand.
-/

#print axioms Zkolang.Nullifier.no_double_spend
#print axioms Zkolang.Transfer.no_inflation_in_field
#print axioms Zkolang.IndexBit.bottom_bit_pinned
#print axioms Zkolang.GrandProduct.boundary_forces_prod_one
#print axioms Zkolang.Quotient.boundary_quotient_sound
#print axioms Zkolang.Fold.decompose
#print axioms Zkolang.SboxSplit.split_sound
#print axioms Zkolang.Pratt.witness_full
