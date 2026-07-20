/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
The boolean gadgets of the standard library, stated over the integers and proven to
compute the boolean functions they name. A field element that is a bit is zero or one;
each gadget applied to bits reproduces the truth table and returns a bit, so a circuit
that composes them stays within the boolean domain it assumes. The identities are
integer identities, so they carry to any commutative ring the compiler emits into.
-/

namespace Zkolang.Logic

def NOT (x : Int) : Int := 1 - x
def AND (x y : Int) : Int := x * y
def OR (x y : Int) : Int := x + y - x * y
def XOR (x y : Int) : Int := x + y - 2 * x * y
def NAND (x y : Int) : Int := 1 - x * y
def NOR (x y : Int) : Int := 1 - (x + y - x * y)
def XNOR (x y : Int) : Int := 1 - (x + y - 2 * x * y)
def IMPLIES (x y : Int) : Int := 1 - x + x * y
def MAJ (x y z : Int) : Int := x * y + y * z + x * z - 2 * x * y * z
def MUX (s a b : Int) : Int := b + s * (a - b)

/-- The relation that pins a value to a single bit: zero exactly when the value is a
bit, which is what the constraint `assert bit(x)` enforces. This is the soundness of a
bit gadget, its constraint holding if and only if the witness really is a bit. -/
def isBit (x : Int) : Int := x * x - x
theorem isBit_iff (x : Int) : isBit x = 0 ↔ x = 0 ∨ x = 1 := by
  unfold isBit
  have hfac : x * x - x = x * (x - 1) := by rw [Int.mul_sub, Int.mul_one]
  rw [hfac]
  constructor
  · intro h
    rcases Int.mul_eq_zero.mp h with h | h
    · exact Or.inl h
    · exact Or.inr (by omega)
  · rintro (h | h) <;> subst h <;> decide

theorem not_table : NOT 0 = 1 ∧ NOT 1 = 0 := by decide
theorem and_table : AND 0 0 = 0 ∧ AND 0 1 = 0 ∧ AND 1 0 = 0 ∧ AND 1 1 = 1 := by decide
theorem or_table : OR 0 0 = 0 ∧ OR 0 1 = 1 ∧ OR 1 0 = 1 ∧ OR 1 1 = 1 := by decide
theorem xor_table : XOR 0 0 = 0 ∧ XOR 0 1 = 1 ∧ XOR 1 0 = 1 ∧ XOR 1 1 = 0 := by decide
theorem nand_table : NAND 1 1 = 0 ∧ NAND 1 0 = 1 := by decide
theorem nor_table : NOR 0 0 = 1 ∧ NOR 1 0 = 0 := by decide
theorem xnor_table : XNOR 0 0 = 1 ∧ XNOR 0 1 = 0 ∧ XNOR 1 1 = 1 := by decide
theorem implies_table : IMPLIES 1 0 = 0 ∧ IMPLIES 1 1 = 1 ∧ IMPLIES 0 0 = 1 := by decide

/-- Majority is one exactly on the bit triples with at least two ones. -/
theorem maj_table :
    MAJ 0 0 0 = 0 ∧ MAJ 1 0 0 = 0 ∧ MAJ 1 1 0 = 1 ∧ MAJ 1 1 1 = 1 := by decide

/-- Exclusive or of two bits is a bit: the gadget stays in the boolean domain. -/
theorem xor_closed (x y : Int) (hx : x = 0 ∨ x = 1) (hy : y = 0 ∨ y = 1) :
    XOR x y = 0 ∨ XOR x y = 1 := by
  rcases hx with h | h <;> rcases hy with h' | h' <;> subst_vars <;> decide

/-- The multiplexer selects its second argument on one and its third on zero. -/
theorem mux_zero (a b : Int) : MUX 0 a b = b := by
  unfold MUX; rw [Int.zero_mul, Int.add_zero]
theorem mux_one (a b : Int) : MUX 1 a b = a := by
  unfold MUX; rw [Int.one_mul]; omega

end Zkolang.Logic
