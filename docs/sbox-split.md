# The S-box split: transition degree, not the domain

Correction, measured on the S emit: the outer's degree ceiling is NOT
the transition round. The assembled engine takes its degree as
`(max_region_degree + 2).max(max_group_width + 2)`, and the widest
permutation group carries ten wired columns, so the grand product sets
the ceiling at twelve regardless of any transition. The strip dropped
compose's transition from thirteen to three and the transcript and
membership rounds sit at seven and eight, all of them under the grand
product's twelve.

So this split lowers the round transition from eight to four, which is
real and which the recursion needs, but it does not move the domain on
its own. The domain lever is narrowing the ten-column permutation
group; the S-box split only starts to bite once that group is narrowed
below eight, at which point membership's round would otherwise become
the binding constraint. Sequenced honestly: narrow the group first,
then this split keeps the round from becoming the new ceiling.

## The change

Witness the squares. Each round lane gains two cells, x2 and x4, with
y = state + rc:

    x2 - y * y          degree 2
    x4 - x2 * x2        degree 2
    sbox = x4 * x2 * y  degree 3 in place of y^7

The MDS layer is linear, so the round output falls from degree 7 to 3,
and the selector multiplication lands the constraint at 4. The square
constraints hold uniformly at every row, boundaries included, so they
need no gating.

## The blast radius, contained by construction

MultiMembership is not recursion-only: the deployed inner's membership
instances are this gadget, and the shield batch rides it too. Changing
its shape unconditionally would rewrite the inner, move the byte digest,
and break any verifier already keyed to the current inner.

So the split is opt-in. `new_witness_split` and its chain twin append
the 2 * WIDTH square cells after the existing columns and swap the
transition to the split round; every default constructor stays
byte-identical. The inner, the shield batch, the digest, and every
existing coordinate formula (siblings at WIDTH + 1, chunk lanes, opened
cells) survive untouched because the new cells append. Only the
recursion assembly's constructors flip: the auth batch, the trace and
periodic chains, and both transcripts.

## What it buys and costs

- Outer ceiling 8 to 4, domain 2^24 to 2^23: point count and per-point
  cost halve once more. With the wide trace and the strip, the three
  campaigns together are the settlement prove the roadmap promised.
- Width: split membership grows by 16 columns, split transcript
  likewise; both stay far under the outer's width, which compose still
  sets.
- Rows: unchanged. Constraints: 2 * WIDTH more per region, all degree 2.

## Order of work

1. Open the round: the hasher exposes its rc-add and MDS pieces
   generically, so the split transition composes them around the
   witnessed squares instead of calling the closed round.
2. MultiMembership grows the split form; a standalone gate proves the
   split region satisfies over a real opening and reports degree 4, and
   the default form's trace stays bit-identical to before the change.
3. TranscriptCheck the same way.
4. The recursion assembly flips its constructors; the ladder walks:
   satisfies, bind truth, diagnose, tamper matrix, and the byte digest,
   which must not move because no default moved.
5. The emitted structure reports constraint_degree 4, and the S prove
   reruns at 2^23.
