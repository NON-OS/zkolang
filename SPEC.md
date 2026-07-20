<!-- NONOS. AGPL-3.0-or-later. -->

# The zKølang language specification

zKølang is a language for verifiable computation. A program is straight-line: it reads
public inputs and a private witness, computes over a finite field, and writes public
outputs, and the whole run is proven by a transparent STARK. The language exists so that
one artifact both runs and proves, so every construct here is chosen to have an honest,
bounded cost in an execution trace. There is no heap, no general recursion, and no
unbounded loop, because none of those has a fixed trace.

This document specifies the language as the compiler implements it. It is normative
where it states a rule and descriptive where it explains one.

## 1. The field

All values are elements of the Goldilocks field, the prime field of order
`p = 2^64 - 2^32 + 1`. Arithmetic is modular. A numeric literal is reduced modulo `p`.
The field has two-adicity thirty two, which is what lets the prover build its evaluation
domains, and it fits a machine word, which is what lets the same computation run as
native code at the same cost budget.

## 2. Lexical structure

- **Comment.** `//` to end of line. Whitespace separates tokens and is otherwise
  insignificant.
- **Identifier.** `[A-Za-z_][A-Za-z0-9_]*`, not equal to a keyword.
- **Number.** `[0-9]+`, read as a field element modulo `p`.
- **String.** `"` up to the next `"`, used only as an include path.
- **Keywords.** `let const fn input secret output assert for in if else inv sel include`.
- **Operators and punctuation.** `+ - * / = == != ! && || .. ( ) [ ] { } , ;`.

## 3. Grammar

In EBNF. A program is a sequence of items.

```
program     = { item } ;
item        = include | const_def | fn_def | statement ;

include     = "include" string ";" ;
const_def   = "const" ident "=" ( number | array ) ";" ;
fn_def      = "fn" ident "(" [ ident { "," ident } ] ")" "=" expr ";" ;

statement   = let_stmt | input_stmt | secret_stmt
            | output_stmt | assert_stmt | for_stmt ;
let_stmt    = "let" ident "=" expr ";" ;
input_stmt  = "input" ident ";" ;
secret_stmt = "secret" ident ";" ;
output_stmt = "output" expr ";" ;
assert_stmt = "assert" expr ";" ;
for_stmt    = "for" ident "in" expr ".." expr "{" { statement } "}" ;

expr        = or ;
or          = and { "||" and } ;
and         = equality { "&&" equality } ;
equality    = sum [ ( "==" | "!=" | "<" | "<=" | ">" | ">=" ) sum ] ;
sum         = product { ( "+" | "-" ) product } ;
product     = unary { ( "*" | "/" ) unary } ;
unary       = ( "-" | "!" ) unary | primary ;
primary     = number | array | inv | sel | if_expr
            | call | index | ident | "(" expr ")" ;

array       = "[" [ expr { "," expr } ] "]" ;
inv         = "inv" "(" expr ")" ;
sel         = "sel" "(" expr "," expr "," expr ")" ;
if_expr     = "if" expr "{" expr "}" "else" "{" expr "}" ;
call        = ident "(" [ expr { "," expr } ] ")" ;
index       = ident "[" expr "]" ;
```

Equality does not chain: a comparison yields a bit, and comparing that bit to a third
value is almost never meant, so it is a syntax error rather than a silent surprise.

## 4. Declarations

- **`input`** and **`secret`** declare a scalar read from the run's inputs in
  declaration order. A public input enters the proven statement; a secret is a private
  witness that feeds the run without being revealed.
- **`let`** binds an expression to a name. A later `let` of the same name shadows the
  earlier binding; there is no mutation, only rebinding, which keeps the trace linear.
- **`const`** binds either a scalar, `const N = 5;`, read by name, or a table,
  `const T = [1, 2, 3];`, read by a constant index. The bracket after `=` selects which.
- **`fn`** defines a function as a single expression. A call is inlined at compile time
  with its arguments substituted, so functions cost nothing beyond the arithmetic they
  name and cannot recurse.
- **`include`** textually resolves another source file once, so a program can draw on a
  standard library. Includes are resolved to a bounded depth.

## 5. Statements and control

- **`output`** publishes an expression as a public output.
- **`assert`** constrains an expression to be zero. `assert a == b;` and `assert a != b;`
  are the equality forms. An assertion that does not hold makes the trace unprovable, so
  a program with a false assertion has no proof.
- **`for i in a .. b`** is a counted loop over a constant range, unrolled at compile
  time. The bound expressions must fold to constants, and the loop variable `i` is a
  compile-time constant inside the body, usable in arithmetic and as an index. There is
  no runtime loop, so the trace length is fixed before the program runs.

## 6. Expressions

Operators, tightest binding last:

| Level | Operators | Meaning |
|---|---|---|
| or | `\|\|` | `a + b - a*b`, exact on bits |
| and | `&&` | `a * b`, exact on bits |
| comparison | `== != < <= > >=` | a bit, one when the relation holds |
| sum | `+ -` | field add and subtract |
| product | `* /` | field multiply, and multiply by an inverse |
| unary | `- !` | negate, and `1 - x` (logical not) |

`inv(x)` is the field inverse; inverting zero has no witness, so it makes the trace
unprovable, and `/` is multiplication by an inverse with the same rule. `sel(c, a, b)`
is a branchless select returning `a` when `c` is one and `b` when `c` is zero, with `c`
constrained boolean. `if c { a } else { b }` is the same select in a familiar shape,
and both arms are evaluated. An array literal is a vector; `name[i]` reads a constant
table or an array at a constant index, which must be in bounds.

## 7. Compilation and the machine

A program compiles to a list of instructions over a register machine with thirty two
registers. Register allocation reuses registers whose values are dead, so a long
straight-line program or an unrolled loop is fine as long as the number of values live
at once fits the file. Exceeding it is the `TooManyRegisters` error, an honest ceiling:
it forces a circuit to be budgeted, and a bounded circuit is the only kind with a fixed
proof.

## 8. The proof

Running a program yields an execution trace. The step AIR binds every operand of every
instruction to the live register file, so register reuse is invisible to soundness and a
forged row cannot pass. The public statement, the program commitment, the trace length,
and the public inputs and outputs are bound into the proof. A per-program verifier key,
`keccak256` over the wiring version, the commitment, the log trace length, the trace
width, the rate, and the periodic root, ties a proof to an exact program, which is what
lets a market register and challenge a program by its key alone. The underlying STARK is
transparent, over the quadratic extension, with no trusted setup.

## 9. Ordered comparison

Equality is a field primitive. Ordered comparison, `a < b` and its relatives, is not:
deciding an order needs the operands' bits, which field arithmetic cannot recover. The
operators `< <= > >=` are first class and supply the witness themselves. `a < b` range
proves both operands to sixteen bits, forms `a + 2^16 - b`, decomposes it into seventeen
bits, and returns the complement of the top bit, which is the sign of the difference.
`a > b` is `b < a`, and the inclusive forms are the negations. The bit decompositions
are advice: the compiler records, per comparison, which value is decomposed, and the
driver evaluates the program, reads those values, fills the bits, and then proves with
every constraint enforced. Soundness rests on the range proofs and the composition
constraints, not on how the bits were produced, so a false order or an operand outside
the sixteen-bit range has no proof. Operands must lie in `[0, 2^16)`.
