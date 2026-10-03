/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Inlining a call: the callee's body lowered in a frame of its own, under the caller's guard. */

use alloc::vec;
use alloc::vec::Vec;

use super::cx::{Frame, Lower};
use super::error::{LowerError, L};
use super::layout::slots;
use crate::compiler::ssa::V;
use crate::compiler::tir::FnId;

/** How deep inlined calls may nest; the checker rejects recursion, so this is a backstop. */
const MAX_FRAMES: usize = 256;

impl<'p> Lower<'p> {
    /** Inline `f` on `args`: its result, and each parameter's final slots. */
    pub(super) fn inline(&mut self, f: FnId, args: Vec<Vec<V>>) -> L<(Vec<V>, Vec<Vec<V>>)> {
        let p = self.p;
        let body = p.fns.get(f.0 as usize).ok_or(LowerError::NoMain)?;
        if self.frames.len() >= MAX_FRAMES {
            return Err(LowerError::Unsupported("calls nested this deep", body.span));
        }
        let zero = self.b.konst(0);
        let locals = body
            .locals
            .iter()
            .map(|l| vec![zero; slots(&p.types, l.ty)])
            .collect();
        let ret = vec![zero; slots(&p.types, body.ret)];
        self.frames.push(Frame {
            f,
            locals,
            ret,
            loops: Vec::new(),
        });
        for (param, arg) in body.params.iter().zip(args) {
            self.set_local(param.local, arg.clone());
            let ty = body.locals.get(param.local.0 as usize).map(|l| l.ty);
            if let Some(ty) = ty {
                self.bind(&param.pat, &arg, ty)?;
            }
        }
        let g0 = self.g;
        let tail = self.block(&body.body)?;
        let frame = self.frames.pop().ok_or(LowerError::NoMain)?;
        let g = self.g;
        let result = match tail.len() == frame.ret.len() {
            true => tail
                .iter()
                .zip(&frame.ret)
                .map(|(&t, &r)| self.b.sel(g, t, r))
                .collect(),
            false => frame.ret.clone(),
        };
        let finals = body
            .params
            .iter()
            .map(|q| {
                frame
                    .locals
                    .get(q.local.0 as usize)
                    .cloned()
                    .unwrap_or_default()
            })
            .collect();
        self.g = g0;
        Ok((result, finals))
    }
}
