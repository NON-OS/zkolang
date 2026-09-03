# proofs manifest

## recursion-real.proof

The aggregation proof over the deployed join-split: a real private transfer,
proven inside the full thirty-two query recursion and verified from these
exact bytes before they were committed.

    sha256        bbeaed2923f4b4e9c4aa80317dea16f6f0f81e0f2b41473dfe14f2b07ae35368
    size          899,196 bytes
    inner         deployed join-split, 32 queries, 16 grind bits, rate 1/16
    outer         32 queries, 8 grind bits, rate 1/2 (pipeline artifact; the
                  settlement-rate outer follows the two shrink campaigns in docs/)
    assembly      width 436, 2^20 rows, degree 13, 168 transitions, 46 groups
    assembled     746.7 s
    proved        8685.7 s on 56 cores
    verified      29.2 s, deserialized from disk, true
    peak memory   33.9 GB
    producing     branch outer-shrink, the commit that carries this file

## transfer.proof

Awaiting the next bench refresh; the CI job emits and verifies it on every
scheduled run. Measured on GitHub's runners meanwhile: proved ~81 s, verified
from disk, 431,580 bytes, ~354 MB peak.
