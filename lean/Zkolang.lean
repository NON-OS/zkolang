/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Logic
import Zkolang.Bits
import Zkolang.Transfer

/-!
The soundness of the zKølang standard library gadgets, in Lean 4 over the core library
alone. The boolean gadgets compute their truth tables and stay in the boolean domain,
a byte range proof binds a value to the byte range, and the transfer balance constraint
is exactly value conservation.
-/
