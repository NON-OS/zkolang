/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Instances of generic functions (section 10.5): one per template and list of arguments,
 * made by the calls that need them and checked like any function, each made only within
 * the bounds `fn_bound` keeps.
 */

use super::cx::{Sema, State};
use crate::compiler::sema::ty::GenArg;
use crate::compiler::source::Span;
use crate::compiler::tir::FnId;

impl<'a> Sema<'a> {
    /**
     * The instance of the template `t` for `args`, made for the call at `at` in `from` if
     * it is new; `None` once the instance would pass the bounds (E0700).
     */
    pub(crate) fn instance_fn(
        &mut self,
        t: FnId,
        args: &[GenArg],
        from: Option<FnId>,
        at: Span,
    ) -> Option<FnId> {
        let key = (t, args.to_vec());
        if let Some(&f) = self.instances.get(&key) {
            return Some(f);
        }
        let mut inst = self.fns.get(t.0 as usize)?.clone();
        if !self.instance_bounded((inst.def, &inst.decl.name.name), args, from, at) {
            return None;
        }
        (inst.template, inst.args, inst.origin) = (false, args.to_vec(), Some((from, at)));
        (inst.sig, inst.body, inst.clean) = (None, State::Unchecked, false);
        let id = FnId(u32::try_from(self.fns.len()).ok()?);
        self.fns.push(inst);
        self.instances.insert(key, id);
        Some(id)
    }
}
