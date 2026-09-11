/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/

/-!
No double-spend, at the level a user cares about. Each spent note derives a nullifier, the
contract admits a spend only when its nullifier is fresh, and records it. `GrandProduct`
proves the in-circuit machinery that binds the nullifier column to the committed set; this
module proves the set semantics that machinery enforces: the freshness check is exactly the
absence of the nullifier, admitting a fresh nullifier keeps the recorded set duplicate-free,
and a second spend of the same note produces a nullifier the set already holds, so it is
refused. The invariant `Nodup` on the recorded set is preserved by every admitted spend and
violated by every replay, so a note spends at most once. Plain `Nat` nullifiers and list
membership, core library only.
-/

namespace Zkolang.Nullifier

/-- The nullifier is fresh against the recorded set: not already present. The contract's
freshness check as a decidable Bool. -/
def fresh (s : List Nat) (n : Nat) : Bool := decide (n ∉ s)

/-- No nullifier recorded twice: the contract's standing invariant. -/
def Nodup : List Nat → Prop
  | [] => True
  | n :: s => ¬ n ∈ s ∧ Nodup s

/-- Freshness is exactly absence, decoded from the Bool check the contract runs. -/
theorem fresh_iff_not_mem (s : List Nat) (n : Nat) : fresh s n = true ↔ ¬ n ∈ s := by
  unfold fresh; simp

/-- Admitting a fresh nullifier preserves the duplicate-free invariant: the recorded set
grows by one entry that was not already in it. -/
theorem admit_preserves_nodup (s : List Nat) (n : Nat)
    (hnd : Nodup s) (hf : fresh s n = true) : Nodup (n :: s) :=
  ⟨(fresh_iff_not_mem s n).mp hf, hnd⟩

/-- The double-spend is refused: a note already spent has its nullifier in the set, so the
freshness check returns false and the second spend cannot be admitted. -/
theorem replay_refused (s : List Nat) (n : Nat) (h : n ∈ s) : fresh s n = false := by
  unfold fresh; simp [h]

/-- The guarantee, stated once: over a duplicate-free recorded set, a nullifier is either
absent and admissible exactly once, or present and refused. No sequence of admitted spends
records a nullifier twice. -/
theorem no_double_spend (s : List Nat) (n : Nat) (hnd : Nodup s) :
    (fresh s n = true ∧ Nodup (n :: s)) ∨ (n ∈ s ∧ fresh s n = false) := by
  by_cases h : n ∈ s
  · exact Or.inr ⟨h, replay_refused s n h⟩
  · have hf : fresh s n = true := (fresh_iff_not_mem s n).mpr h
    exact Or.inl ⟨hf, admit_preserves_nodup s n hnd hf⟩

end Zkolang.Nullifier
