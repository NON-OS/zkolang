/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Constant items (section 11): each checked and evaluated on first use, so a constant may
 * use one declared after it. A constant met again while it is being evaluated is part of
 * a cycle (E0502).
 */

use alloc::format;

use super::cx::{Sema, State};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::interp::Value;
use crate::compiler::tir::{ConstId, TConst};

impl<'a> Sema<'a> {
    /** The value of constant `c`, or `None` if it fails, which is reported. */
    pub(crate) fn const_value(&mut self, c: ConstId) -> Option<Value> {
        let info = self.consts.get(c.0 as usize)?;
        let (m, decl, def) = (info.module, info.decl, info.def);
        match &info.state {
            State::Done((_, v)) => return Some(v.clone()),
            State::Failed => return None,
            State::Checking => {
                let d = Diagnostic::error(
                    Code::CONST_CYCLE,
                    format!("the constant `{}` depends on itself", decl.name.name),
                    decl.name.span,
                    "needed to evaluate itself",
                );
                self.diags.push(d);
                self.set_const(c, State::Failed);
                return None;
            }
            State::Unchecked => {}
        }
        self.set_const(c, State::Checking);
        let ty = self.const_ty(c);
        let out = self.within(decl.name.span, |s| {
            let (init, locals) = s.check_const_expr(m, &decl.value, ty)?;
            let v = s.const_eval(&init, locals.len())?;
            Some((
                TConst {
                    def,
                    name: decl.name.name.clone(),
                    ty,
                    init,
                    locals,
                    span: decl.name.span,
                },
                v,
            ))
        });
        match out {
            Some((t, v)) => {
                self.set_const(c, State::Done((t, v.clone())));
                Some(v)
            }
            None => {
                self.set_const(c, State::Failed);
                None
            }
        }
    }
}
