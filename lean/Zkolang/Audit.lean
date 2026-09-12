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
a placeholder proof would add the placeholder axiom. Neither may appear.

The CI axiom gate runs this file and fails if the output names `ofReduceBool` or
the placeholder axiom, so the "no native_decide, no placeholder" claim in
`VERIFICATION.md` is enforced by the build rather than by discipline. The theorems
below are the load-bearing ones: the two money guarantees, the index-bit pin, the
proof-system soundness core, the primality certificate that has to be axiom-free by
hand, and the zero-knowledge blinding identities, that blinding is invisible on the
trace domain so it costs no soundness, and that off the domain the value moves by
exactly the secret times the vanishing polynomial, where the hiding lives.
-/

#print axioms Zkolang.Nullifier.no_double_spend
#print axioms Zkolang.Transfer.no_inflation_in_field
#print axioms Zkolang.IndexBit.bottom_bit_pinned
#print axioms Zkolang.IndexBit.recovers_the_position
#print axioms Zkolang.GrandProduct.boundary_forces_prod_one
#print axioms Zkolang.Quotient.boundary_quotient_sound
#print axioms Zkolang.Fold.decompose
#print axioms Zkolang.SboxSplit.split_sound
#print axioms Zkolang.Pratt.witness_full
#print axioms Zkolang.Blinding.invisible_on_domain
#print axioms Zkolang.Blinding.shift_off_domain
