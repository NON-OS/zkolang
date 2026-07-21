# zKølang gadget soundness, in Lean 4

The standard library gadgets are polynomial expressions over the field. This project
proves, in Lean 4 with no dependency beyond the core library, that each boolean gadget
computes the function it names when its inputs are bits and that the outputs stay bits,
that a byte decomposition can only represent a value in the range zero through two hundred
fifty five and binds a unique witness, that the equality tests are one exactly on the
condition they name with the pair test sound because the domain has no zero divisors, that
the ordered comparison outputs one exactly
when its operands are in order once they are range proven, that the ordering gadgets
return the true minimum and maximum by composing that comparison with the multiplexer, that
the wider multiplexers select the line their bits address, that the encoding gadgets pack a
tuple losslessly and injectively while each part is in range, that the arithmetic gates add
their input bits exactly, that the power gadgets compute the power they name, that the
polynomial gadgets evaluate their Horner form to the polynomial they name, that the MiMC
S-box exponent is coprime to the field's multiplicative order so the round map is a
permutation, and that the transfer balance constraint holds exactly when value is conserved.

The gadgets are stated over the integers. Every proof is an integer polynomial identity or
a linear fact, so it carries to the Goldilocks field the compiler targets. The `Field`
module makes that descent rigorous: it defines the field as the integers modulo the prime,
gives every element a canonical representative, and proves the transfer principle that turns
an integer identity into a field identity, so each gadget theorem stands as a statement
about the field. The multiplicative inverse the `field` gadgets name is not proven, and is
not assumed either: it exists because the prime is prime, by Fermat's little theorem, facts
that need machinery beyond this core-only development, so they are noted rather than
axiomatised.

Build with `lake build`. There are no `sorry`s and no axioms beyond the logical core.
