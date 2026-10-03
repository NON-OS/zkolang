/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/* Identifiers, literals and comments (SPEC section 2). */

const { INT_TYPES } = require('./util');

const SUFFIX = `(${INT_TYPES.join('|')})?`;

module.exports = {
  identifier: _ => /[A-Za-z_][A-Za-z0-9_]*/,

  integer: _ => token(new RegExp(
    `(0x[0-9a-fA-F][0-9a-fA-F_]*|0o[0-7][0-7_]*|0b[01][01_]*|[0-9][0-9_]*)${SUFFIX}`)),

  boolean: _ => choice('true', 'false'),

  string: _ => token(seq('"', repeat(choice(/[^"\\\n\r]/, /\\["\\nt0]/)), '"')),

  line_comment: _ => token(seq('//', /[^\n\r]*/)),

  _literal: $ => choice($.integer, $.boolean, $.string),
};
