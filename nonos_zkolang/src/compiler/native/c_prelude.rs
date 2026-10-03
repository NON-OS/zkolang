/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What the C target adds to the field prelude of edition 2025: how a failed constraint
 * ends the run, the bound a range check tests, a bit of a value, and the 64-bit division
 * hint.
 */

pub(super) const EXTRA: &str = "\
#define FAIL(at) do { fprintf(stderr, \"the run fails at instruction %d\\n\", at); return 3; } while (0)
static inline int below(u64 x, int n) { return n < 64 && (x >> n) == 0; }
static inline u64 bit(u64 x, int k) { return k < 64 ? (x >> k) & 1 : 0; }
static inline u64 div64(u64 alo, u64 ahi, u64 blo, u64 bhi, int part) {
    u64 x = (alo & 0xFFFFFFFFULL) | (ahi << 32), y = (blo & 0xFFFFFFFFULL) | (bhi << 32);
    u64 q = y ? x / y : 0, r = y ? x % y : x, v = part < 2 ? q : r;
    return part % 2 == 0 ? v & 0xFFFFFFFFULL : v >> 32;
}
";

/**
 * What the C target's inputs and result rest on: a signed value read, and one printed after
 * a space unless it is the first.
 */
pub(super) const IO: &str = "\
typedef __int128 i128;
static inline int read_value(const char *s, i128 *v) {
    int neg = *s == '-', n = 0;
    i128 x = 0;
    for (s += neg; *s; s++, n++) {
        if (*s < '0' || *s > '9' || n >= 30) return 0;
        x = x * 10 + (*s - '0');
    }
    if (!n) return 0;
    *v = neg ? -x : x;
    return 1;
}
static void print_value(i128 x) {
    static int first = 1;
    char b[48];
    int i = 47, neg = x < 0;
    unsigned __int128 u = neg ? -(unsigned __int128)x : (unsigned __int128)x;
    b[i] = 0;
    do { b[--i] = (char)('0' + (int)(u % 10)); u /= 10; } while (u);
    if (neg) b[--i] = '-';
    printf(\"%s%s\", first ? \"\" : \" \", b + i);
    first = 0;
}
";
