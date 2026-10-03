/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/* Lists of the grammar: items between commas, a trailing comma allowed. */

/* Zero or more `rule`, comma separated, with an optional trailing comma. */
function commaSep(rule) {
  return optional(commaSep1(rule));
}

/* One or more `rule`, comma separated, with an optional trailing comma. */
function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)), optional(','));
}

/* The integer types, which are keywords and also start a path such as `u32::MAX`. */
const INT_TYPES = ['u8', 'u16', 'u32', 'u64', 'i8', 'i16', 'i32', 'i64', 'usize'];

/* Binding strength of the operators, loosest first (SPEC section 3). */
const PREC = {
  range: 0, assign: 1, or: 2, and: 3, compare: 4, bitor: 5, bitxor: 6, bitand: 7,
  shift: 8, add: 9, mul: 10, cast: 11, unary: 12, postfix: 13,
};

module.exports = { commaSep, commaSep1, INT_TYPES, PREC };
