/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/* Patterns, as `let`, parameters, `for` and `match` arms bind them. */

const { commaSep } = require('./util');

module.exports = {
  _pattern: $ => choice($.wildcard, $._path, $.mut_pattern, $.integer, $.boolean,
    $.negative_literal, $.range_pattern, $.tuple_pattern, $.tuple_struct_pattern,
    $.struct_pattern, $.slice_pattern, $.or_pattern),
  wildcard: _ => '_',
  mut_pattern: $ => seq('mut', $.identifier),
  negative_literal: $ => seq('-', $.integer),
  range_pattern: $ => seq(choice($.integer, $.negative_literal), '..=',
    choice($.integer, $.negative_literal)),
  tuple_pattern: $ => seq('(', commaSep($._pattern), ')'),
  tuple_struct_pattern: $ => seq(field('type', $._path), '(', commaSep($._pattern), ')'),
  struct_pattern: $ => seq(field('type', $._path), '{',
    commaSep(choice($.field_pattern, $.remaining_field_pattern)), '}'),
  field_pattern: $ => choice(field('name', $.identifier),
    seq(field('name', $.identifier), ':', field('pattern', $._pattern))),
  remaining_field_pattern: _ => '..',
  slice_pattern: $ => seq('[', commaSep($._pattern), ']'),
  or_pattern: $ => prec.left(seq($._pattern, '|', $._pattern)),
};
