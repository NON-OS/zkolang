; Syntax highlighting queries for zKolang. GitHub and editors that speak tree-sitter
; use these to colour source; the capture names are the standard highlight groups.
(comment) @comment
(string) @string
(number) @number

[
  "include" "const" "fn" "let" "input" "secret" "output" "assert" "for" "in" "if" "else"
] @keyword

["inv" "sel"] @function.builtin

(fn_def (identifier) @function)
(call (identifier) @function.call)

["+" "-" "*" "/" "==" "!=" "<" "<=" ">" ">=" "&&" "||" "!" ".."] @operator
["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["," ";"] @punctuation.delimiter

(identifier) @variable
