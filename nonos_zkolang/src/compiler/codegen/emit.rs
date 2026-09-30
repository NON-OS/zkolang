/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The allocation walk: each instruction's operands brought into registers, then it. A
 * constant, input or advice value is read where it is first used, not where it stands,
 * so its register is taken for no longer than it must be.
 */

use alloc::vec::Vec;

use super::machine::{CodegenError, Machine, Origin};
use super::op_of::op_of;
use super::state::Cg;
use crate::compiler::ssa::{Inst, Ssa, V};
use crate::isa::Op;

/** The machine program of `ssa`, whose gadgets are expanded. */
pub fn codegen(ssa: &Ssa) -> Result<Machine, CodegenError> {
    let mut cg = Cg::new(ssa);
    for (i, &inst) in ssa.insts.iter().enumerate() {
        let v = V(u32::try_from(i).unwrap_or(u32::MAX));
        cg.at = v;
        if inst.is_gadget() {
            return Err(CodegenError::Unexpanded(v));
        }
        let read = matches!(inst, Inst::Const(_) | Inst::Input(_) | Inst::Advice(_));
        /* A value no register reads, which cannot fail, is for the witness alone. */
        let pure = matches!(
            inst,
            Inst::Add(..) | Inst::Sub(..) | Inst::Mul(..) | Inst::Eq(..)
        );
        if read || pure && cg.uses.get(i).is_none_or(|u| u.is_empty()) {
            continue;
        }
        let operands: Vec<V> = inst.operands().collect();
        let mut regs: Vec<(V, u8)> = Vec::with_capacity(operands.len());
        for &o in &operands {
            let r = cg.ensure(o, &operands)?;
            regs.push((o, r));
        }
        for &o in &operands {
            if let Some(p) = cg.passed.get_mut(o.index()) {
                *p += 1;
            }
        }
        for &o in &operands {
            if cg.next_use(o).is_none() {
                cg.release(o);
            }
        }
        let d = match inst.is_effect() {
            true => 0,
            false => cg.take_reg(&[])?,
        };
        let r = |x: V| regs.iter().find(|(o, _)| *o == x).map_or(0, |(_, r)| *r);
        let op = op_of(inst, d, &r).ok_or(CodegenError::Unexpanded(v))?;
        cg.push(op, Origin::Def(v));
        if !inst.is_effect() {
            cg.bind(v, d);
            if cg.next_use(v).is_none() {
                cg.release(v);
            }
        }
    }
    cg.push(Op::Halt, Origin::Halt);
    Ok(cg.out)
}
