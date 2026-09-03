# bench

Measured numbers, one JSON per pipeline stage, refreshed by the `bench` CI
job on every run of the proofs workflow. Nothing in here is estimated: a
value either came from a run whose commit is named in the file, or it is
absent.

- `transfer.json` — the sender's proof: prove, verify-from-bytes, size, peak.
- `recursion.json` — the aggregation proof over the deployed join-split.
- `assembly.json` — the outer circuit's shape: rows, width, regions, groups.

The committing run writes its own commit hash into each file, so a number
can always be traced to the exact tree that produced it.
