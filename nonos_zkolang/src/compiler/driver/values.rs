/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The reference run of a program's `main`: its arguments rebuilt from leaf values, where
 * an enum's leaves are its slots and must be the layout of a value of it.
 */

use alloc::vec::Vec;

use super::abi::AbiError;
use super::built::Built;
use super::value_of::value_of;
use crate::compiler::interp::{Failure, Interp, Value};
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::Label;
use crate::compiler::tir::{FnId, TProgram};

/**
 * The arguments of `f` from its public and secret leaf values; `Err` names an enum's
 * leaves that lay out no value of it, and whether they are secret.
 */
pub(super) fn args_of(
    p: &TProgram,
    f: FnId,
    public: &[i128],
    secret: &[i128],
) -> Result<Vec<Value>, (AbiError, bool)> {
    let (mut pubs, mut secs) = ((public, 0), (secret, 0));
    let Some(body) = p.fns.get(f.0 as usize) else {
        return Ok(Vec::new());
    };
    let mut out = Vec::with_capacity(body.params.len());
    for param in &body.params {
        let Some(l) = body.locals.get(param.local.0 as usize) else {
            continue;
        };
        let (at, is_secret) = match l.labels.whole() {
            Some(Label::Secret) => (&mut secs, true),
            _ => (&mut pubs, false),
        };
        let v = value_of(&p.types, l.ty, at)
            .map_err(|position| (AbiError::Range { position }, is_secret))?;
        out.push(v);
    }
    Ok(out)
}

/** The reference run of `b`'s `main` on `args`, and the type of its result. */
pub(super) fn reference(
    b: &Built,
    main: FnId,
    args: Vec<Value>,
) -> Option<(Result<Value, Failure>, TyId)> {
    let f = b.program.fns.get(main.0 as usize)?;
    Some((
        Interp::new(&b.program, 1 << 30).call(main, args, f.span),
        f.ret,
    ))
}
