/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Field

/-!
The guards of the constraint ledger, section 1. Lowering keeps the guard `g` of the point
it is at, a bit that is one at `main`'s entry, and folds it into what it writes, so a
failure counts only at a point whose guard is true (`SPEC.md` section 14.1).
-/

namespace Zkolang.Ledger.Guard
open Zkolang.Field

/-- `require(ok)` writes `AssertZero(g · (1 − ok))` for a guard bit `g` and a condition bit
`ok`. It holds exactly when the guard is off or the condition holds. -/
theorem require_holds (g ok : Int) (hg : g = 0 ∨ g = 1) (hok : ok = 0 ∨ ok = 1) :
    cong (g * (1 - ok)) 0 ↔ g = 0 ∨ ok = 1 := by
  unfold cong
  rcases hg with rfl | rfl <;> rcases hok with rfl | rfl <;> decide

/-- `require_zero(x)` writes `AssertZero(g · x)`: under a true guard it holds exactly when
`x` is zero in the field. -/
theorem require_zero_on (x : Int) : cong (1 * x) 0 ↔ cong x 0 := by
  rw [Int.one_mul]

/-- Under a false guard `AssertZero(g · x)` holds whatever `x` is. -/
theorem require_zero_off (x : Int) : cong (0 * x) 0 := by
  unfold cong; rw [Int.zero_mul]

/-- `guarded(x, safe)` is `Sel(g, x, safe)`, read as `g · x + (1 − g) · safe`: under a true
guard it is `x`. -/
theorem guarded_on (x safe : Int) : 1 * x + (1 - 1) * safe = x := by omega

/-- Under a false guard `guarded(x, safe)` is the safe value, so a gadget on a path that
does not run sees only an operand it accepts and constrains nothing that could fail. -/
theorem guarded_off (x safe : Int) : 0 * x + (1 - 0) * safe = safe := by omega

end Zkolang.Ledger.Guard
