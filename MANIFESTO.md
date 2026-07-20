<!-- NONOS. AGPL-3.0-or-later. -->

# zKølang

A language writes down a way of thinking. zKølang writes down one belief: that a
computation should be able to prove what it did without asking anyone to trust who ran
it. Everything in the language follows from that, and so does its voice.

## What it is

A program reads what is public, holds what is private as a witness, and reveals only
what it chooses. The whole run is proven by a transparent STARK with no trusted setup,
so there is no ceremony to believe in and no operator to trust. The proof is the
authority. A verifier checks it with mathematics, not with faith in the prover.

Write it plainly or write it in its own register. The two spellings are the same
language:

```
witness key;
public position;
reveal nullifier(key, position);
prove balance == 0;
```

`witness` is `secret`, `public` is `input`, `reveal` is `output`, `prove` is `assert`.
The plain words are exact. The others say what the plain words mean.

## What it believes

Privacy is not a feature you switch on. It is a property the proof enforces, at the
boundary, for everyone, whether or not they were watching. A shielded amount is not
hidden by policy; it is unrecoverable from the proof by construction. A nullifier does
not promise no double spend; it makes one collide. The circuit does not describe the
rules. The circuit is the rules.

Verification belongs to the person who received the claim, not the person who made it.
So the proof is small, the field is a machine word, no setup is trusted, and the same
program that proves also runs as native code. Nothing here asks you to take our word.

## Its style

The language is small on purpose. Bounded loops, no heap, no hidden control flow, a
register machine you can hold in your head. A constraint that is honest about its cost
is the only constraint worth proving, and a program you can read line by line is the
only program worth trusting. Beauty here is not decoration. It is the absence of the
places a lie could hide.

Own your keys. Own your compute. Prove it.
