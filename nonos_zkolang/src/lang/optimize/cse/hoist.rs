/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Hoist the repeated pure subtrees of one expression into bindings. */

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::vec::Vec;

use super::ids::Ids;
use super::rewrite::Rewrite;
use super::scan::scan;
use crate::lang::parse::{Expr, Stmt};

/**
 * Hoist repeated pure subexpressions of `e` into let statements appended to `pre`,
 * largest first and, among equals, first in pre-order, returning the rewritten expression.
 * Terminates because each hoist replaces two or more occurrences with one variable, which
 * is not itself hoistable.
 */
pub(super) fn hoist(mut e: Expr, pre: &mut Vec<Stmt>, ctr: &mut usize) -> Expr {
    let mut ids = Ids::default();
    loop {
        let Some((target, shared)) = choose(&e, &mut ids) else {
            return e;
        };
        let name = format!("$cse{}", *ctr);
        *ctr += 1;
        pre.push(Stmt::Let(name.clone(), shared));
        let mut rewrite = Rewrite {
            ids: &mut ids,
            target,
            name,
        };
        e = rewrite.go(&e).0;
    }
}

/** The id and the subtree of the largest candidate that occurs at least twice. */
fn choose(e: &Expr, ids: &mut Ids) -> Option<(usize, Expr)> {
    let (mut order, mut counts) = (Vec::new(), BTreeMap::new());
    scan(e, ids, &mut order, &mut counts);
    let mut best = None;
    let mut best_size = 1;
    for c in &order {
        if c.size > best_size && counts.get(&c.id).is_some_and(|&n| n >= 2) {
            best_size = c.size;
            best = Some((c.id, c.expr.clone()));
        }
    }
    best
}
