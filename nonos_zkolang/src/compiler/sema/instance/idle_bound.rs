/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The names a function binds anywhere: its parameters, and every `let`, `for` and `match`
 * arm pattern of its body. A path whose first name is one of them may name a local.
 */

use alloc::collections::BTreeSet;
use alloc::string::String;

use crate::compiler::syntax::ast::{Block, ExprKind, FnDecl, Param, PatKind, Pattern, StmtKind};

/** The names `decl` binds. */
pub(super) fn bound_names(decl: &FnDecl) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for p in &decl.params {
        if let Param::Typed { pat, .. } = p {
            pattern(pat, &mut out);
        }
    }
    lets(&decl.body, &mut out);
    decl.body.each_expr(&mut |e| match &e.kind {
        ExprKind::Block(b) | ExprKind::While { body: b, .. } => lets(b, &mut out),
        ExprKind::For { pat, body, .. } => {
            pattern(pat, &mut out);
            lets(body, &mut out);
        }
        ExprKind::If(branches, last) => {
            branches.iter().for_each(|br| lets(&br.block, &mut out));
            last.iter().for_each(|b| lets(b, &mut out));
        }
        ExprKind::Match { arms, .. } => arms.iter().for_each(|a| pattern(&a.pat, &mut out)),
        _ => {}
    });
    out
}

/** The names the `let` statements of `b` itself bind. */
fn lets(b: &Block, out: &mut BTreeSet<String>) {
    for s in &b.stmts {
        if let StmtKind::Let { pat, .. } = &s.kind {
            pattern(pat, out);
        }
    }
}

/** The names the pattern `p` binds. */
fn pattern(p: &Pattern, out: &mut BTreeSet<String>) {
    match &p.kind {
        PatKind::Bind { name, .. } => {
            out.insert(name.name.clone());
        }
        PatKind::Tuple(ps) | PatKind::Array(ps) | PatKind::Or(ps) | PatKind::TupleStruct(_, ps) => {
            ps.iter().for_each(|q| pattern(q, out));
        }
        PatKind::Struct { fields, .. } => {
            for f in fields {
                match &f.pat {
                    Some(q) => pattern(q, out),
                    None => {
                        out.insert(f.name.name.clone());
                    }
                }
            }
        }
        PatKind::Range { .. }
        | PatKind::Lit { .. }
        | PatKind::Path(_)
        | PatKind::Wild
        | PatKind::Error => {}
    }
}
