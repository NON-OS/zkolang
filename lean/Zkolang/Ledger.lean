/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Ledger.Range
import Zkolang.Ledger.Guard
import Zkolang.Ledger.Decompose
import Zkolang.Ledger.FieldBits
import Zkolang.Ledger.Divide
import Zkolang.Ledger.Index

/-!
The constraint ledger of the edition 2026 compiler (`docs/audit/constraints.md`), its
soundness arguments machine-checked: the guards, the bounds checks of an index and a shift,
bit decomposition, field bits, and division.
-/
