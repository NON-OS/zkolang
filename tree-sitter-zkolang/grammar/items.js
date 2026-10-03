/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/* Items: functions, structs, enums, aliases, constants, modules, uses and impls. */

const { commaSep, commaSep1 } = require('./util');

module.exports = {
  attribute_item: $ => seq('#', optional('!'), '[', $.attribute, ']'),
  attribute: $ => seq($.identifier, optional(choice(
    seq('(', commaSep(choice(seq($.identifier, optional(seq('=', $._literal))), $._literal)), ')'),
    seq('=', $._literal)))),

  _item: $ => seq(optional($.visibility), choice(
    $.function_item, $.struct_item, $.enum_item, $.type_item, $.const_item,
    $.mod_item, $.use_declaration, $.impl_item)),
  visibility: _ => 'pub',

  function_item: $ => seq(optional('const'), 'fn', field('name', $.identifier),
    optional(field('type_parameters', $.type_parameters)),
    field('parameters', $.parameters),
    optional(seq('->', field('return_type', $._type))), field('body', $.block)),
  type_parameters: $ => seq('<', commaSep1(choice($.identifier, $.const_parameter)), '>'),
  const_parameter: $ => seq('const', field('name', $.identifier), ':', field('type', $._type)),
  parameters: $ => prec.dynamic(1, seq('(', commaSep(choice($.self_parameter, $.parameter)),
    ')')),
  self_parameter: _ => seq(optional(seq('&', 'mut')), 'self'),
  parameter: $ => seq(field('pattern', $._pattern), ':', field('type', $._type)),

  struct_item: $ => seq('struct', field('name', $.identifier),
    optional(field('type_parameters', $.type_parameters)), choice(
      field('body', $.field_declaration_list),
      seq(field('body', $.ordered_field_declaration_list), ';'), ';')),
  field_declaration_list: $ => seq('{', commaSep($.field_declaration), '}'),
  field_declaration: $ => seq(repeat($.attribute_item), optional($.visibility),
    field('name', $.identifier), ':', field('type', $._type)),
  ordered_field_declaration_list: $ => seq('(', commaSep(
    seq(repeat($.attribute_item), optional($.visibility), field('type', $._type))), ')'),

  enum_item: $ => seq('enum', field('name', $.identifier),
    optional(field('type_parameters', $.type_parameters)),
    field('body', seq('{', commaSep($.enum_variant), '}'))),
  enum_variant: $ => seq(repeat($.attribute_item), field('name', $.identifier),
    optional(field('body', choice(
      $.ordered_field_declaration_list, $.field_declaration_list)))),

  type_item: $ => seq('type', field('name', $.identifier),
    optional(field('type_parameters', $.type_parameters)), '=', field('type', $._type), ';'),
  const_item: $ => seq('const', field('name', $.identifier), ':', field('type', $._type),
    '=', field('value', $._expression), ';'),
  mod_item: $ => seq('mod', field('name', $.identifier), choice(';',
    field('body', seq('{', repeat(choice($.attribute_item, $._item)), '}')))),

  use_declaration: $ => seq('use', field('argument', $._use_tree), ';'),
  _use_tree: $ => choice($._path, $.use_as_clause, $.use_wildcard, $.scoped_use_list),
  use_as_clause: $ => seq(field('path', $._path), 'as', field('alias', $.identifier)),
  use_wildcard: $ => seq($._path, '::', '*'),
  scoped_use_list: $ => seq(field('path', $._path), '::',
    field('list', seq('{', commaSep($._use_tree), '}'))),

  impl_item: $ => seq('impl', optional(field('type_parameters', $.type_parameters)),
    field('type', $._type), field('body', $.declaration_list)),
  declaration_list: $ => seq('{', repeat(choice($.attribute_item,
    seq(optional($.visibility), $.function_item))), '}'),
};
