/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang
import Lean

/-!
Every theorem of the library with the axioms its proof rests on, listed by Lean from its
own environment rather than found by a search of the text, so a theorem in a nested
module, or one whose name another prefixes, is listed like any other. A clean proof over
the core library rests on `propext`, `Classical.choice` and `Quot.sound` at most. The
verify workflow runs this file and fails if a line names `sorryAx`, the axiom a
placeholder proof adds, or `Lean.ofReduceBool`, the one `native_decide` adds.
-/

open Lean in
#eval show CoreM Unit from do
  let env ← getEnv
  let mut names : Array Name := #[]
  for (name, info) in env.constants.toList do
    if (`Zkolang).isPrefixOf name && !name.isInternal then
      if let .thmInfo _ := info then
        names := names.push name
  names := names.qsort (fun a b => a.toString < b.toString)
  for name in names do
    let axs ← collectAxioms name
    IO.println s!"{name}: {axs.toList}"
  IO.println s!"{names.size} theorems"
