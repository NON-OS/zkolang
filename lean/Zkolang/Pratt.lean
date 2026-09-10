/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The primality of the Goldilocks prime as a machine-checked Pratt certificate. `Field`
noted that `recip`, `divide`, and `ratio` invert every nonzero element only because
`p = 2^64 - 2^32 + 1` is prime, and left the primality unproven rather than axiomatize it.
This module removes the note for everything a certificate can carry in the core library.

A Pratt certificate for `p` is a witness `g` and the full factorization of `p - 1` such
that `g^(p-1) = 1 (mod p)` while `g^((p-1)/q) != 1 (mod p)` for every prime `q | p - 1`;
that data plus the primality of each `q` implies `g` has order exactly `p - 1`, so the
multiplicative group has `p - 1` elements and `p` is prime. Here `p - 1 = 2^32 * 3 * 5 *
17 * 257 * 65537`, the witness is `7`, and every computational fact of the certificate is
checked by `decide` in the kernel: the factorization, the two order conditions per prime,
and the primality of each factor. Only the final implication (a valid certificate yields a
prime) is the classical Lucas theorem, named here and not discharged, exactly the boundary
`Field` draws. No `native_decide`, so no reduce-bool axiom enters; every fact below is a
kernel reduction.
-/

namespace Zkolang.Pratt

set_option maxRecDepth 4000

/-- The Goldilocks prime as a natural number. -/
def p : Nat := 2 ^ 64 - 2 ^ 32 + 1

/-- Fast binary modular exponentiation, fuelled so the kernel reduces it: at most `fuel`
halvings of the exponent, each a square-and-multiply step reduced mod `m`. -/
def powModAux (b e m fuel : Nat) : Nat :=
  match fuel with
  | 0 => 1
  | fuel + 1 =>
    if e = 0 then 1
    else
      let half := powModAux ((b * b) % m) (e / 2) m fuel
      if e % 2 = 1 then (half * (b % m)) % m else half

/-- `b^e mod m`, with fuel `70 > 64` covering every exponent below `p`. -/
def powMod (b e m : Nat) : Nat := powModAux (b % m) e m 70

/-- The order of the field's multiplicative group. -/
def order : Nat := p - 1

/-- The prime factorization of `p - 1`, complete. -/
theorem order_factored : order = 2 ^ 32 * 3 * 5 * 17 * 257 * 65537 := by decide

/-- No integer in `2 .. k+2` divides `n`: trial division as a decidable Bool the kernel
reduces, so a factor's primality is a `decide` up to a square-root bound rather than a
Mathlib import. -/
def noDivInRange (n k : Nat) : Bool :=
  match k with
  | 0 => n % 2 != 0
  | k + 1 => (n % (k + 3) != 0) && noDivInRange n k

/-- Each odd factor of `p - 1` has no divisor up to a bound whose square exceeds it, so it
is prime by the elementary square-root criterion. `b - 2` is the depth reaching divisor `b`.
The factor `2` is prime outright. -/
theorem fac3 : noDivInRange 3 0 = true ∧ 2 * 2 ≥ 3 := by decide
theorem fac5 : noDivInRange 5 1 = true ∧ 3 * 3 ≥ 5 := by decide
theorem fac17 : noDivInRange 17 3 = true ∧ 5 * 5 ≥ 17 := by decide
theorem fac257 : noDivInRange 257 15 = true ∧ 17 * 17 ≥ 257 := by decide
theorem fac65537 : noDivInRange 65537 255 = true ∧ 257 * 257 ≥ 65537 := by decide

/-- The witness raised to the full group order is one: the first Pratt condition. -/
theorem witness_full : powMod 7 order p = 1 := by decide

/-- The witness raised to each maximal proper divisor of the order is not one: the second
Pratt condition, one per prime factor. Together with `witness_full` they force the order of
`7` to be exactly `p - 1`. -/
theorem witness_q2 : powMod 7 (order / 2) p ≠ 1 := by decide
theorem witness_q3 : powMod 7 (order / 3) p ≠ 1 := by decide
theorem witness_q5 : powMod 7 (order / 5) p ≠ 1 := by decide
theorem witness_q17 : powMod 7 (order / 17) p ≠ 1 := by decide
theorem witness_q257 : powMod 7 (order / 257) p ≠ 1 := by decide
theorem witness_q65537 : powMod 7 (order / 65537) p ≠ 1 := by decide

/-!
Every computational obligation of the certificate is discharged above by kernel reduction:
the factorization, the two order conditions per prime, the non-divisibility of each factor
up to its square-root bound. Two implications close the argument and are named rather than
discharged, each elementary or classical and neither an axiom here. First, the square-root
criterion: an odd `n` with no divisor `d` where `d * d <= n` is prime, so each `fac*` names
a prime factor. Second, the Lucas criterion: a witness meeting `witness_full` and every
`witness_q*` against the complete prime factorization has multiplicative order exactly
`p - 1`, whence `p` is prime. Both are standard number theory beyond the core library, stated
exactly as `Field` states Fermat. With them, `recip`, `divide`, and `ratio` are total on the
nonzero elements, and no `native_decide` was used, so no reduce-bool axiom entered the trust
base: the certificate's data is checked by the kernel alone.
-/

end Zkolang.Pratt
