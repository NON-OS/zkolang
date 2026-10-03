/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/* Expressions, by the precedence of SPEC section 3, loosest first. */

const { commaSep, PREC } = require('./util');

const BINARY = [
  [PREC.or, '||'], [PREC.and, '&&'], [PREC.bitor, '|'], [PREC.bitxor, '^'],
  [PREC.bitand, '&'], [PREC.shift, choice('<<', '>>')], [PREC.add, choice('+', '-')],
  [PREC.mul, choice('*', '/', '%')],
  [PREC.compare, choice('==', '!=', '<', '<=', '>', '>=')],
];

const ASSIGN = ['=', '+=', '-=', '*=', '/=', '%=', '^=', '&=', '|=', '<<=', '>>='];

module.exports = {
  _expression: $ => choice($.assignment_expression, $.binary_expression,
    $.type_cast_expression, $.unary_expression, $.reference_expression, $.call_expression,
    $.index_expression, $.field_expression, $.generic_function, $._path, $._literal,
    $.struct_expression, $.parenthesized_expression, $.unit_expression,
    $.tuple_expression, $.array_expression, $._block_like, $.declassify_expression,
    $.return_expression, $.break_expression, $.continue_expression),

  assignment_expression: $ => prec.right(PREC.assign, seq(field('left', $._expression),
    field('operator', choice(...ASSIGN)), field('right', $._expression))),
  binary_expression: $ => choice(...BINARY.map(([p, op]) => prec.left(p, seq(
    field('left', $._expression), field('operator', op), field('right', $._expression))))),
  type_cast_expression: $ => prec.left(PREC.cast, seq(field('value', $._expression), 'as',
    field('type', $._path))),
  unary_expression: $ => prec(PREC.unary, seq(choice('-', '!'), $._expression)),
  reference_expression: $ => prec(PREC.unary, seq('&', 'mut', field('value', $._expression))),

  call_expression: $ => prec(PREC.postfix, seq(field('function', $._expression),
    field('arguments', $.arguments))),
  index_expression: $ => prec(PREC.postfix, seq($._expression, '[', $._expression, ']')),
  field_expression: $ => prec(PREC.postfix, seq(field('value', $._expression), '.',
    field('field', choice($.identifier, $.integer)))),
  generic_function: $ => prec(PREC.postfix, seq(field('function', choice($._path,
    $.field_expression)), '::', field('type_arguments', $.type_arguments))),

  struct_expression: $ => prec.dynamic(-1, seq(field('name', choice($._path,
    $.generic_function)), field('body', $.field_initializer_list))),
  field_initializer_list: $ => seq('{', commaSep(choice($.shorthand_field_initializer,
    $.field_initializer)), '}'),
  shorthand_field_initializer: $ => $.identifier,
  field_initializer: $ => seq(field('field', $.identifier), ':', field('value', $._expression)),

  parenthesized_expression: $ => seq('(', $._expression, ')'),
  unit_expression: _ => seq('(', ')'),
  tuple_expression: $ => seq('(', $._expression, ',', commaSep($._expression), ')'),
  array_expression: $ => seq('[', choice(commaSep($._expression),
    seq($._expression, ';', field('length', $._const_arg))), ']'),
};
