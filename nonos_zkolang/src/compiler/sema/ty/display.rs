/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Types as a diagnostic writes them: `u8`, `(field, bool)`, `[u8; 4]`, `{integer}`. */

use alloc::format;
use alloc::string::String;

use super::{TyId, TyKind, Types};

impl Types {
    /** How a message names type `t`. */
    pub fn display(&self, t: TyId) -> String {
        match self.kind(t) {
            TyKind::Error => String::from("{unknown}"),
            TyKind::Never => String::from("!"),
            TyKind::Unit => String::from("()"),
            TyKind::Bool => String::from("bool"),
            TyKind::Field => String::from("field"),
            TyKind::Int(i) => String::from(i.name()),
            TyKind::Tuple(ts) => {
                let mut s = String::from("(");
                for (n, &e) in ts.iter().enumerate() {
                    if n > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&self.display(e));
                }
                if ts.len() == 1 {
                    s.push(',');
                }
                s.push(')');
                s
            }
            TyKind::Array(e, n) => format!("[{}; {n}]", self.display(*e)),
            TyKind::Var(_) => String::from("{integer}"),
            TyKind::Adt(_) => self
                .adt(t)
                .map_or_else(|| String::from("{unknown}"), |a| a.name.clone()),
        }
    }
}
