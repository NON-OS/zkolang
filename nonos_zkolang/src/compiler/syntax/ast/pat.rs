/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Patterns as written. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{Ident, Lit, NodeId, Path};
use crate::compiler::source::Span;

/** A pattern. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Pattern {
    pub id: NodeId,
    pub kind: PatKind,
    pub span: Span,
}

/** A field in a struct pattern: `name: pat` or the shorthand `name`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FieldPat {
    pub name: Ident,
    pub pat: Option<Pattern>,
    pub span: Span,
}

/** The shapes a pattern takes. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PatKind {
    /** `_` */
    Wild,
    /**
     * `name` or `mut name`. A lone name that resolves to a unit variant or a constant is
     * turned into `Path` by name resolution.
     */
    Bind { name: Ident, mutable: bool },
    /** A literal, with a leading minus for a negative integer. */
    Lit { lit: Lit, negative: bool },
    /** `lo..=hi`, integer literals. */
    Range { lo: Box<Pattern>, hi: Box<Pattern> },
    /** `(a, b, ...)` */
    Tuple(Vec<Pattern>),
    /** A unit variant, a unit struct or a constant named by a path. */
    Path(Path),
    /** `Path(a, b)` */
    TupleStruct(Path, Vec<Pattern>),
    /** `Path { a, b: p, .. }` */
    Struct {
        path: Path,
        fields: Vec<FieldPat>,
        rest: bool,
    },
    /** `[a, b, c]` */
    Array(Vec<Pattern>),
    /** `p | q` */
    Or(Vec<Pattern>),
    /** A pattern that failed to parse; the parser has reported it. */
    Error,
}
