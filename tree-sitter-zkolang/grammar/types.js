/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/* Types, generic arguments, and the paths types and expressions share. */

const { commaSep, commaSep1, INT_TYPES } = require('./util');

module.exports = {
  _type: $ => choice($.labelled_type, $._bare_type),
  labelled_type: $ => seq(field('label', choice('public', 'secret')), field('type', $._bare_type)),
  _bare_type: $ => choice($.unit_type, $.tuple_type, $.array_type, $.reference_type,
    $.generic_type, $._path),
  primitive_type: _ => choice('field', 'bool', ...INT_TYPES),
  unit_type: _ => seq('(', ')'),
  tuple_type: $ => seq('(', $._type, ',', commaSep($._type), ')'),
  array_type: $ => seq('[', field('element', $._type), ';', field('length', $._const_arg), ']'),
  reference_type: $ => seq('&', 'mut', field('type', $._type)),
  generic_type: $ => prec(1, seq(field('type', $._path), field('type_arguments', $.type_arguments))),
  type_arguments: $ => seq(token(prec(1, '<')), commaSep1(choice($._type, $.integer, $.block)), '>'),
  _const_arg: $ => choice($.integer, $.identifier, $.block),

  _path: $ => choice($.identifier, $.self, $.crate, $.super, $.scoped_identifier,
    $.primitive_type),
  scoped_identifier: $ => seq(field('path', $._path), '::', field('name', $.identifier)),
  self: _ => choice('self', 'Self'),
  crate: _ => 'crate',
  super: _ => 'super',
};
