<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# Reference

## Opcodes

The instruction set (`src/isa.rs`). Every opcode is enforced by the AIR.

| Opcode | Fields | Effect |
|---|---|---|
| `Imm` | `d, v` | `r_d = v` |
| `Add` | `d, a, b` | `r_d = r_a + r_b` |
| `Sub` | `d, a, b` | `r_d = r_a - r_b` |
| `Mul` | `d, a, b` | `r_d = r_a * r_b` |
| `Inv` | `d, a` | `r_d = r_a^{-1}` (zero is unprovable) |
| `Sel` | `d, c, a, b` | `r_d = r_c ? r_a : r_b`, `r_c` a bit |
| `Eq` | `d, a, b` | `r_d = (r_a == r_b)` as `{0,1}` |
| `Bool` | `a` | constrain `r_a` to `{0,1}` |
| `Assert` | `a` | constrain `r_a = 0` |
| `Inp` | `d, idx` | `r_d = input[idx]` (public or secret) |
| `Out` | `a, idx` | `public_output[idx] = r_a` |
| `Halt` | | end of program |

## Grammar

```
program  := item*
item     := constdef | fndef | stmt
constdef := 'const' ident '=' '[' number (',' number)* ']' ';'
fndef    := 'fn' ident '(' params? ')' '=' expr ';'
params   := ident (',' ident)*
stmt     := 'let' ident '=' expr ';' | 'assert' expr ';'
          | 'input' ident ';' | 'secret' ident ';' | 'output' expr ';'
          | 'for' ident 'in' number '..' number '{' stmt* '}'
expr     := equality
equality := sum (('==' | '!=') sum)?
sum      := product (('+' | '-') product)*
product  := unary (('*' | '/') unary)*
unary    := '-' unary | primary
primary  := atom ('[' expr ']')*
atom     := number | ident | ident '(' args? ')' | '(' expr ')' | '[' args? ']'
          | 'inv' '(' expr ')' | 'sel' '(' expr ',' expr ',' expr ')'
          | 'if' expr '{' expr '}' 'else' '{' expr '}'
args     := expr (',' expr)*
```

Keywords: `let`, `assert`, `input`, `secret`, `output`, `inv`, `sel`, `for`, `in`,
`if`, `else`, `fn`, `const`. Operators: `+ - * / == != ` and unary `-`. Comments:
`//` to end of line.

## Public API

From `nonos_zkolang` (`src/lib.rs`):

- `compile_source(&str) -> Result<Vec<Op>, CompileError>`
- `compile_source_full(&str) -> Result<Compiled, CompileError>`: the program with the
  advice plan its ordered comparisons need
- `to_c(&Compiled) -> String`, `to_asm(&Compiled) -> String`,
  `to_python(&Compiled) -> String`: native targets that take the public inputs and
  secrets and compute each comparison's bits themselves
- `prove_source(&str) -> Result<Report, RunError>`
- `prove_source_with_inputs(&str, &[u64]) -> Result<Report, RunError>`
- `prove_source_with_witness(&str, &[u64] public, &[u64] secret) -> Result<Report, RunError>`
- `prove_program(&[Op], &[Fp]) -> Result<Report, RunError>`
- `commit(&[Op]) -> [u8; 32]`, `commit_limbs(&[Op]) -> [Fp; 4]`, `serialize(&[Op]) -> Vec<u8>`
- `quote(&Report) -> Quote`
- `verifier_key(&[Op], u32) -> Result<[u8; 32], KeyError>`, `periodic_root(&[Op], u32) -> Result<[u8; 32], KeyError>`
- `registration_key(&[Op]) -> Result<[u8; 32], KeyError>`, `registration_root(&[Op]) -> Result<[u8; 32], KeyError>`, `REGISTRATION_RATE`
- `StepAir`, `Vm`, `Trace`, `Report`, `Op`, `REGS`, `TRACE_WIDTH`

The verifier key binds a program commitment to its wiring at the fixed
registration rate; `registration_key` and `registration_root` are the rate-pinned
values a NOX proving market registers and challenges against. See
[the recursion ABI](09-recursion-abi.md).

`Report` carries `verified`, `steps`, `log_trace_len`, `trace_len`, `trace_width`,
`outputs`, and `program_commit` (the 32-byte commitment the proof is bound to).

## Errors

Each failure is a typed value, never a panic.

- `CompileError` (`src/lang/error.rs`): `UnexpectedChar { at }`,
  `NumberTooLarge { at }`, `UnexpectedEof`, `UnexpectedToken`, `UnknownVariable`,
  `TooManyRegisters`, `LoopTooLarge`, `ProgramTooLong`, `UnknownFunction`,
  `ArityMismatch`, `RecursionTooDeep`, `NotIndexable`, `UnknownConst`,
  `NonConstantIndex`, `IndexOutOfBounds`, `ArrayNotScalar`, `TupleNotScalar`,
  `TupleArity`, `IncludeNotFound`, `IncludeTooDeep`, `IoLimit`, `NestingTooDeep`,
  `ExpressionTooLarge`, and `Name` carrying a `NameError`.
- `NameError` (`src/lang/name_error.rs`): `Duplicate`, `ShadowsConstant`,
  `ShadowsLoopVariable`, `Recursive`.
- `ProveError` (`src/vm/`): `BadRegister`, `BadInput`, `NoHalt`,
  `Unprovable { step }`.
- `BuildError` (`src/air/`): `NoHalt`, `TooLong`, `MissingPublicOutput`.
- `RunError` (`src/driver/`): `Compile`, `Execute`, `Layout`, `ProgramTooLong`,
  `InputCount`, `TraceTooSmallToHide`, `InputNotInField`.
- `KeyError` (`src/vkey.rs`): `NoHalt`, `ProgramTooLong`.

## Limits

- Thirty-two registers. A program needing more live values than the file holds is
  a `TooManyRegisters` compile error.
- Straight-line only. No runtime loops, conditionals, or calls; bounded loops are
  unrolled and functions are inlined by the front-end.
- Traces up to 2^16 rows (`MAX_LOG_T` in `src/driver.rs`); a longer program is a
  `ProgramTooLong` error rather than a silent truncation.
- No random-access memory or hash primitive: zKølang is a straight-line
  arithmetic language. A memory extension is a future major version, not a
  missing part of the current one.

## Proof parameters

The driver proves at 32 queries, 16 grinding bits, and 3 extra blowup bits
(`src/driver.rs`), the same money-grade setting the `nonos-stark` tests use. The
trace width is 51 and the AIR has 46 transition constraints of degree at most
three.

## FAQ

**Is this zero-knowledge?** Partly, and the docs are careful about which part. A
`secret` input is a private witness: it feeds the run and never enters the public
statement, so a proof can attest knowledge of a hidden value that satisfies a
public relation, for example a square root of a public number. An edition 2026
proof blinds every trace column with fresh randomness, one random coefficient per
point the proof opens it at, so the trace values it reveals are jointly uniform.
An edition 2025 proof from `zkolang run` is not blinded, and its query openings can
show trace values. Full zero-knowledge, an argument that also covers the
composition polynomial and the FRI layers, is not claimed today.

**What is proven versus assumed?** The AIR constraints listed in
[the AIR](05-the-air.md) are proven: opcode semantics, register binding, ordering,
and public input and output binding. Assumed is the soundness of the underlying
STARK, which is the subject of the NØNOS verification paper, and the correctness of
the compiler and executor, which are checked by the host proof suite but not
formally verified.

**Can I trust the answer if `verified` is true?** You can trust that the program
whose text the verifier holds ran and produced the stated outputs on the stated
inputs. Whether that program is the right program for your purpose is your
judgement, not the proof's.

**Where do I start?** Read [the language](02-language.md), then run `zkolang` in
the terminal, then write a `.zkl` file and run `zkolang myfile.zkl`.

## Reproducing the claims

In `userland/nonos_zkolang_proofs`: `cargo test` runs the 102 host proofs behind this
documentation (opcode tamper rejection, register binding, public input and output
soundness, the language end to end including functions, the verifier-key binding at
the registration rate, and the fee model), and
`cargo run --release --example measure` prints the trace shapes and fees.
