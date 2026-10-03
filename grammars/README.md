<!-- NONOS. AGPL-3.0-or-later. -->

# zKolang syntax grammar

`zkolang.tmLanguage.json` is a TextMate grammar for zKølang source (`.zkl`), both
editions: nested block comments and doc comments, attributes, strings and their escapes,
integer literals with a radix and a type suffix, every keyword and reserved word of the
lexer, the primitive types, and the edition 2025 words. `grammar_tests` holds it to the
lexer's keyword table, and the VS Code extension ships the same file.

## Editors

Point any TextMate-compatible editor at the grammar with scope `source.zkolang`
and file type `zkl`. In VS Code, a minimal extension that contributes this grammar
lights up `.zkl` files.

## GitHub language stats

GitHub attributes files to a language with Linguist, which only counts languages
in its database. To have `.zkl` recognized as zKolang and shown in a repository's
language bar, a definition has to be added upstream in `github/linguist`:

- an entry in `lib/linguist/languages.yml` with the name, `type: programming`,
  the `.zkl` extension, a color, and `tm_scope: source.zkolang`,
- this grammar registered under `vendor/grammars`,
- sample `.zkl` files under `samples/`.

Linguist accepts a new language once it is used across enough public repositories,
so recognition follows adoption. Until then the grammar highlights the language in
editors, and the examples in this repository are the samples a submission needs.
