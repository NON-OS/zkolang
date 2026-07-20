; Syntax highlighting queries for zKolang. GitHub and editors that speak tree-sitter
; use these to colour source; the capture names are the standard highlight groups.
(comment) @comment
(string) @string
(number) @number

[
  "include" "const" "fn" "let" "input" "public" "secret" "witness" "output" "reveal"
  "assert" "prove" "for" "in" "if" "else" "match"
] @keyword

["inv" "sel"] @function.builtin

(fn_def (identifier) @function)
(call (identifier) @function.call)

["+" "-" "*" "/" "==" "!=" "<" "<=" ">" ">=" "&&" "||" "!" ".." "=>"] @operator
["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["," ";"] @punctuation.delimiter

(identifier) @variable
