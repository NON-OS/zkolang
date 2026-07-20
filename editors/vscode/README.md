<!-- NONOS. AGPL-3.0-or-later. -->

# zKolang for VS Code

Syntax highlighting and editor configuration for zKølang source (`.zkl`). The extension
contributes the language under scope `source.zkolang` with the TextMate grammar in
`syntaxes/`, line comments, and bracket handling.

## Install from source

```
cd editors/vscode
npx @vscode/vsce package
code --install-extension zkolang-0.1.0.vsix
```

Open any `.zkl` file and it highlights. Publishing to the Marketplace with
`vsce publish` puts it one search away, which is how a language reaches the editors
where people first try it.
