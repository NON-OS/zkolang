/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Starting the walk of a function's body. */

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use super::flow::Flow;
use super::shape::Shape;
use super::summary::Summary;
use super::taint::Taint;
use crate::compiler::tir::{TFn, TProgram};

impl<'p> Flow<'p> {
    /** A walk of `f`'s body, its locals all public until set. */
    pub(super) fn new(
        program: &'p TProgram,
        summaries: &'p [Option<Summary>],
        f: &'p TFn,
    ) -> Flow<'p> {
        let env = f
            .locals
            .iter()
            .map(|l| Shape::of(l.ty, Taint::PUBLIC, &program.types))
            .collect();
        let ref_ret = f
            .params
            .iter()
            .filter(|p| p.by_ref)
            .map(|p| (p.local.0 as usize, Shape::Leaf(Taint::PUBLIC)))
            .collect();
        Flow {
            program,
            summaries,
            f,
            env,
            pc: Taint::PUBLIC,
            returned: Shape::Leaf(Taint::PUBLIC),
            ret_guard: Taint::PUBLIC,
            ref_ret,
            exits: Vec::new(),
            needs_public: 0,
            report: true,
            reported: BTreeSet::new(),
            found: Vec::new(),
        }
    }
}
