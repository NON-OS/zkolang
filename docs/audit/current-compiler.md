# Audit of the edition 2025 compiler

This is an audit of the compiler that ships today, the one in `nonos_zkolang/src/lang`,
the driver's advice fill, the native targets and the command line. It was audited at
commit `97924e5`, the head of `main` when the work began. Locations below are `file:line` at that commit unless a row says
otherwise. The AIR, the prover, the verifier keys and the proof formats are out of scope.

The compiled form of every program in the tree is pinned by the golden table
(`nonos_zkolang_proofs/src/golden_corpus.rs`, added in `19237e8`), because registered
circuits have fixed commitments and verifier keys. Every fix below leaves that table
unchanged, and every fix came with a test that fails without it.

## How the findings were found

Six passes read and ran the compiler, each looking at one part: the front end, the
optimizer, register allocation, the language's meaning, the driver and native targets,
and the standard library and example programs. Each finding came with a program that
reproduces it, and each fixed one was reproduced again as the failing test of its fix.
Fixed findings are listed with the commit and the test that pins them; open ones with
why they stay open. Tests are in `nonos_zkolang_proofs/src/` unless a path says
otherwise.

## Fixed

### Front end

| Finding | Where | Fix | Test |
| --- | --- | --- | --- |
| Nesting and desugaring had no bound, so deep input overflowed the stack and `\|\|` chains grew without limit | `lang/parse/parser/expr.rs:15`, `sum.rs:30`, `product.rs:31` | `296adcd`, `bff77db` | `nesting_tests.rs`, `desugar_budget_tests.rs`, `work_budget_tests.rs` |
| A flat chain of more than about 127 terms was refused as nested too deep (a regression from `296adcd`) | `lang/parse/parser/nesting.rs` after `296adcd` | `bff77db` | `work_budget_tests.rs` |
| Inlining and loops with empty bodies had no work bound; doubling calls or three nested 0..65536 loops ran for hours | `lang/compile/stmt/for_loop.rs:36`, `expr/call.rs:16` | `bff77db` | `work_budget_tests.rs` |
| A comment ran to `\n` only, so with lone carriage returns it swallowed the rest of the program, a false assertion included | `lang/lex/scan.rs:28` | `6891df9` | `line_ending_tests.rs` |
| A byte-order mark at the start of a file was an unexpected character | `lang/lex/scan.rs:22` | `6891df9` | `line_ending_tests.rs` |
| A second `_` arm replaced the first; a missing one was reported after the match | `lang/parse/parser/match_expr.rs:29` | `a4ae8b4` | `match_default_tests.rs` |

### Names and scope

| Finding | Where | Fix | Test |
| --- | --- | --- | --- |
| A constant won over a parameter, `let`, input or block local of its name; the optimizer resolved the other way | `lang/compile/expr/var.rs:19` | `eb9b664`, `441977a` | `name_check_tests.rs`, `lexical_scope_tests.rs` |
| A loop variable won over a `let` of its name in the body, and the `let` was ignored | `lang/compile/expr/var.rs:16` | `eb9b664` | `name_check_tests.rs` |
| The first of two definitions of one function or constant was used without a word | `lang/compile/expr/call.rs:17` | `eb9b664`, `cd01649` | `name_check_tests.rs` |
| Function bodies saw the caller's arrays, ahead of their own scalar parameter of the same name | `lang/compile/expr/args.rs:30` | `aa48154` | `array_scope_tests.rs` |
| A block's scalar local did not hide an outer array, and a loop variable did not either | `lang/compile/expr/block.rs:20`, `expr/index.rs:25` | `aa48154`, `441977a` | `array_scope_tests.rs`, `lexical_scope_tests.rs` |
| Recursion surfaced as too many registers or an unnamed depth error, and not at all while uncalled; a repeated parameter took the last argument | `lang/compile/expr/call.rs:16` | `0cebb62` | `recursion_tests.rs` |

### Optimizer

| Finding | Where | Fix | Test |
| --- | --- | --- | --- |
| Folding `(a == b) + 0` to `a == b` under `assert` turned "the bit is zero" into "a equals b" | `lang/optimize/propagate.rs:135` | `1810de8`, `2ea5370` | `assert_top_tests.rs` |
| Folding dropped an operand that carried a constraint | `lang/optimize/expr.rs:31` | `76515d8` | `fold_constraint_tests.rs` |
| A constant `let` dropped by propagation was never substituted into blocks, which read a stale binding | `lang/optimize/propagate.rs:57` | `84f75d4` | `block_propagation_tests.rs` |
| A `_` slot of a tuple `let` hid an earlier `let _` from substitution | `lang/optimize/propagate.rs:118` | `638ff2f` | `wildcard_env_tests.rs` |
| The optimized build accepted undefined names, arrays and tuples in scalar arithmetic, and an index by a `let`-bound constant, all refused unoptimized | `lang/compile/mod.rs:62`, `lang/optimize/expr.rs:37` | `7615909` | `optimizer_gate_tests.rs` |
| Sharing compared every candidate subtree against the whole statement after every hoist, so a sum of eight thousand terms took about a minute in a debug build | `lang/optimize/cse.rs:59`, `:129` | `9674151` | `cse_scale_tests.rs` |

### Lowering and registers

| Finding | Where | Fix | Test |
| --- | --- | --- | --- |
| A wildcard or rebound destructure slot freed a register a later slot still carried | `lang/compile/stmt/let_tuple.rs:29`, `expr/block.rs:37` | `58f0aad`, `6f48375` | `wildcard_tests.rs`, `tuple_rebind_tests.rs` |
| A register held by several array slots went back to the free list more than once | `lang/compile/stmt/let_scalar.rs:23` and the other `free.push` sites | `cb11ced` | `double_free_tests.rs` |
| A constant index was folded with machine integers unoptimized and in the field optimized | `lang/compile/const_table/eval.rs:18` | `489a4c9` | `const_index_tests.rs` |
| Input, output and advice indices were computed with unchecked `u16` arithmetic | `lang/compile/stmt/input.rs:17`, `output.rs:18`, `expr/decompose.rs:34` | `ebaea68` | `loop_io_tests.rs` |
| A redeclared input kept its old register | `lang/compile/stmt/input.rs:11` | `e06f5e4` | `loop_io_tests.rs` |
| A call returning an element of an array built for it kept that register for the rest of the program | `lang/compile/expr/inline.rs:28` | `8a69448` | `owned_element_tests.rs` |
| The liveness table copied every later name per statement; sixty thousand `let`s took over four minutes | `lang/compile/mod.rs:47` | `7142568` | `liveness_scale_tests.rs` |

### Driver and advice

| Finding | Where | Fix | Test |
| --- | --- | --- | --- |
| A run with fewer or more inputs than declared was proved, reading values outside the statement | `driver/source.rs:58` | `34cc069` | `input_count_tests.rs` |
| The advice fill stopped after eight passes, so nine dependent comparisons were unprovable | `driver/advice.rs:26` | `d1c3d69` | `advice_fill_tests.rs` |
| The advice evaluator refused a select whose condition is a bit only once comparisons settle | `vm/step/sel.rs:31` | `d1c3d69` | `advice_fill_tests.rs` |
| Library entry points reduced an input at or above the modulus instead of refusing it | `driver/source.rs:64`, `:89` | `2c3f39b` | `field_input_tests.rs` |

### Native targets

| Finding | Where | Fix | Test |
| --- | --- | --- | --- |
| C and assembly returned 0 for an inverse of zero and took any nonzero select condition as true | `backend/emit_c/op.rs:21`, `:22`, `backend/emit_asm/op.rs` | `29b831c` | `native_guard_tests.rs` |
| C and assembly read a missing or malformed argument as zero and reduced one past the modulus | `backend/emit_c/program.rs:26`, `backend/emit_asm/io.rs:17` | `29b831c` | `native_guard_tests.rs` |
| The assembly left the stack executable | `backend/emit_asm/program.rs:10` | `29b831c` | `native_guard_tests.rs` |
| Python wrote constraints as `assert`, which `python -O` removes, and read inputs loosely | `backend/emit_python/op.rs:23`, `:24` | `4cf0cd5` | `python_guard_tests.rs` |
| No target could run a program with an ordered comparison, since each asked its caller for the comparison bits | `backend/inputs.rs:12` | `172799b` | `native_compare_tests.rs` |

### Command line and includes

| Finding | Where | Fix | Test |
| --- | --- | --- | --- |
| An unreadable input was dropped, shifting every later input | `nonos_zkolang_cli/src/main.rs:88` | `dca63a5` | `nonos_zkolang_cli/tests/inputs.rs` |
| Includes were keyed by spelling, so one file spliced in twice, and a library naming the program back spliced it in again | `lang/include/expand.rs:31` | `d1402ce` | `include_key_tests.rs` |
| Every include resolved from the main file's directory, so a project file replaced a library's own include | `nonos_zkolang_cli/src/main.rs:107` | `d3c644d` | `nonos_zkolang_cli/tests/includes.rs` |
| An include could name any absolute path or climb with `..` | `nonos_zkolang_cli/src/main.rs:107` | `d3c644d` | `nonos_zkolang_cli/tests/includes.rs` |
| The file was the first argument not starting with `-`; a flag without a value was ignored; `check` passed programs too long to prove | `nonos_zkolang_cli/src/main.rs:77`, `:84` | `72cb2fc` | `nonos_zkolang_cli/tests/args.rs` |

### Diagnostics

| Finding | Where | Fix | Test |
| --- | --- | --- | --- |
| The renderer panicked on an offset inside a multibyte character | `lang/diagnostic.rs:89` | `f47eb05` | `render_tests.rs` |
| The caret did not line up under tabs, and an error at the end pointed past the last line | `lang/diagnostic.rs:89` | `f47eb05` | `render_tests.rs` |

## Open

These stay as they are in edition 2025. Most would change how registers are allocated,
and so the compiled form, and so the commitments of registered circuits; the rest are
limits of the edition's design. Edition 2026 addresses each in its own design.

### Register pressure

Measured at commit `7142568` with a program that declares two inputs `x` and `y`, binds
`n` values `let v_i = x + i;` that stay live until they are all output at the end, and
computes one expression in between.

- One `x * y` compiles beside 30 such values. One `x < y` compiles beside only 9: an
  ordered comparison range-checks both operands to 16 bits and decomposes their
  difference into 17, and each decomposition holds all of its bits in registers before
  it recomposes them (`lang/compile/expr/compare.rs`, `expr/decompose.rs`).
- A `match` on one live input compiles with at most 15 arms. It becomes a chain of
  selects (`lang/parse/parser/match_expr.rs`), and lowering a select holds its condition
  and first arm while it lowers the rest of the chain, two values per arm
  (`lang/compile/expr/select.rs`).
- A loop body frees nothing until the loop ends: 32 single-use `let`s fit in a loop body
  of one iteration, while the same statements at top level compile past 63
  (`lang/compile/stmt/for_loop.rs`; `free_dead` runs only between top-level statements).
- A block holds its locals until it closes: 16 pairs of locals that are dead before the
  block's result fit, and 17 do not (`lang/compile/expr/block_open.rs`).
- A function body's locals are block locals, so the same holds for a body that fits at top
  level but not as a function.
- Arrays are never freed when their name goes dead (`lang/compile/compiler/free_dead.rs`
  reclaims scalar bindings only), and a `let` of a constant that the optimizer drops
  leaves an array of the same name bound.
- A value hoisted by sharing stays live to the end of its statement, and one hoisted in a
  loop body stays live to the end of the loop.
- Propagation puts a constant back at every use as an immediate, which costs a row per
  use.

### Language limits

- An index must fold from literals and loop variables; a named scalar constant, a
  function parameter or a `let` bound to a constant is not accepted, in either build.
- Loop bounds are integer literals.
- A constant table holds non-negative literals below 2^64.
- A function is checked only where it is called, apart from recursion and repeated
  parameters: an undefined name in an uncalled function is not reported.
- The include directive is recognised only on a line of its own.

### Diagnostics

- Most semantic errors carry no location: arity, tuple arity, array or tuple used as a
  scalar, non-constant index, too many registers, loop too large, and the name errors.
- An unknown name is located at its first occurrence as an identifier anywhere in the
  source, which can be a parameter or a definition in another scope.
- After includes are expanded, lines are counted in the expanded text, so an error in a
  program that includes the standard library points at the wrong line and names no file.
- A missing include is reported by the command line with its path and the including
  file, but `CompileError::IncludeNotFound` itself carries neither.
