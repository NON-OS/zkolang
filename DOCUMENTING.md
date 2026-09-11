# Documentation standard

The code carries its own explanation. This file states how, so every module reads as though
one careful engineer wrote all of it. It is a house standard, not a style suggestion: a file
that does not meet it is not finished.

## The two levels

Every source file opens with a module doc comment (`//!`), placed after the license header.
Every public item (`pub fn`, `pub struct`, `pub enum`, `pub trait`, `pub const`, `pub type`)
carries an item doc comment (`///`). These are not optional and they are not decoration; a
reviewer reads the docs to decide whether to trust the code, so the docs must earn that.

## What a doc says, and what it never says

A doc states what the code cannot show on its own: the constraint it enforces, the invariant
it depends on, the reason a decision was made, the soundness role a region plays, the boundary
a value must not cross. It does not restate the signature, narrate the next line, announce that
a change is correct, or record where the code came from. If a comment explains the commit
rather than the code, it is noise the moment the change merges, and it does not belong.

The test: remove the doc and ask what a competent reader still cannot recover from the code.
Write that, and only that.

## Voice

A senior engineer talking to the next engineer. Calm, declarative, lowercase prose. No stacked
capital-label blocks (`SECURITY:`, `IMPORTANT:`), no audit-bot narration, no emoji, no
rule-of-three cadence, no em-dashes. Numbers as digits. First person plural only where it is
natural. The voice of the module docs already in the tree, `wired_multi_gen`, `replay`,
`value_balance`, is the reference; match it.

## Inline comments

Doc comments (`//!`, `///`) explain the item. Inline comments explain a step inside a body,
and they carry the subtlety the code cannot: why the algebra works, which case a branch covers,
the invariant a line preserves. A single non-obvious line takes a `//` above it. A multi-line
explanation, the witness trick behind a constraint, the case split behind a gadget, takes a
block comment in this exact shape, opener and closer on their own rows, every content line led
by an aligned asterisk:

```
/*
 * The prover witnesses aux, and this constraint forces a * aux = 1, so a zero input has no
 * solution and the trace is unprovable rather than wrong.
 */
```

The block form is for the places a reviewer would otherwise have to reverse-engineer the
algebra. Do not spend it narrating obvious code; spend it where the reason is real and would
be lost. Never mix the block form with `///`: doc comments stay `///`, inline stays `//` or the
block above.

## Placement and mechanics

- The module `//!` goes directly under the license block, before the first `use`.
- `mod.rs` files re-export and say what the module is for as a whole; they do not hold logic,
  so their doc describes the shape of the module, not an algorithm.
- An item doc sits immediately above the item, no blank line between.
- Internal (`pub(super)`, `pub(crate)`) items on the trusted path deserve a doc too when the
  reason is not obvious from the name; private helpers get a line only when they carry a
  constraint or a subtlety.

## Priority order for closing gaps

Documentation is closed in the order the priority filter sets, highest value first:

1. The trusted and soundness path: the provers, the verifiers, the AIR regions, the recursion
   assembly, the value-balance and nullifier and membership logic, the on-chain verifier.
2. The public API surface of each crate: anything a caller outside the module can reach.
3. The compiler front end and the standard library.
4. Everything else.

A file on the trusted path with a missing or thin doc is a higher-priority fix than a
formatting nit anywhere.

## The measure

The baseline at the time of writing: 560 source files, 70 percent with a module doc, 68 percent
of public items with an item doc. World-class is 100 percent of both, in the voice above, with
`cargo doc` building clean. The gap is closed by priority, not by a blind sweep, because a
module doc written without understanding the module is worse than none.
