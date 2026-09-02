# The wide trace commitment: the next outer-shrink campaign

Measured tonight, sidecar assembly at cap 2: the auth region is the largest
remaining row owner, and its shape is `n_open x (depth + 1) x l` per query
with `n_open = 17`: one FRI leaf, deep, comp, and fourteen trace openings,
because the poseidon prover commits each trace column under its own tree.

## The change

The poseidon prover commits one wide tree: leaf i is the hash of trace row i
(the chunk-chain rule the periodic tree already uses), one root instead of
fourteen. A query opens one path whose leaf binds all fourteen values.

## What it buys, from tonight's numbers

- auth per query: 17 openings to 4, rows 22400 to about 5300 at deployment
  depth. Just under half the remaining circuit.
- transcript: absorbs 1 trace root instead of 14; ntr collapses, the
  transcript region and every replay shrink with it.
- proof bytes: one trace path per query instead of fourteen. The calldata
  the settlement contract carries shrinks by roughly the same factor as auth.

## What it costs

- A proof format change on the poseidon path: prove, verify, replay,
  deep-term and opening builders, and the recursion's auth all move together.
- The row-hash gadget already exists and is already provable in-circuit
  (the periodic sidecar's chain opening is exactly this shape), so the
  recursion needs no new region type: the trace opening becomes one more
  chain-plus-path opening, like the periodic one.
- The keccak path and the fixture recursion are untouched; the byte-digest
  gate must hold unchanged, which it will because nothing on that path moves.

## Order of work

1. Wide commit in the poseidon prover (plain and pre share it through the
   trace module).
2. Verifiers and the replay walker take the single root.
3. The recursion's openings builder emits one chain-plus-path trace opening;
   deep's value ties bind against its chunk cells, the same way the periodic
   quotients already do.
4. Gates in the same ladder: bind truth, diagnose, tamper matrix, digest.

Do this after the current ladder certifies; it touches the same seams the
ladder just hardened, and the labeled-bind and fast-loop tooling makes its
debugging hours, not nights.
