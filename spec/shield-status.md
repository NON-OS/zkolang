# Shield: what is measured

Every number here comes from a run in this repository, named so it can be
reproduced. Nothing is projected. Where a figure was wrong earlier it is marked,
because the wrong ones circulated.

## A private transfer works

`shield::test::roundtrip`, at the deployed tree depth of 32, two inputs and two
outputs, 1000 + 2000 spent, 1500 + 1200 created, 200 out publicly, 100 in fees:

| | |
|---|---|
| a real transfer proves and verifies | yes |
| a spend of an unowned note | proof does not verify |
| wall clock, both cases | 1980 s |
| peak resident | 565 MB |

This is a prove and verify, not a witness satisfaction check: the prover runs,
FRI commits, the verifier accepts. Both gates carried `#[ignore]` and the history
shows no sign they had been run before.

A wallet can produce a transfer proof on an ordinary machine today.

## What binds

Fourteen bindings in the shield circuit, each with a forgery that violates
exactly it while everything else stays honest. `shield::test::inventory` holds
the list and fails if a binding is added without one. Nullifiers are among them,
so ownership and double spend are covered.

Six binding families in the recursive verifier, each with a forgery, in
`family_tests`. Three of those families had no gate at all until recently:
fold, index and periodic, where index is the seam the assembly itself calls
forgery critical.

Every set includes a positive case. A tamper rejecting shows a cell is
constrained to something, not to the right thing, and a binding that stops
closing rejects honest witnesses too. That is not theoretical: a region
deduplication that merged 32 regions it should not have left all six forgeries
passing and only the honest case failing.

## Recursion is a separate, much larger shape

The recursion assembly verifies a join-split proof inside a proof. It is what
makes batched settlement cheap. It is not on the path of a single transfer, and
its cost has been quoted as a transfer cost more than once, including in this
repository's own probe names.

| | periodic columns | LDE |
|---|---|---|
| one grand product per cycle | 10209 | 11.6 TB |
| one product over all columns | 1146 | 9.8 TB |
| products capped at width 8 | 1130 | 1.22 TB |
| periodic shared between regions of a kind | 789 | 876 GB |

A product over k wired columns carries degree k+1, so fusing everything into one
product raised the degree from 10 to 80 and inflated the evaluation domain
eightfold, which gave back nearly all of what the narrower trace saved.

## What is next, in size order

**Auth's reset columns.** 576 of the 789 remaining periodic columns are the auth
region, one kind per query because all 32 differ. They are
`rc[8] + slot_bnd + op_bnd + reset[8]`, and only `reset` varies: it is
`initial_state(openings[k+1])`, which is `inject(leaf, sibling[0], direction[0])`
— Merkle leaf values, witness data, sitting in public structure. `inject` is a
conditional swap, degree 2 against an AIR already at degree 10, and `sibling` and
`direction` are already trace values under `witness_path`. Moving `reset` to the
trace makes auth query independent, deduplicates it to one kind, and takes the
assembly to roughly 279 GB.

This is a soundness change, not a layout one. A constraint has to force the moved
value to equal the next opening's initial state, and adding trace columns shifts
`ocells`, which the grand-product bindings are written against. That is the exact
mechanism behind the original inner-coverage bug.

**Grand-product id and sigma columns.** About 153 of the 789. `id[r] = r*k+j` is
closed form and `sigma` is a permutation that can be carried as `span * k`
integers rather than a materialised LDE.

**Wallet derivation.** Unowned. The key hierarchy and its vector are frozen in
`spec/shield-key-hierarchy.json`, so it is implementable now. Until it exists a
beta participant cannot spend a deposited note, which makes it the longest pole
to a usable system and the only remaining item that is not an optimisation.

## Corrections

- 11.6 TB and 9.8 TB were quoted as the cost of a transfer. They are the
  recursion assembly. A transfer is 565 MB.
- 287 GB was measured on an assembly that merged 32 distinct auth regions. It
  binds nothing. The correct figure for that change is 876 GB.
- The recursion binding gates were called too heavy for a pull request. They are,
  through a full 32-query prove. On a two-query cap reading witness satisfaction
  they cost three minutes and 300 MB and run on every push.
