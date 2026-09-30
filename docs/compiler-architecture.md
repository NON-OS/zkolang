<!-- NONOS. AGPL-3.0-or-later. -->

# The zKølang compiler

This is how the edition 2026 compiler is built and why. The language it implements is
[`SPEC.md`](../SPEC.md); the machine it targets is the twelve-instruction step machine of
`nonos_zkolang/src/isa`. The compiler is part of the trusted computing base: a
miscompile is a proof of the wrong statement. Every design choice below is made for
correctness first, then for the size of the trace, then for compile speed.

## Status

This is the design the compiler is being built to. So far `source/`, `diag/`, `syntax/`,
`sema/`, `tir/`, `interp/`, `ssa/`, `lower/`, `opt/`, `gadget/`, `schedule/`, `codegen/`
and `driver/` exist. The front end reads a file into a syntax tree; after an error the
parser recovers, so that one run reports each independent mistake once, and a placement
check reports statement forms where they cannot stand. Diagnostics render as text or
JSON. The checker resolves names, checks types and constants, reports recursion, checks
secret flow and lowers the program to the typed IR, which the reference interpreter runs.
It checks a program in one file, structs, enums, `match`, `impl` blocks, methods and
`Self` included, and generic structs, enums and aliases, and reports each form it does
not check yet (generic functions and `impl` blocks, and modules in their own files) as
E0904. Each instance of a generic struct or enum is one entry of the type table; type
inference has general variables beside literal ones, unifies instances by their
arguments, and resolves an instance whose arguments it settles. `sema/exhaust` decides whether a `match` covers every
value, and which arms no value reaches, by the usefulness of its patterns over their
constructors, integer ranges cut at the ends of the ranges its arms name. The back end lowers the typed IR to SSA, runs the
passes, expands the gadgets, schedules, allocates registers, emits machine code and checks
it against the SSA; `driver::build` and `driver::prove` build a program from source and prove
a run of it, hiding the witness. The programs in `nonos_zkolang_proofs/ui/syntax` and
`ui/sema`, each of which states the diagnostics it expects, the `semantics/*.zkl` tests,
run both interpreted and compiled, and the `front_*`, `sema_*`, `ssa_*`, `compile_*`,
`codegen_*`, `flow_*`, `match_*` and `prove_2026_*` tests of `nonos_zkolang_proofs` pin
this behaviour; `match_exhaust_tests` checks exhaustiveness against every value of a
small type. The compiler lands in stages, in this order: the front end; types and secret
flow; the SSA IR, its passes, allocation and code generation; structs, enums, `match`
and generics; packages and the standard library; the tools; and the Lean development.
This section is updated as each stage lands.

## Where it lives

The compiler is the `compiler` module of the `nonos_zkolang` crate, which stays `no_std`
(with `alloc`) so the kernel terminal and editor embed it. The edition 2025 compiler is the
older `lang` module; it is frozen, fixed only for bugs, and its output for every program in
the tree is pinned by `nonos_zkolang_proofs/golden/legacy-commitments.txt`.

```
nonos_zkolang/src/compiler/
  source/     source files, file ids, spans, the source map, the SourceProvider trait
  diag/       diagnostics: codes, severities, labels, notes, help, rendering
  syntax/     tokens, the lexer, the concrete syntax tree nodes (AST), the parser
  pkg/        the manifest reader, module loading, the embedded std sources
  sema/       name resolution, types, type checking and inference, instantiation,
              constant evaluation, exhaustiveness, secret flow, lints
  tir/        the typed IR: monomorphic, resolved, desugared; its printer
  interp/     the reference interpreter over the TIR (the oracle), also the constant
              evaluator
  ssa/        the SSA IR, its printer and its interpreter (the witness generator)
  lower/      TIR to SSA lowering (scalarisation, guards, if-conversion, unrolling)
  opt/        the SSA passes
  gadget/     expansion of high-level SSA constraints into machine-level SSA
  schedule/   the order of the machine-level SSA, chosen for register pressure
  codegen/    register allocation, machine instructions, input and advice layout,
              padding, and the program verifier
  cost/       the cost report
  fmt/        the formatter
  driver/     the pipeline: compile, witness, run, prove
```

`mod.rs` files re-export only. One concept per file.

## The pipeline

```
source ─lex─▶ tokens ─parse─▶ AST ─resolve─▶ resolved AST ─check─▶ typed, instantiated
  ─lower─▶ TIR ─lower─▶ SSA ─passes─▶ SSA ─expand─▶ machine SSA ─schedule, allocate─▶
  ops ─verify─▶ program
```

Every stage is a pure function of its input. Each IR has a printer, reached by
`zkolang build --emit=ast|tir|ssa|asm`.

### Front end

The lexer is hand-written and total: every byte sequence produces a token stream, with
invalid input turned into error tokens and diagnostics, never a panic. Comments are kept
as trivia so the formatter and the doc tool see them. The parser is recursive descent
with a precedence climber for binary operators and an explicit nesting budget over
expressions, blocks and types: exceeding it is a diagnostic, not a stack overflow. On
an error the parser records a diagnostic, inserts an error node, and resynchronises at the
next `;`, `}` or item keyword, so one run reports many errors. Every node carries a span.

### Semantic analysis

Resolution builds the module tree through a `SourceProvider`, so the core never touches a
filesystem: the CLI provides files, the kernel its own store, and `std` is embedded with
`include_str!`. Each module gets one item namespace; `use` imports are resolved to a fixed
point, then every path is resolved to a definition id.

Type checking is bidirectional and local. Function signatures are explicit; inside a body,
an expected type flows down into literals, blocks, `if` and `match` arms, and array
elements, and a unifier with integer-literal variables handles what flows up. Generic
items are templates, checked per instantiation after substitution, which keeps the checker
small and exact: there are no trait bounds to solve because every operation is checked on
concrete types. An error inside an instantiation is reported at the instantiating call with
a note at the template line.

Constant evaluation runs the reference interpreter on the TIR of the constant expression,
with a step budget. Exhaustiveness uses the usual pattern-matrix usefulness algorithm over
the finite types. The secret-flow check runs on the TIR (section 13 of the
spec). It walks each function once, callees first, and writes a summary in terms of the
labels of its parameters' parts: the labels of the result and of each `&mut` parameter's
final value, and the parts that must be public. A call applies the callee's summary to
its arguments' labels. The walk threads the guard's label through branches, short-circuit
operands and early exits, and iterates each loop to a fixed point.

### TIR

The typed IR is the checked program with every name resolved to a local, constant or
function id, every type settled, integer literals checked and folded with their sign, and
built-in methods made direct operations. It keeps structured control flow, compound
assignment and `&&`/`||` as operators of a chain, so the reference interpreter evaluates it
directly with the semantics of the spec: only the taken branch, loops iterating, integers
as mathematical integers bounded by `i128` (every operation's operands and results of the
language fit it), fields modulo `p`.

### SSA

The SSA IR is a list of instructions over field values. It has no control flow at all:
TIR lowering unrolls loops, inlines calls, scalarises aggregates into slots (spec section
6), and if-converts branches.

Instructions:

| Kind | Instructions |
|---|---|
| values | `const c`, `input.pub i`, `input.sec i`, `add`, `sub`, `mul`, `sel c a b`, `eq a b` |
| partial | `inv a` (fails when `a = 0`) |
| constraints | `assert_zero a`, `assert_bool a`, `range a n` |
| advice | `hint kind(args) -> k values`, each value unconstrained until used |
| structured gadgets | `bits a n -> n values` (decomposition, canonical when `n = 64`), `divmod`, `cmp`, ... |
| output | `output i a` |

Every instruction records its span and inline stack, so a failure or a cost is reported at
the source line that caused it.

Guards: the lowering carries the current guard, a boolean SSA value. An instruction that
can fail is lowered under a guard so that it cannot fail when the guard is false: `assert
e` becomes `assert_zero(g * (1 - e))`; a range check of `x` becomes `range(sel(g, x, 0))`;
an inverse of `x` becomes `inv(sel(g, x, 1))`; a division's divisor becomes `sel(g, b, 1)`.
Assignment inside a branch writes a new SSA version; at the join the two versions are
merged with `sel` on the branch condition. This is standard if-conversion; its correctness
for the guarded semantics is one of the Lean theorems.

The SSA interpreter evaluates the list in order. It is the witness generator: evaluated on
the inputs, every `hint` computes its values, and those become the advice. There is no
fixed point to find, because a hint's arguments are always defined before it.

### Passes

Each pass is a function `Ssa -> Ssa`, individually switchable (`--passes=...`), and
individually tested for equivalence on a corpus and on generated programs.

- **Constant folding** evaluates instructions whose operands are constants, and
  **propagation** replaces uses. A `const` operand of `inv` that is zero is left alone, so
  the failure stays.
- **Algebraic simplification**: `x + 0`, `x * 1`, `x * 0`, `x - x`, `sel(c, x, x)`,
  `sel(1, a, b)`, `eq(x, x)`, double negation. The operands' own instructions are not
  removed by this; only the arithmetic changes.
- **CSE** hashes each pure instruction by operation and operands.
- **DCE** removes an instruction when it is pure and unused. Constraints, `inv`, `range`,
  `bits` and `output` are never pure.
- **Booleanity reuse**: `assert_bool x` is dropped when `x` is already known boolean (an
  `eq`, an earlier `assert_bool`, a bit of a decomposition, a product or `1 - b` of
  booleans).
- **Range-check merging**: of several `range x n` the smallest `n` is kept, and a range
  check implied by how `x` was built (a recomposition of `n` checked bits, a boolean) is
  dropped.

Soundness argument for every pass: a pass may only remove a constraint that is implied by
the constraints that remain, and may only change a value to one equal on every run that
satisfies the constraints. Each pass's test suite checks both the accepted set and the
outputs.

### Gadget expansion

High-level constraints become machine-level SSA, where every instruction is one machine
instruction: `range x n` becomes `n` advice bits, `n` booleanity checks and a Horner
recomposition equated with `x`; `bits x 64` adds the canonicity check that the bits encode
an integer below `p` (without it, a 64-bit decomposition of a small value has a second
solution, `x + p`); comparisons, division, bit operations, dynamic indexing and the `u64`
limb arithmetic each have a gadget. Every gadget is in the constraint ledger
(`docs/audit/constraints.md`) with the test that rejects a witness violating it.

### Register allocation

The machine has 32 registers and no memory. Allocation is where a wrong program is most
easily produced, so it is built to be checked. One fact about the proof shapes it: the
step AIR pins a row that reads a public input to the committed value, but a row that
reads a secret input or an advice slot holds whatever the prover writes there. Two reads
of one such slot could differ, so each is read at most once, and a value that must be
read again is held in a register or recomputed.

1. **Scheduling.** Gadget expansion and lowering emit bit-level work in lockstep, each
   operand's bits read from the top down together with the sums that check them, so that
   one bit of each is held at a time. The scheduler then tries two orders, the program's
   own and one evaluating every constraint and output depth first, the operand needing
   the most registers first, and keeps the one holding fewer values at once. In both, a
   constant, input or advice value is placed only where it is first needed; a
   constraint or output as soon as its operands are; and an operation as soon as its
   operands are when it is the last reader of a held operand.
2. **Rematerialisation.** A constant is written again with `Imm` and a public input read
   again with `Inp` when its register was taken. Nothing else is: a secret input or an
   advice value is never read twice, and a computed value is kept.
3. **Allocation.** One walk over the schedule assigns each value a register from where it
   is placed to its last use; when none is free, the constant or public input needed
   furthest ahead gives up its register. If every register holds a value that cannot be
   brought back, compilation stops with E0801.
4. **Padding.** A program shorter than 33 rows is padded with constants written before
   its halt, since a trace of 32 rows cannot carry the blinding of a hiding proof.
5. **Verification.** An independent checker replays the emitted instructions, tracking
   which machine SSA value each register holds, and confirms that every operand of every
   instruction is the value the machine SSA says it should be, that every read of an
   input or advice value reads its own slot, that no secret input or advice slot is read
   twice, and that every constraint, output, inverse and selection appears. A failure of
   this checker is an internal compiler error, reported and never emitted.

### Witness, run and prove

The driver compiles, runs the SSA interpreter to produce the advice, runs the machine with
every constraint enforced, runs the reference interpreter beside it, and proves through
the existing `nonos-stark` API, hiding the witness. The two runs must both fail or return
the same result; a disagreement is reported as a compiler bug. A failing run has no
witness, so the reference interpreter's failure, with its source span, is the report.

## Testing

- The reference interpreter is the oracle. For every corpus program and for generated
  well-typed programs it must agree with the machine run of the compiled program, with a
  proof that verifies for the honest witness, and with rejection of every mutated advice
  value (the uniqueness property of the spec, section 21.3).
- A random program generator produces well-typed programs over every feature; failures
  are minimised and kept as regression tests. It runs in CI with a fixed budget.
- `cargo fuzz` targets cover the lexer, the parser, the checker on arbitrary syntax trees,
  and the whole pipeline. The corpus is committed. No input may panic.
- Every pass has before and after equivalence tests, and the pipeline is tested with every
  pass on and off.
- The emitted assembly of the standard library and the examples is pinned as golden files.
- `lean/` proves the SSA semantics preserved by constant folding, CSE, DCE and the guarded
  lowering of `if`.
