<!-- NONOS. AGPL-3.0-or-later. -->

# The constraint ledger

This ledger lists the constraints the edition 2026 compiler writes so that a compiled
program keeps the compilation contract of `SPEC.md` section 14.3: a run that meets a
failure condition of section 14.1 has no accepting execution of the machine program, and
an accepting execution has no advice value but the one the witness generator gives.

It takes each machine instruction of section 21.1 to do on every row what it names: an
`Assert` row holds only for zero, a `Bool` row only for zero or one, an `Inv` row has no
result for zero, and a `Sel` row gives its first operand when its condition is one and the
second when it is zero. Whether the proof system enforces those relations is a property of
the prover, outside this ledger; the ledger covers what the compiler builds on them. Paths
are under `nonos_zkolang/src/compiler/`; tests under `nonos_zkolang_proofs/`.

## 1. Guards

A failure counts only at a point whose guard is true (section 14.1). Lowering keeps the
guard of the point it is at, `g`, a value that is zero or one and is one at `main`'s
entry (`lower/entry.rs`). The primitives of `lower/guard.rs` fold it in:

| Primitive | Constraint | Where `g` is 0 |
|---|---|---|
| `require(ok)`, `ok` a boolean | `AssertZero(g · (1 − ok))` | holds |
| `require_zero(x)` | `AssertZero(g · x)` | holds |
| `require_below(x, n)` | `RangeCheck(Sel(g, x, 0), n)` | checks 0, which holds |
| `guarded(x, safe)` | `Sel(g, x, safe)` | gives `safe` |

A gadget (section 3) is never multiplied by the guard. Its operands are guarded instead,
with a value the gadget accepts: 0 for a range check, 1 for a divisor or an inverted
value. So a gadget on a path that does not run constrains nothing that could fail.

## 2. The failure conditions

Each condition of section 14.1, the constraint that makes a run meeting it unsatisfiable,
and a test that runs a program meeting it, interpreted and compiled, and requires both to
fail (`semantics_compiled_tests.rs` runs every test of `semantics/` compiled as well).

| Condition | Constraint | Code | Test |
|---|---|---|---|
| `assert` of `false` | `require(c)` | `lower/block.rs` | `semantics/assert_false.zkl`, `a_guarded_one_fails_when_its_guard_holds` |
| overflow, 32 bits or fewer | the result below `2^n`, a signed one after adding `2^(n−1)`: `check_int` | `lower/int.rs` | `semantics/integers.zkl`, `u8_overflow_fails`, `min_over_minus_one_fails`, `negating_min_fails`, `pow_overflow_fails` |
| overflow, 64 bits | carries and signs of the two 32-bit halves | `lower/int64/` | `semantics/wide.zkl`, `semantics/wide_edges.zkl`, `semantics/pow64.zkl` |
| division or remainder by zero | `AssertZero(g · (b == 0))`, then the division gadget on guarded operands | `lower/int_div.rs`, `lower/int64/div.rs` | `semantics/integers.zkl`, `division_by_zero_fails` |
| field `/` by zero, inverse of zero | `Inv(Sel(g, b, 1))` | `lower/binop_scalar.rs`, `lower/builtin.rs` | `semantics/fields.zkl`, `field_division_by_zero_fails`, `inverse_of_zero_fails` |
| dynamic index out of bounds | for length `n`: `(n − 1) − i` below `2^bits(n − 1)`; for `n = 0`, `require(0)` | `lower/place_index.rs` | `semantics/arrays.zkl`, `dynamic_index_out_of_bounds_fails` |
| shift by at least the width | `(N − 1) − k` below `2^log2(N)` | `lower/shift.rs`, `lower/int64/shift.rs` | `semantics/bits.zkl`, `shift_by_the_width_fails` |
| failing `checked_from` | the value below the target's bound | `lower/conv.rs`, `lower/int64/conv.rs`, `lower/int64/narrow.rs` | `semantics/conversions.zkl`, `checked_from_out_of_range_fails`, `checked_from_negative_fails`, `bool_from_two_fails`; `semantics/wide_edges.zkl`, `a_negative_to_unsigned_fails` |
| `from_le_bits` of a non-canonical `field` | `AssertZero(g · (hi == 2^32 − 1) · lo)` | `lower/bits_conv.rs` | `semantics/bits.zkl`, `non_canonical_field_bits_fail` |
| `while` past its limit | after `limit` iterations, `require(!cond)` | `lower/iteration.rs` | `semantics/control.zkl`, `while_past_its_limit_fails` |
| an input outside its type | a `bool` slot `AssertBool`; an integer of 32 bits or fewer `check_int`; each half of a wider one below `2^32`; a record's parts each by its type; an enum's tag one of its variants, the fields of that variant each by its type, and every slot that variant does not use zero | `lower/inputs.rs`, `lower/inputs_enum.rs` | `prove_2026_enum_tests.rs`, `the_compiled_program_rejects_them_too`; `field_input_tests.rs`, `an_input_past_the_modulus_is_refused` |

Why the index check is exact: an index is a `usize`, 32 bits, so `i < 2^32`. For `i < n`
the difference `(n − 1) − i` is at most `n − 1 < 2^bits(n − 1)`; for `i ≥ n` it is
`p − (i − n + 1) > p − 2^32`, far above `2^bits(n − 1)`.

## 3. Gadgets

The gadget instructions of the SSA IR (`ssa/inst.rs`) are expanded into machine-level
ones in `gadget/`, by `expand_division` and then `expand_bits`, before scheduling
(`driver/backend.rs`). Each reads advice, values the prover supplies that the witness
generator finds from the instruction's hint (section 21.3), and constrains them.

### Bit decomposition: `Bit(v, k, n)` and `RangeCheck(v, n)`, `n < 64`

`gadget/decompose.rs` and `gadget/read_bits.rs`. Advice: `n` bits `b_k`, the hint
`Bit(v, k)`. Constraints: `AssertBool(b_k)` for each, and `AssertZero(Σ 2^k b_k − v)`.

*Soundness.* The sum lies in `[0, 2^n)`, below `2^63 < p`, so its equality with `v` in the
field is equality of integers: `v`'s canonical representative is below `2^n`, and its bits
are the `b_k`, which are therefore unique. *Completeness.* The hint gives bit `k` of `v`.

A decomposition of the same value and width written at most 64 instructions earlier in the
program being expanded (`NEAR`) is reused rather than written again. For `n ≥ 64` the
gadget writes `AssertZero(1)`, which fails, as the semantics does outside the domain.

### Field bits: `FieldBit(v, k)`

`gadget/field_bits.rs`. Advice: 64 bits, each `AssertBool`, read as `lo` (bits 0 to 31) and
`hi` (bits 32 to 63). Constraints: `AssertZero(lo + 2^32 · hi − v)` and
`AssertZero((hi == 2^32 − 1) · lo)`.

*Soundness.* `lo + 2^32 · hi` lies in `[0, 2^64)` and is congruent to `v` modulo
`p = 2^64 − 2^32 + 1`. The integers in that range congruent to `v` are its canonical
representative and, when `v < 2^32 − 1`, also `v + p`, whose `hi` is `2^32 − 1` and whose
`lo` is `v + 1 > 0`: the second constraint refuses it. The largest canonical value,
`p − 1`, has `hi = 2^32 − 1` and `lo = 0`, and passes. So the bits are those of the
canonical representative. Test: `ssa_gadget_tests.rs`,
`field_bits_admit_only_the_canonical_representative`.

### Division: `Quot(a, b, n)` and `Rem(a, b, n)`, `n ≤ 32`

`gadget/divide.rs`. Advice: `q` and `r`, the hints `Quot(a, b)` and `Rem(a, b)`.
Constraints: `a`, `b`, `q`, `r` and `b − r − 1` each below `2^n`, and
`AssertZero(q · b + r − a)`.

*Soundness.* `b − r − 1` below `2^n`, with `b` and `r` below `2^32`, holds only when
`r < b` as integers, so `b ≥ 1`. With `q, b < 2^32`, `q · b ≤ (2^32 − 1)^2 = 2^64 − 2^33 + 1`,
and with `r < 2^32 − 1`, `a + p − r ≥ 2^64 − 2^33 + 3`: so `q · b + r = a` holds in the field
only as integers, and integer division makes `q` and `r` unique. Test:
`ssa_gadget_tests.rs`, `division_admits_only_the_true_quotient_and_remainder`. The quotient
and remainder of one pair of operands share one expansion. For `n > 32` the gadget writes
`AssertZero(1)`.

### Pinned advice

Every advice value of a compiled program is pinned: changing any one makes the machine
program reject. `compile_pinned_tests.rs`, `every_advice_value_of_a_compiled_program_is_pinned`,
checks it over compiled programs; `ssa_backend_tests.rs`,
`the_machine_program_runs_as_the_ssa_program`, over random SSA programs, where the machine
program must also accept exactly when the SSA semantics does.

## 4. Range checks the compiler drops

`opt/elide.rs` drops a `RangeCheck(v, n)` whose bound always holds where it stands, and
writes `Const(0)` in its place so the program's indices stay. The bound of each value
(`opt/bounds.rs`) comes from its definition (a constant is exact; a sum, product or
difference counts only where it cannot wrap past `p − 1`; `Sel` takes the larger arm;
`Eq`, `Bit` and `FieldBit` are at most 1) and from the constraints before it: a range check
or bit of `v` at `n` bits bounds `v` below `2^n`, a division's operands likewise, and
`AssertBool(v)` bounds `v` by 1. The SSA program is one straight line, so each such
constraint holds in every accepting run that reaches the dropped check. A check of 64 or
more bits, which the semantics fails, is never dropped.
