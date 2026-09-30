/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * 64-bit integers (section 6): the low and high 32-bit halves of the two's-complement
 * pattern, each below `2^32`. Arithmetic works on the halves with carries and borrows;
 * a signed result overflows when its operands' signs say it cannot have its sign.
 */

mod arith;
mod bits;
mod builtin;
mod cmp;
mod conv;
mod div;
mod div_signed;
mod halves;
mod mul;
mod narrow;
mod ops;
mod pow;
mod unary;
