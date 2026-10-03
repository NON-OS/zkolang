/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/* Blocks and statements, and the expressions that end in a block. */

const { commaSep, PREC } = require('./util');

module.exports = {
  block: $ => seq('{', repeat($._statement), optional($._expression), '}'),
  _statement: $ => choice(';', $.let_declaration, $.assert_statement, $.expression_statement),
  let_declaration: $ => seq('let', field('pattern', $._pattern),
    optional(seq(':', field('type', $._type))), '=', field('value', $._expression), ';'),
  assert_statement: $ => seq('assert', field('condition', $._expression),
    optional(seq(',', field('message', $.string))), ';'),
  expression_statement: $ => choice(seq($._expression, ';'), prec(1, $._block_like)),

  _block_like: $ => choice($.block, $.if_expression, $.match_expression, $.for_expression,
    $.while_expression),
  if_expression: $ => prec.right(seq('if', field('condition', $._expression),
    field('consequence', $.block),
    optional(field('alternative', seq('else', choice($.block, $.if_expression)))))),
  match_expression: $ => seq('match', field('value', $._expression),
    field('body', $.match_block)),
  match_block: $ => seq('{', repeat($.match_arm),
    optional(alias($.last_match_arm, $.match_arm)), '}'),
  match_arm: $ => seq($._arm_head, choice(seq(field('value', $._expression), ','),
    prec(1, field('value', $._block_like)))),
  last_match_arm: $ => seq($._arm_head, field('value', $._expression)),
  _arm_head: $ => seq(field('pattern', $._pattern),
    optional(seq('if', field('guard', $._expression))), '=>'),
  for_expression: $ => seq('for', field('pattern', $._pattern), 'in',
    field('value', choice($.range_expression, $._expression)), field('body', $.block)),
  range_expression: $ => prec.left(PREC.range, seq(field('start', $._expression),
    choice('..', '..='), field('end', $._expression))),
  while_expression: $ => seq('while', field('condition', $._expression), 'limit',
    field('limit', $._const_arg), field('body', $.block)),
  declassify_expression: $ => seq('declassify', '(', $._expression, ')'),
  return_expression: $ => prec.right(seq('return', optional($._expression))),
  break_expression: _ => 'break',
  continue_expression: _ => 'continue',
  arguments: $ => seq('(', commaSep($._expression), ')'),
};
