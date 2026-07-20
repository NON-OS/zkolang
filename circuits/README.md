<!-- NONOS. AGPL-3.0-or-later. -->

# Production circuits

These are the zKølang circuits the system runs in production, not illustrations. Each
is a registered program: the kernel proves it, and NOX gates settlement on its verifier
key. A circuit becomes real here by being registered, its commitment and key fixed and
referenced on chain, the same way a deployed contract is real by its address.

The verifier key is
`keccak256(0x01 ‖ commit ‖ log2N_le ‖ trace_width_le ‖ rate_le ‖ periodic_root)`, the
registration rate is three, and the trace width is fifty one. Recompute any row with
`cargo run --example circuit_key -- circuits/<name>.zkl`.

| Circuit | Role | Program commitment | Verifier key |
|---|---|---|---|
| `transfer.zkl` | value conservation of a shielded transfer | `6fb93b25…d28faaff` | `e33e71a0…0571506e` |
| `confidential_tx.zkl` | two in two out transfer, outputs range proven | `96c0ebf9…d03016b1` | `839c6f5d…4e331f45` |
| `commitment.zkl` | hiding commitment to a note value | `121c1aed…86ed5e8e` | `e2004d8d…a3ae91a1` |
| `nullifier.zkl` | spend marker binding key and position | `aac4b462…50abf6c4f`| `49514ea6…90e2d6f35`|
| `range8.zkl` | byte range proof for an amount | `b157561a…0785e1bc8` | `7a587789…326d68dc` |

A circuit's key is what the registry stores and what a proof is checked against. Change
one byte of a circuit and its commitment and key change, so a registered key pins an
exact circuit. The confidential value family above shares the fifty one wide VM, so one
verifier and one fee schedule serve them all; a new circuit joins by registering its key.
