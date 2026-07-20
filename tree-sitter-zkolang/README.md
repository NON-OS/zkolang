<!-- NONOS. AGPL-3.0-or-later. -->

# tree-sitter-zkolang

A tree-sitter grammar for zKølang. tree-sitter is the incremental parser GitHub uses
for syntax highlighting and code navigation, so a grammar here is what lights `.zkl`
files up on GitHub and in editors built on tree-sitter.

## Build

The grammar source is `grammar.js`. Generate and test the parser with the tree-sitter
CLI, which reads it and emits the C parser:

```
npx tree-sitter generate
npx tree-sitter parse ../examples/cube.zkl
npx tree-sitter test
```

`queries/highlights.scm` maps syntax nodes to highlight groups.

## GitHub recognition

GitHub attributes source with Linguist, which counts only languages defined upstream in
`github/linguist`. A submission there adds an entry to `languages.yml` for zKølang with
the `.zkl` extension and the scope `source.zkolang`, references this grammar, and
includes sample `.zkl` files. Linguist takes a new language once it is in real use
across public repositories, so recognition follows adoption, not the other way around.
This grammar is the parser that submission points at.
