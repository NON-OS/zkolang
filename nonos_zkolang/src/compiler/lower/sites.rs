/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The sites lowering records (section 15.2): each an expression's span and the calls
 * inlined to reach it, the outermost first, kept once each. Lowering an expression makes
 * its site the one the instructions it writes carry, and the enclosing one again after.
 */

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::cx::Lower;
use crate::compiler::source::Span;
use crate::compiler::ssa::Site;
use crate::compiler::tir::FnId;

/** Where code comes from: a span, and the functions inlined to reach it, outermost first. */
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Place {
    pub span: Span,
    pub chain: Vec<FnId>,
}

/** The sites of a program: `Site(k)` for `k` from 1 is the `k`th place. */
#[derive(Clone, Debug, Default)]
pub struct Sites {
    pub places: Vec<Place>,
    index: BTreeMap<Place, Site>,
}

impl Sites {
    /** The site of `place`, added if new. */
    pub fn intern(&mut self, place: Place) -> Site {
        if let Some(&s) = self.index.get(&place) {
            return s;
        }
        self.places.push(place.clone());
        let s = Site(u32::try_from(self.places.len()).unwrap_or(u32::MAX));
        self.index.insert(place, s);
        s
    }

    /** The place of site `s`; `None` for `Site(0)`, the program's own. */
    pub fn place(&self, s: Site) -> Option<&Place> {
        self.places.get(usize::try_from(s.0).ok()?.checked_sub(1)?)
    }
}

impl<'p> Lower<'p> {
    /** Make `span`, in the calls being lowered, the site written from now; the one before. */
    pub(super) fn enter(&mut self, span: Span) -> Site {
        let chain = self.frames.iter().map(|f| f.f).collect();
        let site = self.sites.intern(Place { span, chain });
        core::mem::replace(&mut self.b.site, site)
    }
}
