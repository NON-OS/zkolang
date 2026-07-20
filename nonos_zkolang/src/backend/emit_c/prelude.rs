/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The C field prelude: Goldilocks add, subtract, multiply, and inverse, each
//! reducing a 128-bit intermediate so operands stay canonical.

pub(super) const PRELUDE: &str = "\
#include <stdio.h>
#include <stdlib.h>
typedef unsigned long long u64;
typedef unsigned __int128 u128;
static const u64 P = 0xFFFFFFFF00000001ULL;
static u64 fadd(u64 a, u64 b) { return (u64)(((u128)a + (u128)b) % P); }
static u64 fsub(u64 a, u64 b) { return (u64)(((u128)a + (u128)P - (u128)b) % P); }
static u64 fmul(u64 a, u64 b) { return (u64)(((u128)a * (u128)b) % P); }
static u64 finv(u64 a) {
    u64 r = 1, b = a, e = P - 2;
    while (e) { if (e & 1) r = fmul(r, b); b = fmul(b, b); e >>= 1; }
    return r;
}
";
