/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What the Python target adds to the field prelude of edition 2025: the bound a range
 * check tests, a bit of a value, and the 64-bit division hint; and the entry that runs
 * the module from a command line as the C target runs.
 */

/** What the Python target adds to the field prelude of edition 2025. */
pub(super) const EXTRA: &str = "

def _below(x, n):
    return n < 64 and x >> n == 0


def _bit(x, k):
    return (x >> k) & 1 if k < 64 else 0


def _div64(alo, ahi, blo, bhi, part):
    x, y = (alo & 0xFFFFFFFF) | (ahi << 32), (blo & 0xFFFFFFFF) | (bhi << 32)
    q, r = divmod(x, y) if y else (0, x)
    v = q if part < 2 else r
    return v & 0xFFFFFFFF if part % 2 == 0 else v >> 32
";

/**
 * The entry of `python3 module.py`: one argument per input leaf, the leaves of the result
 * printed on one line; exit status 1 for any other command line and 3 when a constraint
 * fails, as the C target's.
 */
pub(super) const MAIN: &str = "

if __name__ == \"__main__\":
    import sys
    try:
        print(*run([int(a) for a in sys.argv[1:]]))
    except ValueError as e:
        sys.exit(str(e))
    except Unprovable as e:
        print(f\"the run fails at {e}\", file=sys.stderr)
        sys.exit(3)
";
