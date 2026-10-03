/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The size of a type written out: one for each of its parts, however often the type
 * table shares one. A type is stored once, so a type made by doubling another stays cheap
 * to store while its size, and the work of laying it out, grows exponentially.
 */

use super::{GenArg, TyId, TyKind, Types};

impl Types {
    /** Whether `t`, written out, takes at most `budget` parts; what it takes is spent. */
    pub fn within(&self, t: TyId, budget: &mut usize) -> bool {
        if *budget == 0 {
            return false;
        }
        *budget -= 1;
        match self.kind(t) {
            TyKind::Tuple(ts) => ts.iter().all(|&e| self.within(e, budget)),
            TyKind::Array(e, _) => self.within(*e, budget),
            TyKind::Adt(_) => {
                let args = self.adt(t).map(|a| a.args.as_slice()).unwrap_or(&[]);
                args.iter().all(|g| match g {
                    GenArg::Type(e) => self.within(*e, budget),
                    GenArg::Const(_) => true,
                })
            }
            _ => true,
        }
    }
}
