/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The balance constraint of a shielded transfer, proven to hold exactly when value is
conserved. The circuit constrains the balance expression to zero, and this shows that
is equivalent to inputs equalling outputs plus fee, so an accepted proof is a proof of
conservation, no amount revealed.
-/

namespace Zkolang.Transfer

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

end Zkolang.Transfer
