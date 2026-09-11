# The compose pipeline: degree thirteen becomes bounded

Profiled on the first full-coverage emit and named in the wide-trace doc:
the generic compose region evaluates the inner's whole transition at the
out-of-domain point as one constraint per inner transition, so the outer's
constraint degree is the inner's, thirteen. That degree sets the outer
blowup: bound = next_power_of_two(13 * 2^20) = 2^24, domain 2^25. Halve the
degree and the domain halves with it; the prover's dominant pass shrinks by
the same factor again.

## Why the degree is thirteen

`ComposeCheckGen::transition_impl` pins each witnessed transition value to
`transition_gen::<Ext2<F>>(&frame, &periodic)` evaluated symbolically over
the frame cells. The inner's constraint code is opaque to the gadget, which
is the whole point of the generic assembler, so the recompute constraint
carries whatever degree the inner's own polynomials have. The shield inner's
Poseidon rounds put that at thirteen.

## The change: record the evaluation, witness its products

The gadget cannot see inside `transition_gen`, but it can run it over a
field element that records. A `Tape` felt implements the `Felt` operations
by appending nodes to a shared tape: inputs (frame, periodic cells),
constants, additions, multiplications. One run of the inner's own
constraint code over `Ext2<Tape>` yields the complete arithmetic DAG of the
recompute, without touching a line of any inner.

The compiler then lays the tape onto a strip of rows:

- Every multiplication node becomes a witnessed slot. Its constraint is
  `slot - a * b`, degree two in witnessed values; with the linear
  combinations folded in, nothing exceeds degree three.
- Additions and constant scalings fold into the consumers; they cost no
  slot and no constraint.
- The strip is as wide as the region's existing width budget allows and as
  tall as the mul count requires. Row r's constraints may read rows r and
  r+1 (window two), so producers sit at most one row above consumers; the
  scheduler orders the DAG by depth and spills across rows.
- The final row pins the recomputed transition values to the `out` slots
  the comp_z batching already consumes, so everything downstream of the
  recompute is untouched: the tower, E, the boundary quotients, and the
  batched sum keep their shape and their bindings.

Determinism: the tape is a pure function of the inner's constraint code and
the slot layout is a pure function of the tape, so the emitted structure
stays derived, never typed, and two builds of the same inner produce the
same strip.

## What it buys

- Outer constraint degree: 13 to at most 3 for the recompute, leaving the
  comp_z batching's 3 as the region's max. The outer bound drops from
  2^24 toward 13*2^20 rounded at the new degree: two domain bits, 2^25 to
  2^23. Point count and per-point cost both halve twice.
- The wide-trace campaign cut rows; this cuts the domain. Together they are
  the tens-of-minutes settlement prove the roadmap promises.

## What it costs

- The strip's rows: one slot per multiplication in the inner's transition
  evaluation. The shield inner's Poseidon-heavy code records hundreds of
  muls; at the region's width that is a strip of tens of rows, against a
  layout whose per-query blocks are thousands. Net rows still fall with
  the domain.
- A new Felt implementation and a small scheduler. No inner changes, no
  proof-format changes, no transcript changes.

## Measured: the carry model loses, the cycle model wins

The scheduler ran against the real tape and the numbers overruled the
carry-lane design. Cone-ordered packing at any row budget plateaus at 672
products consumed as mul operands far from their birth (the wide-mul limb
chains, not Poseidon, which is local), plus 164 live inputs: width 1768 at
best against the outer's 436. Carry lanes cannot ship this strip.

The wired engine already holds the answer: distant equality is what the
permutation argument's copy cycles do, at the cost of cycles rather than
columns. A product read far below its birth binds by cycle, not by lane;
inputs bind by cycle to the statement cells they already ride; and the 168
output-fed products fold into running accumulator lanes. Width becomes 2k
for the product slots plus a handful of accumulators: roughly 300 at
k = 128 over 27 rows, inside budget with room. The strip joins the wired
permutation the same way every other region already does.

## Measured: the strip fits, and the next ceiling has a name

Scalar folding settled the size: constant-by-variable products are edges,
not slots, so the 3525 recorded muls are 1548 witnessed ones. Under the
cycle model the width driver is per-row echo reads, and at 32 products a
row the strip is 45 rows by 328 columns, inside the 436 budget with room
for the accumulators.

The degree ledger, from each gadget's own declaration: compose falls from
the inner's 11 to the strip's 3, and the outer's ceiling becomes
MultiMembership's 8, the Poseidon x^7 round under its selector, with the
transcript's 7 just beneath it. So this campaign buys one domain bit,
2^25 to 2^24, and the second bit has a named follow-up: witness the
S-box squares inside the membership and transcript rounds (x^2 and x^4
as cells, the round constraint falling to degree 4), a bounded change to
two gadgets that takes the ceiling to about 4 and the domain to 2^23.
Speed lands in two steps with a receipt at each, rather than one step
with a hope.

## The census settles the shape: narrow and tall

Compose is a shared region, so its rows cost almost nothing against the
span while its width bids against the scarcest resource in the trace. The
packing sweep at small k: 4 products a row gives 355 rows by about 100
columns with 46 echoes a row, narrower than the flat region it replaces.
The operand census over the layout: 79% of linear forms carry one or two
terms, 128 carry seven, and 256 carry thirty-three; the wide ones are the
value-balance limb recombinations, sums over products spread across rows,
and they run as accumulator chains exactly like the outputs. 176 forms
carry a folded constant; 128 recorded products feed no output and fall
out of the layout unwitnessed.

The schedule at this scale is dense and honest: each row's eight operand
slots read the row's candidate cells under periodic coefficients, about
480 schedule columns total, committed under the outer sidecar beside the
418 the outer already carries.

## The strip's AIR: coefficients are the schedule

Every product row checks different linear combinations, and an AIR's
transition is one uniform function. The resolution is the pattern this
codebase already ships everywhere else: the per-row combination
coefficients become periodic columns. Each product lane carries one
uniform constraint,

    out = (sum_j A_j(row) * cell_j) * (sum_j B_j(row) * cell_j)

with A and B periodic schedules over the strip's rows and the cell set
drawn from the window: the lane's own operand echoes, the previous row's
products, and the accumulator lanes. Degree stays at three (schedule times
cell, squared by the product), and the audit asserts it before any prover
runs.

The pieces, concretely:

- k product lanes per row, each one Ext2 slot, constraint out = opA * opB
  with operands assembled by the periodic schedules.
- Echo cells for the 672 distant operands: a distant value is cycled into
  an echo cell on its consumer's row by the permutation argument, and the
  schedule reads the echo like any local cell. Averaged over the strip
  that is under one echo per five products.
- Accumulator lanes for the 168 output-fed products: acc[r+1] = acc[r] +
  sum of schedule-weighted products born at row r+1, degree three, and the
  final row's accumulators pin to the out cells the comp_z batching
  already consumes.
- Statement inputs bind by cycle into echo cells where consumed, so the
  164 input lanes stop existing as lanes at all.
- The schedules commit under the periodic sidecar like every other
  schedule: baked root, one opened row per query, never recomputed. The
  strip's schedule is derived from the tape, and the tape from the inner's
  own code, so the whole layout remains a pure function of the inner.

## Order of work

1. The `Tape` felt and the recorder: run the shield inner's
   `transition_gen` over `Ext2<Tape>`, snapshot the tape, and prove the
   replayed tape evaluates to the same values as the direct evaluation on
   host, over random frames.
2. The strip compiler: tape to slot layout to witness rows, with the
   depth scheduler and the degree audit (assert no emitted constraint
   exceeds three before it ever reaches a prover).
3. `ComposeCheckGen` grows a strip form beside the flat form; the flat
   form stays for window-size-two inners whose degree is already low.
4. The assembly consumes the strip form for the real inner; layout and
   bindings move with it (frame, periodic, point, coefficient, comp_z
   cells keep their accessor discipline).
5. Gates in the ladder's order: satisfaction, bind truth, diagnose,
   tamper matrix, capped FRI round trip, and the keccak byte digest,
   which must not move because nothing on the keccak path does.

## Built: the split lands in the assembly

The integration went in three gated stages. The statement type lives with
the region, one source. The flat region's strip mode keeps the tower, the
exempt factor, the quotients and the comp_z batching, drops the recompute,
gains one acc cell per transition, and pins out = acc + statement with the
statement evaluated over its own cells; base input lane u of the recording
is window column u by the shared frame-then-periodic ordering, so the pin
constraints read raw window cells. Gated standalone over the real inner:
width 410, all 122 constraints vanish, and the region reports degree 3
where the recompute form reported 13.

The assembly inserts the strip as a shared region between compose and the
FRI transcript. Every echo cell binds by cycle to its producer, a strip
lane or a flat statement cell; every final accumulator binds to its acc
cell in the flat region; the cycles are resolved to absolute coordinates
in the build and emitted through their own binding family. The strip's
own boundary carries the accumulator zero pins, and the stacked engine's
base-boundary collection enforces them with no external pin. The fixture
and step assemblies keep the flat recompute, so the default suite and the
byte digest never see the strip.
