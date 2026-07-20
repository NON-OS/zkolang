<!-- NONOS Operating System. Copyright (C) 2026 NONOS Contributors. AGPL-3.0-or-later. -->

# zKolang

A verifiable-compute language by NONOS. zKolang compiles a small straight-line
language to a register virtual machine, and the machine's execution trace is
proven by a transparent post-quantum STARK with no trusted setup. A proof attests
that a specific program ran on specific public inputs and produced specific public
outputs, not that some bag of valid rows happens to exist.

The step AIR binds every operand to the live register file, so register reuse is
invisible to soundness and a forged row cannot pass. The public statement, the
program commitment, its trace length, and its public inputs and outputs are bound
into the proof, and a per-program verifier key ties the commitment to the program's
wiring at a fixed rate, which is what lets a pay-to-prove market register and
challenge a program by its commitment alone.

## Workspace

```
nonos-stark/           transparent STARK primitives: field, Poseidon, FRI, DEEP
nonos_zkolang/         the language, the VM, the step AIR, and the prover binding
nonos_zkolang_proofs/  the host proof suite behind the documentation
docs/                  language, machine, AIR, economics, reference, recipes
paper/                 the research paper
```

The language and the STARK travel together here so the repository builds and proves
on its own, with no dependency outside `blake3`.

## Quickstart

```
cargo test -p nonos_zkolang_proofs
cargo run -p nonos_zkolang_proofs --release --example measure
```

The proof suite covers the AIR tamper set, register binding, public input and
output soundness, the language end to end including functions, the verifier-key
binding at the registration rate, and the fee model. The measurement example prints
the trace shapes and fees the documentation reports.

## A first program

```
fn sq(x) = x * x;
input x;
let y = sq(x) + 5;
output y;        // proves y = x^2 + 5 for the committed public x and y
```

The language has `let`, `assert`, `input`, `secret`, `output`, field arithmetic
with division and negation, equality and not-equal, the branchless `sel` and its
`if`/`else` sugar, bounded `for` loops unrolled at compile time, constant tables
read by a compile-time index, and functions inlined hygienically at each call. See
[docs](nonos_zkolang/docs/02-language.md).

## Examples

Programs written in the language, under [examples](examples): a cubed input, a
polynomial by Horner's rule, Fibonacci, a factorial, a power, a geometric series, a
dot product and a matrix-vector product over arrays, a range proof, a round schedule
over a constant table, a two-to-one hash-tree node, and `mimc.zkl`, a full MiMC hash
whose output is proven equal to the field reference. Every example is compiled, run,
and proven by the suite in `nonos_zkolang_proofs`.

## Targets

The compiled program is target-independent, so one `.zkl` source has three
back-ends: the proven STARK trace, native **C** (`to_c`), and **Python**
(`to_python`). All three compute over the same field, and the C back-end is checked
end to end against the proof, native binary versus prover. The syntax grammar for
editors and GitHub is under [grammars](grammars).

## Documentation

Start with the [overview](nonos_zkolang/docs/01-overview.md) and
[the language](nonos_zkolang/docs/02-language.md), then the
[reference](nonos_zkolang/docs/07-reference.md) and the
[recipes](nonos_zkolang/docs/10-recipes.md). The economics, the recursion ABI, and
the on-chain contract spec are under the same directory.

## License

AGPL-3.0-or-later. See [LICENSE](LICENSE).
