/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

import Zkolang.Field

/-!
The balance constraint of a shielded transfer, proven to hold exactly when value is
conserved. The circuit constrains the balance expression to zero, and this shows that
is equivalent to inputs equalling outputs plus fee, so an accepted proof is a proof of
conservation, no amount revealed.

The subtlety a field system must not miss: the circuit's equality is congruence mod `p`,
not integer equality, so a prover could in principle satisfy the balance while the real
amounts differ by a multiple of `p`, minting `p` units from nothing. That vector is closed
only by the range checks. The final theorem here makes the dependence explicit: when both
sides of the balance are held below `p` by the amount range proofs, field conservation is
integer conservation, and the inflation-by-wraparound vector is gone. Range checks and the
balance constraint are load-bearing together, neither alone.
-/

namespace Zkolang.Transfer

open Zkolang.Field

/-- The balance expression the transfer circuit constrains to zero. -/
def balance (in0 in1 out0 out1 fee : Int) : Int := (in0 + in1) - (out0 + out1 + fee)

/-- The constraint is satisfied if and only if value is conserved. -/
theorem transfer_sound (in0 in1 out0 out1 fee : Int) :
    balance in0 in1 out0 out1 fee = 0 ↔ in0 + in1 = out0 + out1 + fee := by
  unfold balance; constructor <;> intro h <;> omega

/-- A fee bearing transfer that conserves value leaves the outputs and fee no greater
than the inputs, so no value is created. -/
theorem transfer_no_inflation (in0 in1 out0 out1 fee : Int)
    (h : balance in0 in1 out0 out1 fee = 0) :
    out0 + out1 + fee = in0 + in1 := by
  unfold balance at h; omega

/-- The wraparound vector, closed. The circuit checks the balance as a field congruence,
so field conservation alone permits the amounts to differ by a multiple of `p`. When the
range proofs hold both sides below `p` (and non-negative), the canonical representative is
each side itself, so the congruence forces integer equality: no value is minted through the
modulus. This is exactly why the range checks and the balance constraint are one guarantee,
not two. -/
theorem no_inflation_in_field (lhs rhs : Int)
    (hlo : 0 ≤ lhs) (hhi : lhs < p) (hlo2 : 0 ≤ rhs) (hhi2 : rhs < p)
    (hcong : cong lhs rhs) : lhs = rhs := by
  unfold cong at hcong
  rw [Int.emod_eq_of_lt hlo hhi, Int.emod_eq_of_lt hlo2 hhi2] at hcong
  exact hcong

end Zkolang.Transfer
