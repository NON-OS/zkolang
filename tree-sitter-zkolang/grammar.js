/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*
 * zKølang for tree-sitter. Edition 2026 as SPEC.md section 3 gives it, one file of rules
 * per part, and the top-level forms of edition 2025 beside it.
 */

const lexical = require('./grammar/lexical');
const items = require('./grammar/items');
const types = require('./grammar/types');
const statements = require('./grammar/statements');
const expressions = require('./grammar/expressions');
const patterns = require('./grammar/patterns');
const legacy = require('./grammar/legacy');

module.exports = grammar({
  name: 'zkolang',
  word: $ => $.identifier,
  externals: $ => [$.block_comment],
  extras: $ => [/\s/, $.line_comment, $.block_comment],
  conflicts: $ => [
    [$._expression, $.struct_expression],
    [$.parameters, $.legacy_parameters],
  ],
  rules: {
    source_file: $ => repeat(choice($.attribute_item, $._item, $._legacy_item)),
    ...lexical, ...items, ...types, ...statements, ...expressions, ...patterns, ...legacy,
  },
});
