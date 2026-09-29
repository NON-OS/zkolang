/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The bounds that keep compilation itself finite. */

/**
 * The largest number of iterations a single loop may unroll to, a fail-fast guard
 * before the trace-length cap catches anything larger at prove time.
 */
pub(crate) const MAX_UNROLL: u64 = 65_536;

/**
 * The largest number of instructions a program may unroll to. A single loop is
 * bounded by MAX_UNROLL, but nested loops multiply, so total emission is capped
 * here to keep a hostile program from exhausting memory during compilation. The
 * bound sits well above any provable trace, so the prove-time cap still gives the
 * tighter answer for programs that merely will not fit.
 */
pub(crate) const MAX_OPS: usize = 1 << 20;

/**
 * The deepest a chain of inlined calls may nest, which turns a recursive call into
 * a compile error rather than a non-terminating inline.
 */
pub(crate) const MAX_INLINE: usize = 256;

/** The most calls a program may inline in all, since a call that emits nothing still costs work. */
pub(crate) const MAX_INLINES: usize = 1 << 20;

/** The most loop iterations a program may unroll in all, counting loops whose bodies emit nothing. */
pub(crate) const MAX_ITERATIONS: usize = 1 << 20;

/**
 * The deepest expression lowering may recurse. The parser bounds how deep one expression
 * nests, but inlining stacks function bodies on top of each other, so lowering keeps its
 * own bound and stops with an error instead of exhausting the host's stack.
 */
pub(crate) const MAX_EXPR_DEPTH: usize = 1024;
