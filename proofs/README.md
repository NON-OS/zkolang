# proofs

Real artifacts at deployment parameters: 32 queries, 16 grind bits, rate
1/16. Every proof here was verified from its own bytes after being written,
by the run that committed it; the sha256 sits beside it so anyone can check
they hold the same file.

These are not fixtures and not test vectors. A file lands here when the
pipeline that will settle on L1 produced it end to end.

- `transfer.proof` + `.sha256` — one private transfer, proven and verified.
- `recursion.proof` + `.sha256` — the aggregation proof over the deployed
  join-split circuit.
- `MANIFEST.md` — parameters, timings and the producing commit for each.
