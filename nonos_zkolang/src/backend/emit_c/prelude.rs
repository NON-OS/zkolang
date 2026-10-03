/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The C field prelude: Goldilocks add, subtract, multiply, and inverse, each reducing a
 * 128-bit intermediate so operands stay canonical, and a strict reader for one input.
 */

pub(super) const PRELUDE: &str = "\
#include <stdio.h>
#include <stdlib.h>
typedef unsigned long long u64;
typedef unsigned __int128 u128;
static const u64 P = 0xFFFFFFFF00000001ULL;
static inline u64 fadd(u64 a, u64 b) { return (u64)(((u128)a + (u128)b) % P); }
static inline u64 fsub(u64 a, u64 b) { return (u64)(((u128)a + (u128)P - (u128)b) % P); }
static inline u64 fmul(u64 a, u64 b) { return (u64)(((u128)a * (u128)b) % P); }
static inline u64 finv(u64 a) {
    u64 r = 1, b = a, e = P - 2;
    while (e) { if (e & 1) r = fmul(r, b); b = fmul(b, b); e >>= 1; }
    return r;
}
static inline int read_input(const char *s, u64 *v) {
    u64 x = 0;
    if (!*s) return 0;
    for (; *s; s++) {
        if (*s < '0' || *s > '9') return 0;
        u128 y = (u128)x * 10 + (u128)(*s - '0');
        if (y >= P) return 0;
        x = (u64)y;
    }
    *v = x;
    return 1;
}
";
