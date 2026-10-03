; zKølang highlights: the node names of grammar.js mapped to the capture names editors
; and GitHub share. Later patterns win over earlier ones in most editors, so the general
; captures come first.

(identifier) @variable
((identifier) @type (#match? @type "^[A-Z]"))
((identifier) @constant (#match? @constant "^[A-Z][A-Z0-9_]+$"))

(line_comment) @comment
(block_comment) @comment
((line_comment) @comment.documentation (#match? @comment.documentation "^//[/!]([^/]|$)"))
((block_comment) @comment.documentation (#match? @comment.documentation "^/[*][*!]([^*/]|$)"))

(string) @string
(integer) @number
(boolean) @boolean
(primitive_type) @type.builtin
(self) @variable.builtin
(wildcard) @variable.builtin
(attribute (identifier) @attribute)

(type_parameters (identifier) @type)
(const_parameter name: (identifier) @constant)
(struct_item name: (identifier) @type)
(enum_item name: (identifier) @type)
(type_item name: (identifier) @type)
(enum_variant name: (identifier) @constructor)
(const_item name: (identifier) @constant)
(legacy_const name: (identifier) @constant)
(mod_item name: (identifier) @module)
(field_declaration name: (identifier) @property)
(field_expression field: (identifier) @property)
(field_initializer field: (identifier) @property)
(shorthand_field_initializer (identifier) @property)
(field_pattern name: (identifier) @property)
(parameter pattern: (identifier) @variable.parameter)
(legacy_parameters (identifier) @variable.parameter)

(function_item name: (identifier) @function)
(legacy_function name: (identifier) @function)
(call_expression function: (identifier) @function.call)
(call_expression function: (scoped_identifier name: (identifier) @function.call))
(call_expression function: (field_expression field: (identifier) @function.method))
(generic_function function: (identifier) @function.call)
(declassify_expression "declassify" @function.builtin)

[
  "as" "assert" "const" "else" "enum" "fn" "for" "if" "impl" "in" "let" "limit"
  "match" "mod" "mut" "return" "struct" "type" "use" "while"
  "public" "secret" "include" "input" "witness" "output" "reveal" "prove"
] @keyword
(crate) @keyword
(super) @keyword
(visibility) @keyword
(break_expression) @keyword
(continue_expression) @keyword

[
  "+" "-" "*" "/" "%" "^" "!" "&" "|" "&&" "||" "<<" ">>" "=" "+=" "-=" "*=" "/=" "%="
  "^=" "&=" "|=" "<<=" ">>=" "==" "!=" "<" ">" "<=" ">=" ".." "..=" "->" "=>" "::"
] @operator
["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["," ";" ":" "." "#"] @punctuation.delimiter
