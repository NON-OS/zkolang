/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Logic
import Zkolang.Bits
import Zkolang.Cmp
import Zkolang.Compare
import Zkolang.Order
import Zkolang.Select
import Zkolang.Encode
import Zkolang.Gate
import Zkolang.Math
import Zkolang.Poly
import Zkolang.Hash
import Zkolang.Field
import Zkolang.Transfer

/-!
The soundness of the zKølang standard library gadgets, in Lean 4 over the core library
alone. The boolean gadgets compute their truth tables and stay in the boolean domain,
a byte range proof binds a value to the byte range, the equality tests are one exactly on
the condition they name with the pair test sound because the domain has no zero divisors,
the ordered comparison outputs one exactly when its operands are in order, the ordering gadgets return the true minimum and
maximum by composing the comparison with the multiplexer, the wider multiplexers select the
line their bits address, the encoding gadgets pack a tuple losslessly and injectively while
each part is in range, the arithmetic gates add their input bits exactly, the power gadgets
compute the power they name, the polynomial gadgets evaluate their Horner form to the
polynomial they name, the MiMC S-box exponent is coprime to the field's multiplicative order
so the round map is a permutation, and the transfer balance constraint is exactly value
conservation. A field module defines Goldilocks as the integers modulo the prime and proves
the transfer principle under which every one of these integer identities is a field
identity; the multiplicative inverse, which needs the prime's primality and Fermat's
theorem, is noted as beyond the core library rather than assumed.
-/
