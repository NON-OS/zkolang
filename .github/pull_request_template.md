<!-- Keep this short and true. Delete lines that do not apply. -->

## What this changes

<!-- One or two sentences. What moves, and why. -->

## Checks

- [ ] `cargo test --workspace --release` passes, and any new binding has a forgery that rejects.
- [ ] If a proof or circuit shape changed, the byte-digest gate and the affected fixtures were re-baked on purpose, not by accident.
- [ ] If a soundness parameter changed, the 128-bit security-level test still passes.
- [ ] If Lean sources changed, `lake build` is clean and the axiom audit shows no `ofReduceBool` or `sorryAx`.
- [ ] No authorship traces, host secrets, or coordination notes in the diff (the hygiene gate enforces this).
- [ ] Commit messages read as a human wrote them: no em-dashes, no machine trailers.

## Notes for the reviewer

<!-- Anything load-bearing that is not obvious from the diff. -->
