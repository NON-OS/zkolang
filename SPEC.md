<!-- NONOS. AGPL-3.0-or-later. -->

# The zKølang language reference

Edition 2026. This document is normative. Where it says *must*, *must not*, *is an
error* or *fails*, a conforming compiler behaves exactly so; paragraphs marked *Note*
explain and are not normative. The language of the previous edition, 2025, is specified
in [`docs/edition-2025.md`](docs/edition-2025.md) and remains supported, frozen, for the
circuits registered under it (section 20).

zKølang is a language for statements proven by a STARK. A program reads public inputs
and a private witness, computes over the Goldilocks field and fixed-width integers, and
returns public outputs. Running a program either *accepts*, producing its outputs, or
*fails*. A proof exists for a public input and a claimed output exactly when some
witness makes the program accept with that output. Everything in the language is chosen
so that this correspondence can be held exactly by the compiler and so that the cost of
a program, the rows of its trace, is a static property of its text.

## Status

This edition is specified ahead of its implementation. This section, which is not
normative, is the only one that describes the repository rather than the language.

- The compiler that ships, `compile_source` in `nonos_zkolang` and the `zkolang`
  command line, compiles edition 2025.
- Of edition 2026, the front end exists, in `nonos_zkolang/src/compiler`: the lexer, the
  parser with its error recovery, the placement check and the diagnostics. So do the
  checker of sections 4 to 13 and the reference interpreter of the typed IR, for a
  program with structs, enums, `match`, `impl` blocks, methods and `Self`
  included, and generic structs, enums, aliases, functions, methods and `impl` blocks,
  with type and constant parameters, and constant `if` conditions.
  A crate of several files is loaded from its root file, each `mod name;` from its own
  file (section 4.2). A program that a manifest governs is loaded with the path
  dependencies of its package and theirs, each a crate that its dependents name by the
  key their manifests give it (section 4.1). The cost warnings W0100 and W0101 (section
  15.3) are given at the thresholds the manifest's `[cost]` sets, and W0102 when a program
  builds. The command line takes a
  file's edition from `--edition`, else from the manifest that governs it, and otherwise
  compiles edition 2025, where section 4.1 says 2026. The
  standard library, written in zKølang under `std/` and built into the compiler, is
  loaded beside every crate as the crate `std`; so far it holds `std::array`,
  `std::cmp`, `std::curve` (short Weierstrass curves over `field`), `std::hash` (the
  MiMC permutation and compressions of the edition 2025 library), `std::merkle`,
  `std::option`, `std::poly`, `std::result` and the prelude, `std::prelude` (section
  18.1). The
  back end compiles the typed IR of such a
  program to the machine: lowering to SSA, the passes, gadget expansion, scheduling, register allocation
  and a check of the machine program against the SSA. `compiler::driver::build` and
  `prove` build a program and prove a run of it with the STARK, hiding the witness;
  `zkolang check` and `zkolang run` call them with `--edition 2026`. The rest of the compiler is being built in the stages
  `docs/compiler-architecture.md` describes.
- `zkolang test` runs the tests of an edition 2026 crate (section 16), each compiled and
  run on the machine beside the reference interpreter; `zkolang explain CODE` prints a
  code's description (section 19); `zkolang check --json` prints an edition 2026
  program's diagnostics as one JSON array; `zkolang doc` renders a crate's reference from
  its doc comments (section 17.2), and `docs/stdlib.md` is the standard library's, which a
  test keeps equal to what `zkolang doc --std` makes; `zkolang abi` prints the layout of
  `main`'s inputs and result (section 12.2), and `zkolang check --declassify` lists each
  `declassify` (section 13.2); `zkolang check --cost` gives the cost report of section
  15.2; `docs/migration-2026.md` is the migration guide of section 20.3, each of its
  pairs checked by a test to give the same outputs. The constraint ledger does not exist
  yet.

## Contents

1. Notation and conformance
2. Lexical structure
3. Grammar
4. Packages, modules and names
5. Types
6. Values and their representation
7. Expressions
8. Statements, blocks and control flow
9. Patterns
10. Functions, methods and generics
11. Constants and compile-time evaluation
12. Programs, inputs and outputs
13. Secret flow
14. Failure
15. Cost
16. Tests
17. Attributes and doc comments
18. The standard library and the prelude
19. Diagnostics
20. Editions
21. The target machine (informative)

---

## 1. Notation and conformance

The grammar is written in EBNF: `=` defines, `|` separates alternatives, `[ x ]` is
optional, `{ x }` repeats zero or more times, `( ... )` groups, and quoted text is a
terminal. `p` denotes the Goldilocks prime `2^64 - 2^32 + 1`.

A *compile-time* property is one the compiler decides from the program text and the
values of constants. A *run* is one evaluation of a program on concrete inputs. The
*reference semantics* is the evaluation this document defines; the compiler's output
must agree with it on every run (section 14.3).

A conforming compiler rejects every program this document calls an error, with a
diagnostic, and never aborts, panics or fails to terminate on any input text.

A compiler may bound how deeply the source nests, so that every stage walks a tree of
bounded depth. Nesting is a bracket, block or argument list inside another, a prefix
operator or cast applied to an operand, a field, index or call applied to a receiver,
and an operator of one precedence taking an operand built with an operator of another.
A run of operators of one precedence, such as a sum of any number of terms, and an
`else if` chain of any length are not nesting. The reference compiler's bound is 64
levels; source nested deeper is an error (E0102).

## 2. Lexical structure

Source text is UTF-8. Outside comments and string literals only ASCII is permitted; any
other character is an error. A byte-order mark at the start of a file is ignored.

### 2.1 Whitespace and comments

Whitespace is space, tab, carriage return and line feed. It separates tokens and is
otherwise insignificant. A line ends at a line feed, a carriage return, or the two
together.

```
line_comment  = "//" { any character except line feed and carriage return } ;
block_comment = "/*" { block_comment | any character } "*/" ;
```

Block comments nest. An unterminated block comment is an error. A comment that starts
with exactly `/**` or `///` is an *outer doc comment*, and one that starts with `/*!` or
`//!` an *inner doc comment* (section 17.2); `/***`, `/**/` and `////` start ordinary
comments.

### 2.2 Identifiers and keywords

```
ident = ( letter | "_" ) { letter | digit | "_" } ;   (* not a keyword, not "_" alone *)
letter = "A".."Z" | "a".."z" ;
digit  = "0".."9" ;
```

The following are keywords and cannot be identifiers:

```
as        assert    bool      break     const     continue  crate     declassify
else      enum      false     field     fn        for       i8        i16
i32       i64       if        impl      in        let       limit     match
mod       mut       pub       public    return    secret    self      Self
struct    super     true      type      u8        u16       u32       u64
use       usize     while
```

These are reserved for future editions and are also errors as identifiers:
`async await dyn extern include loop macro move ref static trait unsafe where yield`.
The word `include` is additionally recognised at item position to report the edition
2026 replacement for a textual include (section 20.2).

A lone `_` is the wildcard token, used in patterns and in `let _ = e;`.

### 2.3 Literals

```
int_lit    = ( dec_lit | hex_lit | oct_lit | bin_lit ) [ int_suffix ] ;
dec_lit    = digit { digit | "_" } ;
hex_lit    = "0x" hex_digit { hex_digit | "_" } ;
oct_lit    = "0o" oct_digit { oct_digit | "_" } ;
bin_lit    = "0b" bin_digit { bin_digit | "_" } ;
int_suffix = "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "usize" ;
bool_lit   = "true" | "false" ;
str_lit    = '"' { str_char | escape } '"' ;
escape     = "\\" ( '"' | "\\" | "n" | "t" | "0" ) ;
str_char   = any character except '"', "\\", line feed and carriage return ;
```

An integer literal denotes a non-negative mathematical integer; its type is given by its
suffix or inferred (section 5.6). A literal whose value does not fit its type is an
error; in particular a `field` literal must be less than `p` (there is no silent
reduction). A radix prefix is followed directly by a digit. A string literal ends on the
line it starts. String literals appear only in `assert` messages and attributes; there
is no string type.

### 2.4 Punctuation and operators

```
+  -  *  /  %  ^  !  &  |  &&  ||  <<  >>
+= -= *= /= %= ^= &= |= <<= >>=  =  ==  !=  <  >  <=  >=
.  ..  ..=  ,  ;  :  ::  ->  =>  #  (  )  [  ]  {  }
```

The lexer is maximal munch. Where a generic list may close, the first `>` of a `>>`, `>=`
or `>>=` closes it and the rest is read as the next token, so `C<D<u8>>` and
`type A<T>= u8;` parse.

## 3. Grammar

```
(* ----- files and items ----- *)
file          = { inner_attr } { item } ;
inner_attr    = "#!" "[" attr "]" ;
outer_attr    = "#" "[" attr "]" ;
attr          = ident [ "(" [ attr_arg { "," attr_arg } [ "," ] ] ")" | "=" literal ] ;
attr_arg      = attr_name [ "=" literal ] | literal ;
attr_name     = ident | keyword ;    (* a keyword of section 2.2 but `true` and `false` *)
literal       = int_lit | bool_lit | str_lit ;

item          = { outer_attr } [ "pub" ] item_kind ;
item_kind     = fn_item | struct_item | enum_item | type_alias | const_item
              | mod_item | use_item | impl_item ;

fn_item       = [ "const" ] "fn" ident [ generics ] "(" [ params ] ")"
                [ "->" type ] block ;
generics      = "<" generic_param { "," generic_param } [ "," ] ">" ;
generic_param = ident | "const" ident ":" type ;
params        = param { "," param } [ "," ] ;
param         = self_param | pattern ":" type ;
self_param    = [ "&" "mut" ] "self" ;

struct_item   = "struct" ident [ generics ]
                ( "{" [ field_def { "," field_def } [ "," ] ] "}"
                | "(" [ tuple_field { "," tuple_field } [ "," ] ] ")" ";"
                | ";" ) ;
field_def     = { outer_attr } [ "pub" ] ident ":" type ;
tuple_field   = { outer_attr } [ "pub" ] type ;

enum_item     = "enum" ident [ generics ] "{" [ variant { "," variant } [ "," ] ] "}" ;
variant       = { outer_attr } ident
                [ "(" [ type { "," type } [ "," ] ] ")"
                | "{" [ variant_field { "," variant_field } [ "," ] ] "}" ] ;
variant_field = { outer_attr } ident ":" type ;      (* no "pub": section 4.3 *)

type_alias    = "type" ident [ generics ] "=" type ";" ;
const_item    = "const" ident ":" type "=" expr ";" ;
mod_item      = "mod" ident ( ";" | "{" { inner_attr } { item } "}" ) ;
use_item      = "use" use_tree ";" ;
use_tree      = path [ "as" ident ]
              | path "::" "*"
              | path "::" "{" [ use_tree { "," use_tree } [ "," ] ] "}" ;
impl_item     = "impl" [ generics ] type "{" { { outer_attr } [ "pub" ] fn_item } "}" ;

(* ----- types ----- *)
type          = [ "public" | "secret" ] bare_type ;
bare_type     = "field" | "bool" | int_type
              | "(" ")"
              | "(" type "," [ type { "," type } [ "," ] ] ")"
              | "[" type ";" const_arg "]"
              | "&" "mut" type                       (* parameters only *)
              | "Self"
              | path [ generic_args ] ;
int_type      = "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "usize" ;
generic_args  = "<" generic_arg { "," generic_arg } [ "," ] ">" ;
generic_arg   = type | const_arg ;
const_arg     = int_lit | ident | "{" expr "}" ;

path          = path_start { "::" ident } ;
path_start    = ident | "crate" | "super" | "self" | "Self" | int_type | "field" | "bool" ;

(* ----- statements and blocks ----- *)
block         = "{" { stmt } [ expr ] "}" ;
stmt          = ";"
              | let_stmt
              | assert_stmt
              | expr_stmt ;
let_stmt      = "let" pattern [ ":" type ] "=" expr ";" ;
assert_stmt   = "assert" expr [ "," str_lit ] ";" ;
expr_stmt     = block_like_expr [ ";" ]
              | expr ";" ;
block_like_expr = block | if_expr | match_expr | for_expr | while_expr ;

(* ----- expressions, loosest first ----- *)
expr          = assign_expr ;
assign_expr   = range_expr [ assign_op assign_expr ] ;        (* statement position only *)
assign_op     = "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "^=" | "&=" | "|="
              | "<<=" | ">>=" ;
range_expr    = or_expr ;                                     (* ranges: see for_expr *)
or_expr       = and_expr { "||" and_expr } ;
and_expr      = cmp_expr { "&&" cmp_expr } ;
cmp_expr      = bitor_expr [ cmp_op bitor_expr ] ;            (* does not chain *)
cmp_op        = "==" | "!=" | "<" | "<=" | ">" | ">=" ;
bitor_expr    = bitxor_expr { "|" bitxor_expr } ;
bitxor_expr   = bitand_expr { "^" bitand_expr } ;
bitand_expr   = shift_expr { "&" shift_expr } ;
shift_expr    = add_expr { ( "<<" | ">>" ) add_expr } ;
add_expr      = mul_expr { ( "+" | "-" ) mul_expr } ;
mul_expr      = cast_expr { ( "*" | "/" | "%" ) cast_expr } ;
cast_expr     = unary_expr { "as" cast_type } ;
cast_type     = "field" | "bool" | int_type | path ;
unary_expr    = ( "-" | "!" ) unary_expr | "&" "mut" unary_expr | postfix_expr ;
postfix_expr  = primary { call_suffix | index_suffix | field_suffix | method_suffix } ;
call_suffix   = "(" [ expr { "," expr } [ "," ] ] ")" ;
index_suffix  = "[" expr "]" ;
field_suffix  = "." ( ident | dec_lit ) ;
method_suffix = "." ident [ "::" generic_args ] call_suffix ;

primary       = literal
              | path_expr
              | struct_lit
              | "(" expr ")"
              | "(" ")"
              | "(" expr "," [ expr { "," expr } [ "," ] ] ")"
              | "[" [ expr { "," expr } [ "," ] ] "]"
              | "[" expr ";" const_arg "]"
              | block
              | if_expr
              | match_expr
              | for_expr
              | while_expr
              | "return" [ expr ]
              | "break"
              | "continue"
              | "declassify" "(" expr ")" ;
path_expr     = path [ "::" generic_args ] ;
struct_lit    = path [ "::" generic_args ] "{" [ field_init { "," field_init } [ "," ] ] "}" ;
field_init    = ident [ ":" expr ] ;

if_expr       = "if" expr_no_struct block [ "else" ( block | if_expr ) ] ;
match_expr    = "match" expr_no_struct "{" [ arm { "," arm } [ "," ] ] "}" ;
arm           = pattern [ "if" expr ] "=>" ( expr | block ) ;
for_expr      = "for" pattern "in" ( range | expr_no_struct ) block ;
range         = expr_no_struct ( ".." | "..=" ) expr_no_struct ;
while_expr    = "while" expr_no_struct "limit" const_arg block ;

(* ----- patterns ----- *)
pattern       = "_"
              | [ "mut" ] ident
              | literal_pat
              | "(" [ pattern { "," pattern } [ "," ] ] ")"
              | path [ "(" [ pattern { "," pattern } [ "," ] ] ")" ]
              | path "{" [ field_pat { "," field_pat } ] [ "," ".." ] [ "," ] "}"
              | "[" [ pattern { "," pattern } [ "," ] ] "]"
              | pattern "|" pattern ;
literal_pat   = [ "-" ] int_lit | bool_lit
              | [ "-" ] int_lit ( "..=" ) [ "-" ] int_lit ;
field_pat     = ident [ ":" pattern ] ;
```

`expr_no_struct` is `expr` with struct literals disallowed at its top level, so that
`if x == S { .. } { .. }` is not ambiguous; parenthesise a struct literal there. In a
`match`, the comma after an arm may be left out when the arm's body is block-like. An
`expr_stmt` whose expression is block-like may omit its semicolon only when its type is
`()`. The assignment forms, `return`, `break` and `continue` are expressions of type `!`
or `()` (section 8) but may appear only in statement position or as a block's tail;
elsewhere they are an error. A block's value is its tail expression, or `()` without
one.

Precedence is given by the grammar. `a < b < c` is a syntax error, not a chain. `-x as T`
is `(-x) as T`. `!a == b` is `(!a) == b`.

## 4. Packages, modules and names

### 4.1 Packages

A *package* is a directory holding a manifest `zkolang.toml` and sources. The manifest
is a TOML subset:

```toml
[package]
name    = "shield"          # an identifier; `std` and `crate` are reserved
version = "0.3.1"           # semantic version
edition = "2026"            # "2025" or "2026"; required
entry   = "src/main.zkl"    # optional; default src/main.zkl, else src/lib.zkl

[dependencies]
merkle = { path = "../merkle" }   # path dependencies; the key is the crate name

[cost]                            # optional thresholds, section 15.3
unroll_warn   = 1024
dyn_index_warn = 64
```

Unknown keys are an error. Dependencies are resolved by path relative to the manifest.
A dependency cycle is an error. Every dependency must itself declare edition 2026; an
edition 2025 program is compiled by the frozen compiler and cannot be imported.

A single file compiled without a manifest is a package of one file whose edition is
2026, unless a manifest is found in an ancestor directory, in which case that manifest
governs it.

### 4.2 Modules

The entry file is the crate root module. `mod name;` declares a child module whose source
is `name.zkl` beside the declaring file, or `name/mod.zkl`; both existing is an error, as
is neither. For a declaring file other than a crate root or a `mod.zkl`, the child's
files are sought in the directory named after the declaring file's module. `mod name {
... }` declares an inline module. A module is loaded once; declaring the same file twice
is an error.

### 4.3 Items and visibility

An item is visible in the module that declares it and in that module's descendants. An
item marked `pub` is additionally visible wherever its parent module is visible. A
struct field is private to the module declaring the struct unless marked `pub`; building
a struct value with a struct literal requires every field to be visible. Enum variants
and their fields have the enum's visibility.

### 4.4 Paths and resolution

A path names an item. `crate::` starts at the current crate root, `super::` at the parent
module, `self::` at the current module, and a crate name at that crate's root. A path
beginning with any other identifier is resolved against, in order: local variables (for
a single-segment path in expression position), generic parameters, items declared in
the current module, names imported by `use` in the current module, dependency crate
names, `std`, and the prelude (section 18.1). An ambiguous glob import that is used is
an error.

Each module has one namespace for items: two items of the same name in one module are
an error, as is an import that collides with an item. Associated functions and methods
of a type are found through that type (`Type::name`, `value.name(...)`).

`use` imports a name into the current module. Imports are not re-exported unless marked
`pub use`. Import cycles among globs are resolved to a fixed point; a name that cannot
be resolved is an error that names the closest candidates. They are taken from the names
usable where it stands: items, imports, crates and the prelude, and for a first name also
the generic parameters, the locals where a variable is read, and the primitive types where
a type stands. A candidate is at most a third of the name's length of edits away, and at
least one, where an edit inserts, removes or changes a letter or swaps two neighbouring
ones. A name that differs only in case is the closest; at most three are named.

## 5. Types

### 5.1 Primitive types

| Type | Values | Notes |
|---|---|---|
| `field` | the integers modulo `p` | arithmetic is modular |
| `bool` | `false`, `true` | not a number |
| `u8 u16 u32 u64` | `0 ..= 2^N - 1` | `N` is the width |
| `i8 i16 i32 i64` | `-2^(N-1) ..= 2^(N-1) - 1` | two's complement semantics |
| `usize` | `0 ..= 2^32 - 1` | lengths, indices and loop counters |
| `()` | the unit value | no representation |

### 5.2 Compound types

- `(T1, ..., Tn)`, a tuple; `(T,)` is a one-tuple.
- `[T; N]`, an array of exactly `N` elements, `N` a constant of type `usize`.
- `struct` types, named records or tuple structs, possibly generic.
- `enum` types: a set of variants, each with no data, a tuple of fields or named fields.
- `Self`, inside an `impl` block, is the implemented type.

### 5.3 Type aliases

`type Hash = field;` names an existing type. An alias is transparent: `Hash` and `field`
are the same type.

### 5.4 Qualifiers: `public` and `secret`

Every type may be qualified `public T` or `secret T`. The qualifier is a *label* the
secret-flow check tracks (section 13); it does not change representation, layout or cost.
An unqualified type in a function signature is *label-polymorphic*. The qualifiers are
not allowed inside a generic argument list or an array element type of a `main`
parameter; a whole parameter is labelled at once.

### 5.5 Generics

A function, struct, enum, alias or `impl` block may take type parameters and constant
parameters of type `usize`. Generic items are *templates*: they are type-checked once
per distinct instantiation, after substitution. An instantiation is created by a use
with concrete arguments, written or inferred. A type error in an instantiation is
reported at the instantiating use, with a note at the offending line of the template.
In a type, a generic item is given every argument. In a body, a struct literal, a variant
or a pattern that names a generic struct or enum without arguments has its type
arguments inferred from what builds or meets it; a constant argument is written there. A
call of a generic function infers its constant arguments too (10.5). A type or constant
argument nothing settles is an error (E0303).

### 5.6 Integer literal inference

An unsuffixed integer literal takes its type from context: the expected type in a checked
position, else the type it is unified with by the operators and calls it flows into,
else a default. The default is `usize` for an array length, a repeat count, an index,
a `for` range bound and a `limit`, and `field` everywhere else. An `as` gives its operand
no type, except that a literal type nothing else fixes, converted with `as` to an integer
type, takes that type. The conversion is checked against the type the operand ends with.
A literal whose value does not fit the type so determined is an error.

### 5.7 No implicit conversion

There is no implicit conversion between any two distinct types, including between integer
types of different width, between `bool` and any number, and between a type and its
qualified form beyond the label rules of section 13. Conversions are explicit (7.8).

## 6. Values and their representation

*Note: this section fixes how values live in the machine, because cost and the soundness
argument depend on it. It is normative for the compiler and for the ABI (section 12).*

Every value is a fixed-length vector of field elements, its *slots*:

| Type | Slots | Invariant every slot satisfies |
|---|---|---|
| `field` | 1 | none |
| `bool` | 1 | the slot is `0` or `1` |
| `uN`, `usize`, N ≤ 32 | 1 | the slot is in `[0, 2^N)` |
| `iN`, N ≤ 32 | 1 | the slot is `x mod p` for the value `x` |
| `u64` | 2 | `lo, hi`, each in `[0, 2^32)`; value `lo + 2^32 hi` |
| `i64` | 2 | the `u64` slots of the value's two's complement bit pattern |
| `(T1..Tn)`, struct | sum of fields | each field's invariant, in declaration order |
| `[T; N]` | `N * slots(T)` | each element's invariant |
| enum | `1 + max payload slots` | tag in `[0, variants)`; the active variant's payload holds its invariant; every other payload slot is `0` |
| `()` | 0 | |

The compiler maintains these invariants for every value on every path that is not
guarded off (section 8.4): every operation producing a value establishes its invariant,
by construction or by a constraint.

## 7. Expressions

Evaluation order is left to right: operands before the operator, arguments before the
call, the receiver before the arguments. Because the only effect is failure, the order
never changes a run's outcome; it is fixed so that the reference interpreter is
deterministic.

### 7.1 Field arithmetic

For `field` operands: `+`, `-`, `*` are modular; unary `-x` is `p - x` (and `0` for `0`);
`a / b` is `a * b^(p-2)` and fails when `b = 0`. `%`, the ordering comparisons and the
bitwise operators are not defined on `field` (an error). `a.inv()` is `1 / a`.
`a.pow(k)` raises to a constant exponent `k: u64`.

### 7.2 Integer arithmetic

For operands of one integer type `T` with width `N`, the arithmetic operators are
*checked*: the result is the mathematical result, and the run fails if it is not a value
of `T`.

- `a + b`, `a - b`, `a * b`: fail on overflow.
- `a / b`: the quotient truncated toward zero; fails when `b = 0` and, for signed `T`,
  when `a = MIN` and `b = -1`.
- `a % b`: the remainder `a - b * (a / b)`, with the sign of `a`; fails when `b = 0` and
  for signed `MIN % -1`.
- `-a`: signed types only; fails for `MIN`. Unary minus on an unsigned type is an error.

The *wrapping* methods compute modulo `2^N` (two's complement for signed types) and never
fail: `wrapping_add`, `wrapping_sub`, `wrapping_mul`, `wrapping_neg`. For division,
`checked` behaviour is the only one; there is no wrapping division.

### 7.3 Comparison

`==` and `!=` are defined on every type and compare values structurally (enum values
compare by variant and payload). `<`, `<=`, `>`, `>=` are defined on integer types and
`bool` (`false < true`) and compare mathematical values. Both operands must have the same
type. The result is `bool`.

### 7.4 Boolean operators

`!a`, `a & b`, `a | b`, `a ^ b` on `bool` are negation, and, or, exclusive or, all
operands evaluated. `a && b` and `a || b` *short-circuit*: the right operand is evaluated
only when the left does not already decide the result, and a failure in an unevaluated
right operand does not fail the run.

### 7.5 Bitwise operators and shifts

On integer types, `!a` is bitwise complement, `&`, `|`, `^` are bitwise on the two's
complement bit patterns. `a << k` and `a >> k` take `k: u32`; `<<` shifts in zeros and
discards bits shifted past the width, `>>` is logical for unsigned and arithmetic for
signed types. The run fails if `k >= N`; when `k` is a constant this is a compile-time
error instead.

### 7.6 Indexing

`a[i]` on an array reads element `i`. `i` has type `usize`. If `i` is a compile-time
constant it must be in bounds, else it is an error. If `i` is a runtime value the read
is *dynamic*: the run fails when `i >= N`. A dynamic index costs work proportional to
`N` (section 15). `a.len()` is the constant `N`.

### 7.7 Fields and tuples

`s.f` reads a named field, `t.0` a tuple or tuple-struct field. The field must be
visible (4.3).

### 7.8 Conversions

`e as T` is permitted only when every value of the source type is a value of `T`:

- `T as T`;
- `bool as` any integer type or `field` (`false` is 0, `true` is 1);
- an unsigned type as a wider unsigned or a wider signed type; `u8`, `u16`, `u32` as
  `usize` and `usize as` `u32` or `u64`;
- a signed type as a wider signed type;
- any integer type as `field`, mapping `x` to `x mod p`.

Every other `as` is an error that names the explicit forms:

- `T::checked_from(e)` for an integer type `T` and an integer or `field` source: the value
  of `e` as `T`, failing when it is not a value of `T`. A `field` source is read as the
  integer in `[0, p)`.
- `T::wrapping_from(e)` for an integer type `T` and an integer source: the value
  of `e` modulo `2^N`, reinterpreted in two's complement for signed `T`. For a `field`
  source the canonical integer in `[0, p)` is reduced.
- `bool::checked_from(e)` for an integer or `field` source: fails unless `e` is 0 or 1.

### 7.9 Bits

`x.to_le_bits()` on an integer type of width `N` returns `[bool; N]`, the two's
complement bit pattern least significant first. On `field` it returns `[bool; 64]`, the
canonical representative's bits: the decomposition is unique. `T::from_le_bits(b)` is the
inverse for integer types and for `field`, where it fails if the 64-bit value is not less
than `p`.

### 7.10 Calls

`f(args)`, `Type::f(args)`, `value.m(args)` call a function (section 10). Arguments are
evaluated left to right. An argument for a `&mut T` parameter is written `&mut place`.

### 7.11 Struct, tuple and array expressions

`S { f: e, .. }` builds a struct; every field must be given exactly once; `S { f }` is
shorthand for `S { f: f }`. `(a, b)` builds a tuple, `[a, b, c]` an array, and `[e; N]` an
array of `N` copies of `e`, where `e` is evaluated once.

A variant of an enum `E` is built through its enum's path, in the form it is declared in:
`E::V` for a unit variant, `E::V(a, b)` for a tuple variant and `E::V { f: e }` for one
with named fields, the fields given as for a struct. Inside an `impl` block of `E`,
`Self::V` names the same variant. Generic arguments written after the path, `E::V::<A>`,
are the enum's. A variant's fields are visible wherever its enum is. A
lone name names a variant only of an enum the prelude exports (section 18.1): `Some(e)`
and `None` where no item of that name is in scope.

### 7.12 `declassify`

`declassify(e)` evaluates `e` and gives it the label `public` (section 13). It has no
runtime effect.

## 8. Statements, blocks and control flow

### 8.1 Bindings

`let pattern = e;` evaluates `e` and binds the pattern's names. The pattern must be
irrefutable (9.2). `let mut x = e;` binds a mutable variable. A later `let` of a name
shadows the earlier binding for the rest of the block. A type annotation checks `e`
against it.

### 8.2 Assignment

A *place* is a mutable variable, or a field, tuple field or indexed element of a place.
`place = e;` replaces the value of the place; `place op= e;` is `place = place op e;`
with the place evaluated once. Assigning to an element through a dynamic index writes
that element and leaves the others unchanged, and fails when the index is out of bounds.
Assigning to anything but a place rooted in a mutable variable or a `&mut` parameter is an
error.

### 8.3 `assert`

`assert e;` requires `e: bool` and fails the run when `e` is `false`. `assert e, "msg";`
attaches a message the tools report when a run fails there. (In edition 2025, `assert e`
meant `e == 0`; section 20.3.) An `assert` whose condition is the literal `false` fails
wherever it is reached, so it leaves its block as `return` does (section 8.8): what
follows it in the block is unreachable, and a block without a tail that it leaves has
type `!`. This is how `Option::unwrap` fails on `None`.

### 8.4 `if` and guards

`if c { A } else { B }` requires `c: bool`. It evaluates `c`, then exactly one of `A` and
`B`. Its value is the value of the evaluated block; without `else`, `B` is `()` and `A`
must have type `()`. Both blocks must have the same type.

The *guard* of a point in a program is the conjunction of the conditions of the `if`
arms, `match` arms, short-circuit operands, loop continuations and live-return flags
enclosing it. A failure condition (section 14) at a point only fails the run when that
point's guard is true on the run. *Note: the compiler evaluates both arms and merges
their results with the machine's select; this rule is what makes that correct.
Section 21.4 describes the lowering.*

If the condition of an `if` is a constant expression (section 11), it is evaluated where
the `if` is checked, separately in each instance of a generic function, and if the
evaluation does not fail, only the block it selects is type-checked and compiled: a false condition's block is left out, and
a true condition's block ends the `if`, the branches after it left out. A block left out
is not checked, and a local that a name in it could read counts as read (W0001). The `if`
keeps the rule of its written form: without `else`, the block it takes must have type
`()`. The code after the `if` is checked as always, so a recursion is written in the
other branch's block, not after an `if` that returns. A condition whose evaluation
fails is an ordinary condition, left to the run, where it fails only if reached (section
14). This is how generic recursion terminates (10.6).

### 8.5 `match`

`match e { p1 => a1, ..., pn => an }` evaluates `e`, then the first arm whose pattern
matches and whose guard (`if cond`) is true, and yields its value. The arms must be
exhaustive for the scrutinee's type (9.3). All arm values have the same type.

### 8.6 `for`

`for x in a..b { body }` and `for x in a..=b { body }` iterate `x` over the integers of
the range in increasing order; `a` and `b` must be constant expressions of one integer
type, and `x` has that type. An empty range iterates zero times. `for x in arr { body }`
iterates over the elements of an array value in order; `for (i, x) in arr.enumerate()` also
binds the index, a `usize`. The loop is evaluated by repeating `body`; its value is `()`.

### 8.7 `while ... limit`

`while c limit N { body }` evaluates `c`, and while it is `true` evaluates `body` and
repeats, at most `N` times, `N` a constant `usize`. If `c` is still `true` after `N`
iterations, the run fails. *Note: a `while` is compiled as `N` guarded copies of its body.*

### 8.8 `break`, `continue`, `return`

`break` ends the innermost enclosing loop; `continue` ends its current iteration.
`return e` ends the current function with value `e` (or `()`). Code after one of them in
the same block is unreachable, a warning. Failures after a taken `break`, `continue` or
`return` are guarded off (8.4).

### 8.9 Blocks

A block evaluates its statements in order, then its tail expression. Names bound in a
block are not visible after it. A block is an expression.

## 9. Patterns

### 9.1 Forms

A pattern is a wildcard `_`, a binding `x` or `mut x`, a literal (`bool`, integer and
`field` scrutinees), an inclusive literal range `lo..=hi` with `lo <= hi` (integer
scrutinees), a tuple pattern, an array pattern of exactly the array's length, a struct or
tuple-struct pattern (with `..` to ignore the remaining fields), an enum variant pattern
written with its enum's path (`E::V`, `E::V(p, q)`, `E::V { f: p, .. }`, or `Self::V`) or,
for an enum the prelude exports, with the variant's name alone (`Some(p)`, `None`), or
an alternation `p | q` whose alternatives bind the same names at the same types and
mutability. A pattern binds each name at most once. A lone name in a pattern binds,
unless it is the name of a unit variant of an enum the prelude exports and no item of
that name is in scope; then it is that variant, as `None` is.

### 9.2 Irrefutability

`let` and function parameters take *irrefutable* patterns: wildcards, bindings, and tuple,
array and struct patterns of irrefutable patterns, and variant patterns of an enum of one
variant. Anything else there is an error.

### 9.3 Exhaustiveness

A `match` is exhaustive when every value of the scrutinee's type matches some arm without a
guard. The compiler decides this exactly for every type: `bool`, enums, tuples, structs
and arrays by their parts, and integers and `field` by the values their literals and
ranges cover, so an integer scrutinee needs a wildcard or binding arm unless its ranges
cover the whole type. A non-exhaustive `match` is an error (E0400) naming a pattern that
some value no arm covers matches, written as an arm could write it: a generic struct or
enum is named without its arguments. An arm that no value reaches, because the arms without
a guard before it cover every value it matches, is a warning (W0004). *Note: the check of
one `match` takes at most 200 000 steps; one that needs more is reported as E0400, and
splitting it into nested `match`es brings it under the budget.*

## 10. Functions, methods and generics

### 10.1 Functions

`fn f(x: T, ...) -> R { body }` declares a function; without `-> R` the result is `()`.
Parameter and result types are always written; locals are inferred. The body's value is
the result, or the value of a `return`.

### 10.2 Methods and associated functions

`impl T { ... }` attaches functions to `T`, which must be a struct or enum declared in
the same crate. A function whose first parameter is `self` or `&mut self` is a method,
called `value.m(args)`; others are associated functions, called `T::f(args)`. At most one
`impl` item may define a given name for a type.

A generic `impl<T> G<T> { ... }` is for a struct or enum `G`; its type may place its
parameters anywhere in `G`'s arguments, as in `impl<T> G<(T, u8)>`. A function of it is
found through a type when the type matches the block's type, which gives the block's
parameters their arguments; through `G` named without arguments, `G::f(..)`, they are
inferred. A function of an `impl` block for one instance, such as `impl G<u8>`, and a
function of a generic block for the same struct or enum may not share a name.

### 10.3 `&mut` parameters

A parameter of type `&mut T` receives a place. The call behaves as if the callee read the
place's value on entry and wrote the parameter's final value back on return. Two `&mut`
arguments of one call must not overlap (the same variable, or one a sub-place of the
other), an error when the compiler cannot prove they are disjoint. `&mut T` appears only
as a parameter type; there are no other references.

### 10.4 Calls are inlined

Every call is expanded at its site. *Note: there is no call stack on the machine; the cost
of a call is the cost of its body.*

### 10.5 Generic functions

A generic function is instantiated per distinct list of type and constant arguments.
Arguments may be written with a turbofish, `f::<u8, 4>(x)`, or inferred from the argument
and expected result types. A constant parameter is inferred from the arguments: where a
parameter's type names it as an array length or as a constant argument of a struct or
enum, the argument's type gives its value, the first argument that gives one deciding it,
and an argument whose type then disagrees is a mismatch (E0300). A function of an `impl`
block, called as a method or through its type, infers its own constant parameters the
same way, its block's arguments given by the type. In a body, a constant
parameter is a `usize` value. An instantiation that cannot be inferred is an error (E0303);
so is an instance whose body does not check (E0702, at the call that makes it, beside the
errors in the body). The instances of one function nest at most 64 deep along the calls
that make them, and an instance's type arguments take at most 1024 parts written out;
a call past either is an error (E0700).

### 10.6 Recursion

A function may call itself, directly or through others, only if every chain of calls the
compiler expands terminates within a depth of 64 nested calls of any one function. A
recursion that does not is an error naming the cycle. *Note: recursion terminates when its
controlling condition is a constant expression, typically on a constant generic parameter,
because constant `if` conditions select one arm at compile time (8.4). A recursion driven
by a runtime value never terminates at compile time and is rejected.*

## 11. Constants and compile-time evaluation

`const NAME: T = e;` declares a constant. `e` must be a *constant expression*: a literal;
a constant; a constant generic parameter; a loop variable of a `for` loop inside a `const
fn`; an operator, conversion, array, tuple, struct or enum expression over constant
expressions; `a.len()`; an `if` or `match` over constant expressions; or a call of a
`const fn` with constant arguments.

A `const fn` is a function whose body uses no `declassify`, no `secret` or `public`
qualified type, and calls only `const fn`s. It may be called at runtime too.

Constants are evaluated by the reference semantics. A failure during evaluation (an
overflow, a failed assertion, a division by zero, an out-of-bounds index) is a compile
error pointing at the failing operation. Evaluation is bounded by a step budget of 10^7
operations, beyond which it is an error.

Constant cycles are an error.

## 12. Programs, inputs and outputs

### 12.1 `main`

A program is a package whose root module declares `fn main`. Every parameter of `main`
has a type qualified `public` or `secret`; `main` is not generic. The result of `main` is
the program's public output; its label must be `public` (section 13).

### 12.2 The ABI

The *public input vector* is the concatenation, in parameter order, of the slots (section
6) of every `public` parameter; the *secret input vector* likewise for the `secret`
parameters; the *output vector* is the slots of the result. A tool that accepts typed
inputs encodes them by this layout, taking one value per scalar and an enum slot by slot:
its tag, then its payload as laid out. Slots that lay out no value of an enum are
refused. `zkolang abi` prints the layout.

### 12.3 Inputs are checked

On entry, `main` checks every slot of every parameter against its type's invariant
(section 6): the run fails if a slot violates it. *Note: this is what makes a `u8`
parameter trustworthy inside the program whoever supplied it.*

## 13. Secret flow

### 13.1 Labels

Every value has a label, `public` or `secret`, ordered `public < secret`. The label of a
`main` parameter is its qualifier. Literals and constants are `public`. The label of every
other value is the least upper bound of the labels of the values it is computed from, and
of the labels of the guards (8.4) that decide it: the conditions of an `if` its value
comes from, the guard under which a variable is assigned, the guards of the `return`s
that may give a function's result or a `&mut` parameter's final value, and, for a
variable a loop changes, the guards of the loop's `break`, `continue` and `return` and of
its `while` condition. *Note: this is what catches a secret that leaks through a branch:
`if s { 1 } else { 0 }` is secret.*

A variable or parameter declared with a qualified type keeps its qualifier: every value
assigned to a `public` part must be `public`, and a `secret` part stays `secret`.

A function is checked once. The label of each part of an unqualified parameter is the
label of that part of the argument (section 5.4): a tuple's or struct's fields have their
own labels, an enum's tag and each field of each of its variants have theirs, and an
array's elements share one. A variant built where a guard does not decide it has a
`public` tag. Which arm of a `match` runs depends on the parts of the scrutinee that its
pattern and the patterns before it test (the tag of each variant they name, and each part
a literal or range compares; a binding or `_` tests nothing) and on the guards before
it; those labels are the `match`'s guard labels, as the condition's are an `if`'s.
*Note: the compiler follows the labels of the first 128 scalar parts of a function's
parameters apart; it takes the parts after them as `secret`, which can only add errors.*

### 13.2 Checked positions

It is an error for a value labelled `secret` to reach:

- the result of `main`;
- a binding, parameter, struct field or result whose type is written `public T`.

`declassify(e)` is the only way to give a secret-derived value the label `public`. Every
`declassify` in a program is listed by `zkolang check --declassify`.

A `secret T` annotation raises the label of the value it receives to `secret`.

### 13.3 What secret flow does not claim

A run's success is public by definition: `assert s == 3;` tells a verifier that the
witness satisfies `s == 3`. The trace's shape never depends on a secret because it never
depends on any value (section 15). The proof system's zero knowledge is a property of the
prover, not the language (see `docs/threat-model.md`).

## 14. Failure

### 14.1 Failure conditions

A run fails if, at a point whose guard is true, one of the following occurs: an `assert`
of `false`; a checked arithmetic overflow; a division or remainder by zero; the inverse of
a zero `field`; a dynamic index out of bounds; a shift by at least the width; a failing
`checked_from`; `from_le_bits` of a non-canonical `field`; a `while` that exceeds its
limit; an input slot that violates its type's invariant.

### 14.2 Outcome

A run on public inputs `x` and secret inputs `w` either *accepts* with output `y`,
written `run(x, w) = accept(y)`, or *fails*.

### 14.3 The compilation contract

For a program `P` compiled to the machine program `C`, for every `x` and `y`:

> there exist a secret input `w` and advice `a` such that `C` executes on `(x, w, a)`
> satisfying every constraint and exposing `y`, **if and only if** there exists `w` with
> `run(x, w) = accept(y)`.

The *if* direction is completeness; the compiler's witness generator supplies `a`. The
*only if* direction is soundness: no choice of advice makes the machine accept a statement
the reference semantics rejects. *Note: this contract is what the differential tests, the
constraint ledger (`docs/audit/constraints.md`) and the Lean development each check a
piece of.*

## 15. Cost

### 15.1 Rows

The cost of a program is the number of machine instructions its compilation emits, each
one trace row, including the final `Halt`. It is a function of the program text alone.
A program must compile to at most `2^16` rows; a larger one is an error. A program shorter
than 33 rows is padded to 33, the fewest in which the prover hides the witness.

### 15.2 Reporting

`zkolang check --cost` reports, per function, the rows attributed to it inclusive of the
functions inlined into it and exclusive of them, the peak number of live registers, and
the source lines that emit the most rows. A row is attributed to the innermost expression
whose instruction it is, in the function whose code that expression is, and to each
function inlined to reach it; a row that brings a value back into a register, to the row
after it that needs the value. Reading and checking the inputs, padding and the final
`Halt` are no function's. The live registers at a row are those it or a later row reads
before any row writes them again; a function's peak is the most at a row of its own.

### 15.3 Warnings

The compiler warns when a single loop unrolls to more than `unroll_warn` iterations
(default 1024), when a dynamic index or dynamic assignment is over an array longer than
`dyn_index_warn` elements (default 64), and when a function's inclusive rows exceed half
the row limit. `#[allow(cost)]` on a function silences these for its body. The thresholds
may be set in the manifest's `[cost]` table. The warnings are given for the code of the
program's own crate, not of its dependencies or `std`.

### 15.4 Registers

The machine has 32 registers and no memory. A row that reads a secret input or an advice
value is pinned to no value by the proof, so two reads of one such slot could differ: the
compiler reads each once and holds it in a register until its last use. A constant is
written again and a public input read again when needed. The compiler orders the
program's instructions to keep few values held at once. When the values a program needs
at one point still cannot fit, compilation fails with E0801. The compiler never emits a
program that is wrong because of register pressure.

## 16. Tests

A function marked `#[test]` takes no parameters and returns `()`. `zkolang test` compiles
and runs each test; a test passes when its run accepts. `#[should_fail]` inverts that. A
test may call `main` and any other function. Items marked `#[cfg(test)]` are compiled only
when testing.

## 17. Attributes and doc comments

### 17.1 Attributes

| Attribute | On | Meaning |
|---|---|---|
| `#[test]` | fn | a test (section 16) |
| `#[should_fail]` | test fn | the test must fail |
| `#[cfg(test)]` | item | compiled only for tests |
| `#[allow(lint, ...)]` | item, `#!` module | silence warnings: `unused`, `cost`, `unreachable`, `declassify` |
| `#[deprecated = "..."]` | named item | using it is a warning with the message |

An unknown attribute, one where it does not apply, one with the wrong arguments, and one
given twice on an item are errors. A test takes no parameters and returns `()`.

`unused` covers W0001 and W0002, `unreachable` W0003 and W0004, `declassify` W0006, and
`cost` W0100 to W0102. A variable is unused (W0001) when no expression reads its value:
assigning to it is not a read, and passing it as `&mut` or updating it with `op=` is. A
`&mut` parameter counts as read, since the caller reads its final value, and a name that
starts with `_` is never reported.

### 17.2 Doc comments

An outer doc comment, `/** ... */` or `///`, documents the following item, and an inner
one, `/*! ... */` or `//!`, the enclosing module. Doc comments are Markdown. `zkolang
doc` renders them. A doc comment elsewhere is a warning, one for each block of adjacent
doc comments, given only for a file that parses without errors.

## 18. The standard library and the prelude

### 18.1 The prelude

The prelude is the module `std::prelude`. Every module names what it exports without an
import, after the names the module binds itself and after `std` (section 4.4); it
exports `std::option::Option` and `std::result::Result`. The variants of an enum it
exports are named alone, in expressions (section 7.11) and patterns (section 9.1), where
no item of that name is in scope: `Some(x)`, `None`, `Ok(x)` and `Err(e)`.

### 18.2 `std`

The standard library is written in zKølang and shipped with the compiler. Its modules are
documented in [`docs/stdlib.md`](docs/stdlib.md), generated from its doc comments. The
language does not depend on it: every construct of this document is built in.

### 18.3 Built-in methods

Integer types: `wrapping_add wrapping_sub wrapping_mul wrapping_neg to_le_bits pow
min max` and the associated functions `checked_from wrapping_from from_le_bits MIN
MAX BITS`. `field`: `inv pow to_le_bits` and `from_le_bits`. Arrays: `len enumerate`.
`a.len()` is the length of `a`'s array type, a constant; `a` is not evaluated.

## 19. Diagnostics

Every error has a code `E` followed by four digits and every warning a code `W` and four
digits, a message, a primary span with a label, and optionally secondary labels, notes
and one help line. `zkolang explain CODE` prints the long description. Codes are stable
across compiler versions of one edition.

## 20. Editions

### 20.1 Selection

The edition of a package is its manifest's `edition`. Edition 2025 programs are compiled
by the frozen compiler of that edition, specified in `docs/edition-2025.md`, so every
registered circuit keeps its commitment. Edition 2026 is this document.

### 20.2 `include`

Edition 2026 has no textual include. `include "x.zkl";` at item position is an error
whose help line gives the module form: declare `mod x;` and import with `use`.

### 20.3 Migration

`docs/migration-2026.md` maps every edition 2025 form to its edition 2026 form. The two
changes of meaning are: `assert e` asserts a `bool` is true where 2025 asserted a field
is zero; and ordered comparison is defined on typed integers of any width, where 2025
range-checked field operands to sixteen bits.

## 21. The target machine (informative)

### 21.1 Machine

32 registers over the Goldilocks field, and twelve instructions: `Imm Add Sub Mul Inv Sel
Eq Bool Assert Inp Out Halt`. There is no memory, jump or data-dependent control. `Inp`
reads the input vector: the public inputs, then the secret inputs, then the *advice* the
compiler asks the prover to supply for bits, quotients and similar witnesses.

### 21.2 Costs of the primitives

Every instruction is one trace row. Field `+`, `-` and `*` are one instruction each,
plus an `Imm` for a constant operand not already in a register; field `/` is an `Inv`
and a `Mul`. Every operation that must establish an integer invariant (section 6), such
as a range check, a checked integer operation or an ordered comparison, decomposes a value
into advice bits and costs rows in proportion to their number. `zkolang check --cost`
reports the exact figures of a program.

### 21.3 Advice

Every advice value must be determined uniquely by the inputs: the constraints admit
exactly one advice vector for each accepted run.

### 21.4 Guarded lowering

Both arms of an `if` are lowered. A constraint in an arm with guard `g` is lowered so it
holds trivially when `g` is false: an assertion of `e` becomes `g -> e`, a range check of
`x` checks `g ? x : 0`, an inverse of `x` inverts `g ? x : 1`. The arm's results are
merged with `Sel` on the arm's condition. Section 14.3 is the correctness statement.
