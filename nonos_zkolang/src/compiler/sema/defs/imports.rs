/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Resolving `use` imports to a fixed point (section 4.4). An import may name what another
 * import brings, so every round resolves what it can, until a round changes nothing.
 * A glob is applied again each round, as the module it names may gain names. What is
 * left unresolved is reported.
 */

use alloc::vec;

use super::PendingImport;
use super::{Defs, PathError};
use crate::compiler::diag::Diagnostics;

/** Where an import stands in the fixed point. */
#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Pending,
    Resolved,
    Failed,
}

impl<'a> Defs<'a> {
    /** Bind every import of `imports` in its module, and report those that fail. */
    pub fn resolve_imports(&mut self, imports: &[PendingImport<'a>], diags: &mut Diagnostics) {
        let mut states = vec![State::Pending; imports.len()];
        loop {
            let mut changed = false;
            for (imp, state) in imports.iter().zip(states.iter_mut()) {
                let again = *state == State::Pending || (*state == State::Resolved && imp.glob);
                if !again {
                    continue;
                }
                match self.resolve_import(imp) {
                    Ok(def) if imp.glob => {
                        changed |= *state == State::Pending;
                        *state = State::Resolved;
                        changed |= self.apply_glob(imp, def, diags);
                    }
                    Ok(def) => {
                        *state = State::Resolved;
                        changed = true;
                        self.bind_import(imp, def, diags);
                    }
                    Err(PathError::Unresolved(_)) => {}
                    Err(e) => {
                        *state = State::Failed;
                        self.report_import(imp, e, diags);
                    }
                }
            }
            if !changed {
                break;
            }
        }
        for (imp, state) in imports.iter().zip(states) {
            if state == State::Pending {
                let e = self
                    .resolve_import(imp)
                    .err()
                    .unwrap_or(PathError::Unresolved(0));
                self.report_import(imp, e, diags);
            }
        }
    }
}
