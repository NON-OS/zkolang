/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Registering a program's functions and constants, each under an id, and reporting the
 * items this build does not check yet (E0904), so none of them passes as checked.
 */

use super::cx::{ConstInfo, FnInfo, Sema, State};
use super::defs::{DefId, DefKind};
use crate::compiler::syntax::ast::ItemKind;
use crate::compiler::tir::{ConstId, FnId};

impl<'a> Sema<'a> {
    /** Give every function and constant an id; report the items not checked yet. */
    pub(super) fn register(&mut self) {
        for i in 0..self.defs.defs.len() {
            let Some(def) = self.defs.defs.get(i) else {
                continue;
            };
            let (id, module, item) = (
                DefId(u32::try_from(i).unwrap_or(u32::MAX)),
                def.parent,
                def.item,
            );
            let (Some(module), Some(item)) = (module, item) else {
                continue;
            };
            match (&item.kind, def.kind) {
                (ItemKind::Fn(f), DefKind::Fn) => {
                    let fid = FnId(u32::try_from(self.fns.len()).unwrap_or(u32::MAX));
                    self.fns.push(FnInfo::new(id, f, module, None));
                    self.fn_of.insert(id, fid);
                }
                (ItemKind::Const(c), DefKind::Const) => {
                    let cid = ConstId(u32::try_from(self.consts.len()).unwrap_or(u32::MAX));
                    self.consts.push(ConstInfo {
                        def: id,
                        decl: c,
                        module,
                        ty: None,
                        state: State::Unchecked,
                    });
                    self.const_of.insert(id, cid);
                }
                (ItemKind::TypeAlias(_) | ItemKind::Struct(_) | ItemKind::Enum(_), _) => {}
                (ItemKind::Mod(m), _) => {
                    if m.body.is_none() {
                        self.unloaded(&m.name);
                    }
                }
                _ => self.not_yet(item),
            }
        }
    }
}
