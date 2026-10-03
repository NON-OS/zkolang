/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*
 * Edition 2025: textual includes, untyped constants and expression functions, and the
 * statements of a program's body. They stand at the top level, where edition 2026 has
 * only items, so the words they start with name nothing else there.
 */

const { commaSep } = require('./util');

module.exports = {
  _legacy_item: $ => choice($.include_directive, $.legacy_const, $.legacy_function,
    $._legacy_statement),
  include_directive: $ => seq('include', field('path', $.string), ';'),
  legacy_const: $ => seq('const', field('name', $.identifier), '=',
    field('value', choice($.integer, $.array_expression)), ';'),
  legacy_function: $ => seq('fn', field('name', $.identifier),
    field('parameters', $.legacy_parameters), choice(
      seq('=', field('body', $._expression), ';'), field('body', $.block))),
  legacy_parameters: $ => seq('(', commaSep($.identifier), ')'),

  _legacy_statement: $ => choice($.let_declaration, $.assert_statement,
    $.input_declaration, $.output_statement, $.prove_statement, $.legacy_for),
  input_declaration: $ => seq(choice('input', 'public', 'secret', 'witness'),
    field('name', $.identifier), ';'),
  output_statement: $ => seq(choice('output', 'reveal'), $._expression, ';'),
  prove_statement: $ => seq('prove', $._expression, ';'),
  legacy_for: $ => seq('for', field('pattern', $.identifier), 'in',
    field('value', $.range_expression), '{', repeat($._legacy_statement), '}'),
};
