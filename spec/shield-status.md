# Shield: what is measured

Every number here comes from a run in this repository. Nothing is projected.

## One verifier key covers every transfer

A verifier key binds the preprocessed periodic columns. Anything witness dependent
among them makes the key instance specific, and a contract holding one key cannot
then check a second transfer.

Two transfers built from different secrets, amounts, assets and notes:

| | |
|---|---|
| periodic columns | 140, identical across both |
| columns following the witness | 0 |
| preprocessed root | same for both |

`vk_stability_tests::two_transfers_share_one_verifier_key` builds the pair and fails
if a single column deviates. It runs in 0.05 s on every change. It was run against the
tree before the fix to watch it fail, which is the only way to know a gate works.

## A private transfer works

`shield::test::roundtrip`, at the deployed tree depth of 32, two inputs and two
outputs:

| | |
|---|---|
| a real transfer proves and verifies | yes |
| a spend of an unowned note | proof does not verify |
| wall clock, both cases | 1147 s |
| peak resident | 438 MB |

Prover runs, FRI commits, verifier accepts. Not a witness satisfaction check.

## What binds

Fourteen bindings in the shield circuit, each with a forgery that violates exactly it
while everything else stays honest. `shield::test::inventory` holds the list and fails
if a binding is added without one. Nullifiers are among them, so ownership and double
spend are covered.

Six binding families in the recursive verifier, each with a forgery, in `family_tests`.
Fold, index and periodic had no gate until recently, and index is the seam the assembly
itself calls forgery critical.

Every set carries a positive case. A tamper rejecting shows a cell is constrained to
something, not to the right thing, and a binding that stops closing rejects honest
witnesses too. A region deduplication that merged regions it should not have left all
six forgeries passing and only the honest case failing.

## Recursion

The recursion assembly verifies a join-split proof inside a proof. It is what makes
batched settlement cheap. It is not on the path of a single transfer.

| | periodic columns | LDE |
|---|---|---|
| one grand product per cycle | 10209 | 11.6 TB |
| one product over all columns | 1146 | 9.8 TB |
| products capped at width 8 | 1130 | 1.22 TB |
| periodic shared between regions of a kind | 789 | 876 GB |
| opening state moved to the witness | 192 | 279 GB |
| identity and selector columns shared | 136 | 223 GB |

A product over k wired columns carries degree k+1, so fusing everything into one
product raised the degree from 10 to 80 and the evaluation domain with it.

## What the LDE figures do and do not count

The dims probes report `trace_lde + periodic_lde`. They do not count the Merkle trees
over those extensions, and a tree keeps its whole leaf layer at `[Fp; RATE]` per leaf,
which is four times the column it commits to. Nor do they count `comp_d` or `deep_d`.

So the recursion figures are a like-for-like measure of the extension arrays across
changes, which is what they were written for. They are a lower bound on what a prover
needs, not the requirement. The only measured peak in this document is the transfer's,
which is a real resident set.

## What is next, in size order

**The commitment layer.** With the periodic columns streamed, the trace extension and
the trees over it are what remain. The trees cannot be dropped the same way, because the
leaf layer is what an opening walks, so this is a change to how commitment works rather
than a loop that can be chunked.

**Sigma.** The per-group sigma columns are most of what is left among the periodic
columns. Sigma is a permutation and can be carried as `span * k` integers rather than a
materialised extension. The identity column is closed form and already shared.

**Wallet derivation.** Unowned. The key hierarchy and its vector are frozen in
`spec/shield-key-hierarchy.json`, so it is implementable now. Until it exists a beta
participant cannot spend a deposited note, which makes it the longest pole and the only
remaining item that is not an optimisation.

## The opening boundary is proven, not just tested

Leaving the opening's first state to the witness rests on the copy constraint that pins
the half of it the leaf occupies. `Zkolang.Opening.injected_of_nodeHalf` proves that a
state whose node half is the bound leaf is an injection of that leaf, so pinning the
half is not weaker than constructing the whole, and the prover's remaining freedom is
the sibling. `nodeHalf_inject` proves the pin reads back what was injected, so the
constraint the assembly places is the one the argument assumes.

Both hold over all states rather than the ones a prover happens to produce, in Lean 4
over the core library alone, and neither depends on any axiom.

## Corrections

- 11.6 TB and 9.8 TB were quoted as the cost of a transfer. They are the recursion
  assembly. A transfer is 565 MB.
- 287 GB was measured on an assembly that merged 32 distinct auth regions. It binds
  nothing. The figure for that change alone is 876 GB.
- The recursion binding gates were called too heavy for a pull request. They are,
  through a full 32-query prove. On a two-query cap reading witness satisfaction they
  cost three minutes and 300 MB and run on every push.
