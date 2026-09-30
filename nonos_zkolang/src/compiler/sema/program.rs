/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The checked program: its functions and constants in the order of their ids, and its
 * `main`. A function or constant that failed to check stands as an error, so the ids stay
 * valid; a program with errors is not run.
 */

use alloc::vec::Vec;

use super::cx::{Sema, State};
use super::defs::Defs;
use super::info::FnInfo;
use super::program_failed::{failed_const, failed_fn};
use crate::compiler::interp::Value;
use crate::compiler::tir::{TConst, TProgram};

impl<'a> Sema<'a> {
    /** The program as checked. */
    pub(crate) fn program(&self) -> TProgram {
        let fns = self.fns.iter().map(|info| match &info.body {
            State::Done(b) => b.clone(),
            _ => failed_fn(info),
        });
        let consts: Vec<(TConst, Value)> = self
            .consts
            .iter()
            .map(|info| match &info.state {
                State::Done((c, v)) => (c.clone(), v.clone()),
                _ => (failed_const(info), Value::Unit),
            })
            .collect();
        let main = self
            .defs
            .modules
            .get(&Defs::ROOT)
            .and_then(|m| m.names.get("main"))
            .and_then(|b| self.fn_of.get(&b.def))
            .copied();
        let (consts, values) = consts.into_iter().unzip();
        TProgram {
            types: self.types.clone(),
            fns: fns.collect(),
            consts,
            values,
            main,
            tests: Vec::new(),
            cost_quiet: self.fns.iter().map(|info| self.cost_quiet(info)).collect(),
        }
    }
}

impl<'a> Sema<'a> {
    /**
     * Whether the cost warnings (section 15.3) are silent for the function `info`: one
     * outside the program's own crate, or inside an item that allows `cost`.
     */
    fn cost_quiet(&self, info: &FnInfo<'a>) -> bool {
        let at = info.decl.name.span;
        let allowed = self.allowed.iter().any(|(s, lint)| {
            *lint == "cost" && s.file == at.file && s.lo <= at.lo && at.hi <= s.hi
        });
        allowed || self.defs.crate_of(info.module) != Defs::ROOT
    }
}
