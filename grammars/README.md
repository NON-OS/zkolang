<!-- NONOS. AGPL-3.0-or-later. -->

# zKolang syntax grammar

`zkolang.tmLanguage.json` is a TextMate grammar for zKolang source (`.zkl`). It
gives editors and GitHub the rules to highlight the language.

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
