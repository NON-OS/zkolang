<!-- NONOS. AGPL-3.0-or-later. -->

# tree-sitter-zkolang

A tree-sitter grammar for zKølang. tree-sitter is the incremental parser GitHub and
editors such as Neovim, Helix and Zed use for highlighting and code navigation.

It parses edition 2026 as [`SPEC.md`](../SPEC.md) section 3 gives it, and the top-level
forms of edition 2025 beside it: includes, untyped constants and functions, and the
statements of a program's body. The rules are split by part under [`grammar/`](grammar),
and [`src/scanner.c`](src/scanner.c) reads block comments, which nest. The grammar is
lenient where the compiler is not: it accepts a chain of comparisons, which the compiler
refuses with E0103, and it does not bound nesting.

## Build and test

```
npm install
npx tree-sitter generate
npx tree-sitter test
python3 ../scripts/check_grammar.py node_modules/.bin/tree-sitter
```

`tree-sitter test` runs the parse trees pinned under [`test/corpus`](test/corpus). The
last command is what CI runs: the corpus, the highlight queries, and a parse of every
program in the repository that marks no lexical or syntax error, and of every program in
the README, each with no error node.

`queries/highlights.scm` maps syntax nodes to the highlight groups editors share.

## GitHub recognition

GitHub attributes source with Linguist, which counts only languages defined upstream in
`github/linguist`. A submission there adds an entry to `languages.yml` for zKølang with
the `.zkl` extension and the scope `source.zkolang` ([`linguist-entry.yml`](linguist-entry.yml)),
and includes sample `.zkl` files. Linguist takes a new language once it is in real use
across public repositories, so recognition follows adoption.
