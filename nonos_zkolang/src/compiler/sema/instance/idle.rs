/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Names in a generic function no use instantiates (section 5.5). Such a function is not
 * checked, but a path in an expression of it must name an item where it is written unless
 * its first name is a generic parameter, `Self`, a primitive type or a name the function
 * binds; a path through a type item names a member, which an instance settles (E0200).
 */

use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::Sema;
use super::super::defs::{DefId, DefKind};
use super::idle_bound::bound_names;
use crate::compiler::syntax::ast::{ExprKind, FnDecl, GenericParam, Path, PathRoot};
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::FnId;

impl<'a> Sema<'a> {
    /** Report the paths that name nothing in each template no instance was made of. */
    pub(crate) fn check_idle_templates(&mut self) {
        let used: BTreeSet<FnId> = self.instances.keys().map(|(t, _)| *t).collect();
        let idle: Vec<(DefId, &'a FnDecl, &'a [GenericParam])> = (self.fns.iter().enumerate())
            .filter(|(i, f)| f.template && !used.contains(&FnId(*i as u32)))
            .map(|(_, f)| (f.module, f.decl, f.impl_generics))
            .collect();
        for (m, decl, outer) in idle {
            let mut skip = bound_names(decl);
            for g in outer.iter().chain(&decl.generics) {
                skip.insert(String::from(match g {
                    GenericParam::Type(p) => p.name.as_str(),
                    GenericParam::Const { name, .. } => name.name.as_str(),
                }));
            }
            let mut paths: Vec<Path> = Vec::new();
            decl.body.each_expr(&mut |e| match &e.kind {
                ExprKind::Path(p) | ExprKind::Struct { path: p, .. } => paths.push(p.clone()),
                _ => {}
            });
            for p in &paths {
                self.idle_path(m, p, &skip);
            }
        }
    }

    /** Report `p`, written in module `m`, if it names nothing and depends on no name in `skip`. */
    fn idle_path(&mut self, m: DefId, p: &Path, skip: &BTreeSet<String>) {
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let Some(&first) = names.first() else {
            return;
        };
        let primitive = IntTy::from_name(first).is_some() || first == "field" || first == "bool";
        let local = p.root == PathRoot::Plain && (skip.contains(first) || primitive);
        if p.root == PathRoot::SelfType || local || self.defs.prelude_variant(first).is_some() {
            return;
        }
        let Err(e) = self.defs.resolve(m, p.root, &names) else {
            return;
        };
        let owner = names
            .split_last()
            .and_then(|(_, pre)| self.defs.resolve(m, p.root, pre).ok());
        let kind = owner.and_then(|d| self.defs.get(d)).map(|d| d.kind);
        if !matches!(kind, Some(DefKind::Struct | DefKind::Enum | DefKind::Alias)) {
            let near: Vec<&str> = skip.iter().map(String::as_str).collect();
            self.report_path(m, p, e, &near);
        }
    }
}
