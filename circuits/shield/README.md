<!-- NONOS. AGPL-3.0-or-later. -->

# Shield circuits

The private-value utility: a note is a hidden commitment, a spend proves the note is in
the tree and retires it with a nullifier, and value stays private throughout. These
circuits are what NOX settles against, registered on chain by verifier key and proven in
the kernel. `spend_note.zkl` is the whole utility in one circuit; `note_root.zkl` is its
companion that computes the tree root a spend authenticates to.

The verifier key is
`keccak256(0x01 ‖ commit ‖ log2N_le ‖ trace_width_le ‖ rate_le ‖ periodic_root)` at
registration rate three and trace width fifty one.

| Circuit | Role | Verifier key |
|---|---|---|
| `spend_note.zkl` | prove a note's membership, retire it, range-prove its value | `a841dec9…a8752da0` |
| `note_root.zkl` | compute the commitment-tree root a note authenticates to | `b026aa91…f777b7e4` |

`spend_note.zkl` composes the whole language: the standard library and its MiMC
permutation, an array indexed by an unrolled loop, a nested loop over the Merkle path,
the range gadget, and the cypherpunk register. A spend reveals only the nullifier; the
value, the spending key, and the note's position stay private. It is proven in
`shield_tests`: a valid note spends, the nullifier is deterministic and position
dependent, a spend against the wrong root has no proof, and a value that is not a byte
has no proof.
