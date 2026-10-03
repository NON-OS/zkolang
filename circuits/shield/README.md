<!-- NONOS. AGPL-3.0-or-later. -->

# Shield circuits

The private-value utility: a note is a hidden commitment, a spend proves the note is in
the tree and retires it with a nullifier, and value stays private throughout.
`note_commit.zkl` computes the pool's note commitment, the thirty two round Poseidon over
eleven limbs, and `note_commit_deployed_tests` requires it to equal the digest the STARKs
repository pins as deployed. `spend_note.zkl` and `note_root.zkl` are a demonstration of
the language, not the deployed protocol: they build the note and the tree from the
standard library's MiMC permutation over a short path, so they do not interoperate with
a real note. `spend_note.zkl` is the whole utility in one circuit; `note_root.zkl` is its
companion that computes the tree root a spend authenticates to.

The verifier key is
`keccak256(0x01 ‖ commit ‖ log2N_le ‖ trace_width_le ‖ rate_le ‖ periodic_root)` at
registration rate three and trace width fifty one, as the prover of the STARKs
repository builds it; `shield_readme_tests` checks each key below against its circuit.

| Circuit | Role | Verifier key |
|---|---|---|
| `spend_note.zkl` | prove a note's membership, retire it, range-prove its value | `c1d6c325…e783854b` |
| `note_root.zkl` | compute the commitment-tree root a note authenticates to | `c6565601…d3145954` |

`spend_note.zkl` composes the whole language: the standard library and its MiMC
permutation, an array indexed by an unrolled loop, a nested loop over the Merkle path,
the range gadget, and the cypherpunk register. A spend reveals only the nullifier; the
value, the spending key, and the note's position stay private. It is proven in
`shield_tests`: a valid note spends, the nullifier is deterministic and position
dependent, a spend against the wrong root has no proof, and a value that is not a byte
has no proof.
