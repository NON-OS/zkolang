/-
 zKølang by NØNOS
 AGPL-3.0-or-later
-/
import Zkolang.Logic

/-!
The selection gadgets, multiplexers built by nesting the one-bit primitive. A four-way mux
is a tree of three multiplexers over two select bits; soundness is that on bit selectors it
returns the data line the selectors address, so a select cannot read a line it did not name.
Each case follows from the one-bit multiplexer's soundness, so this is the primitive
composing into a wider select.
-/

namespace Zkolang.Select

open Zkolang.Logic

/-- `cond` is the one-bit select, the primitive itself. -/
def cond (c a b : Int) : Int := MUX c a b

/-- The four-way multiplexer: a tree over the high bit and the low bit. -/
def mux4 (s1 s0 d0 d1 d2 d3 : Int) : Int := MUX s1 (MUX s0 d3 d2) (MUX s0 d1 d0)

/-- Selector `00` reads line zero. -/
theorem mux4_00 (d0 d1 d2 d3 : Int) : mux4 0 0 d0 d1 d2 d3 = d0 := by
  unfold mux4; rw [mux_zero, mux_zero]

/-- Selector `01` reads line one. -/
theorem mux4_01 (d0 d1 d2 d3 : Int) : mux4 0 1 d0 d1 d2 d3 = d1 := by
  unfold mux4; rw [mux_zero, mux_one]

/-- Selector `10` reads line two. -/
theorem mux4_10 (d0 d1 d2 d3 : Int) : mux4 1 0 d0 d1 d2 d3 = d2 := by
  unfold mux4; rw [mux_one, mux_zero]

/-- Selector `11` reads line three. -/
theorem mux4_11 (d0 d1 d2 d3 : Int) : mux4 1 1 d0 d1 d2 d3 = d3 := by
  unfold mux4; rw [mux_one, mux_one]

/-- The one-bit select returns the first line on one and the second on zero. -/
theorem cond_one (a b : Int) : cond 1 a b = a := mux_one a b
theorem cond_zero (a b : Int) : cond 0 a b = b := mux_zero a b

end Zkolang.Select
