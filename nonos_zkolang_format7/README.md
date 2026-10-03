<!-- NONOS. AGPL-3.0-or-later. -->

# nonos_zkolang_format7

zKølang runs proven in STARKs format 7, the format the
[STARKs](https://github.com/NON-OS/STARKs) verifiers read, and checked by `nox_verify`,
the `no_std` verifier a STARKs gate links.

## What is proven

The step AIR of the run, unchanged: its 62 constraints, its boundaries, and the program's
wiring as periodic columns. The two-round prover of format 7 needs a column committed after
the copy challenges; the step AIR reads none, so its last column is committed then and
nothing is filled between the rounds. The proof is made at STARKs' query shape A, 19
queries after a 28-bit grind, on the launch transcript with its DEEP and folding grinds,
with five extra blowup bits. Every column is blinded from the prover's seed, which needs a
trace of at least 2^10 rows.

## What a verifier pins

`zkolang statement <file> --edition 2026 --out dir` writes, from the program alone:

- `program.bin`, the program image: the constraints as a tape of additions, subtractions
  and multiplications, and the boundaries, each a value or a public word.
- `statement.txt`, one `name value` line each: `program_hash` (the image's keccak256),
  `periodic_root`, `params` (the parameter identity a proof's header names), the numbers
  of the AIR's shape, `words` (how many public words), and `head`, the five words every
  run of the program begins with.

The public words of a run are `head` (the program commitment's four limbs and the trace
length), then the public input slots, then the output slots, as `zkolang abi` lays them
out. A verifier computes them from values it expects; a proof never supplies them.

## Proving and verifying

```sh
zkolang prove sum.zkl --edition 2026 --public 12 --secret 5 --out run
zkolang verify sum.zkl --edition 2026 --proof run/proof.bin --public 12 --outputs 149
```

From Rust: `statement`, `prove`, `words` and `verify`. In a page, the module
[`nonos_zkolang_wasm`](../nonos_zkolang_wasm) verifies against the same files:

```js
import { load, verify } from './zkolang-verify.mjs';
const m = await load(wasmBytes);
const { ok, reason } = verify(m, { image, statement, proof, slots: [12n, 149n] });
```

## The fixture

[`fixture`](fixture) holds a proof of [`fixture/sum.zkl`](fixture/sum.zkl) on public 12 and
secret 5, with its image, statement and words. `tests/fixture.rs` holds them to what this
crate makes now, and CI has the browser module verify them. A change that moves any of
them writes them again with `cargo run --release -p nonos_zkolang_format7 --example
fixture`, whose fixed seed reproduces the bytes.

## Not here

An on-chain verifier is generated from a statement's image in the contracts repository;
none is built for a zKølang program here.
