# zKølang gadget soundness, in Lean 4

The standard library gadgets are polynomial expressions over the field. This project
proves, in Lean 4 with no dependency beyond the core library, that each gadget computes
the function it names when its inputs are bits, that the outputs stay bits, that a byte
decomposition can only represent a value in the range zero through two hundred fifty
five, and that the transfer balance constraint holds exactly when value is conserved.

The gadgets are stated over the integers. Every proof is an integer polynomial identity
or a linear fact, so it carries to any commutative ring the compiler targets, the
Goldilocks field included, through the ring homomorphism from the integers.

Build with `lake build`. There are no `sorry`s and no axioms beyond the logical core.
