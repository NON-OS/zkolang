<!-- NONOS. AGPL-3.0-or-later. -->

# Kernel circuits

The NONOS microkernel expresses its trust boundary as zKølang circuits. Each is proven
in the kernel's transparent STARK, so there is no second prover to trust, and each is
registered on chain by its verifier key, so a proof is checked against an exact circuit.
These are production circuits, not examples: every one has an accept case and, where it
enforces a rule, a reject case with no proof, in `kernel_tests`.

The verifier key is
`keccak256(0x01 ‖ commit ‖ log2N_le ‖ trace_width_le ‖ rate_le ‖ periodic_root)` at
registration rate three and trace width fifty one. Recompute any row with
`zkolang key circuits/kernel/<name>.zkl`.

| Circuit | Kernel role | Verifier key |
|---|---|---|
| `attest.zkl` | admit a capsule only if its content hashes to a trusted measurement | `d1b0e0b7…f8a110f3` |
| `anti_rollback.zkl` | boot only if the rollback index meets the TPM monotonic floor | `2ec49574…c18df008` |
| `capability.zkl` | grant a right only if it is in the broker's granted set | `e10fbb5f…711f6edd3` |
| `measure_root.zkl` | fold four boot-stage measurements into one root | `c56fbad1…b63fead80` |
| `boot_chain8.zkl` | fold eight boot-stage measurements into one root | `9d1d56ed…7bd13c7e2` |
| `seal.zkl` | bind data to a measurement so only that platform unseals | `b7c6f04d…f493b94ba` |
| `syscall_auth.zkl` | authorize a syscall by its broker capability token | `74724f73…5b8eca7b` |

`anti_rollback.zkl` is the ordered comparison in a real setting: the kernel keeps a
monotonic floor in a TPM counter, and an image boots only when `floor <= index`. Below
the floor the circuit has no proof, so a signed but stale image carries no attestation.
`attest.zkl` keeps the capsule content private, a witness, and publishes only the
measurement. `capability.zkl` and `syscall_auth.zkl` decide authorization with field
arithmetic alone, no bitwise operations, sound because a product of differences is zero
exactly when a request matches a grant and a hash binds a token to its fields.
