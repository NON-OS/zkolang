/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Structural identities. Equal expressions get one id and unequal ones different ids, so
 * finding and counting a repeated subtree compares numbers instead of whole trees.
 */

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::pure::children;
use super::tag::tag;
use crate::lang::parse::Expr;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    Num(u64),
    Var(String),
    Node(u8, Vec<usize>),
    Call(String, Vec<usize>),
    Index(usize, usize, usize),
    Block(Vec<(Vec<String>, usize)>, usize),
}

/** The ids handed out so far, one per distinct expression. */
#[derive(Default)]
pub(super) struct Ids {
    table: BTreeMap<Key, usize>,
}

impl Ids {
    /**
     * The id of `e`, given the ids of the children `children` lists for it, in order. An
     * index or a block lists none and is keyed by its whole content.
     */
    pub(super) fn node(&mut self, e: &Expr, kids: Vec<usize>) -> usize {
        let key = match e {
            Expr::Num(v) => Key::Num(*v),
            Expr::Var(n) => Key::Var(n.clone()),
            Expr::Call(f, _) => Key::Call(f.clone(), kids),
            Expr::Index(a, b, at) => Key::Index(self.id(a), self.id(b), *at),
            Expr::Block(locals, r) => {
                let locals = locals
                    .iter()
                    .map(|(n, v)| (n.clone(), self.id(v)))
                    .collect();
                Key::Block(locals, self.id(r))
            }
            other => Key::Node(tag(other), kids),
        };
        let next = self.table.len();
        *self.table.entry(key).or_insert(next)
    }

    /** The id of `e`, walking all of it. */
    pub(super) fn id(&mut self, e: &Expr) -> usize {
        let kids = children(e).into_iter().map(|c| self.id(c)).collect();
        self.node(e, kids)
    }
}
