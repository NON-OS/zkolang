/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Types as a diagnostic writes them: `u8`, `(field, bool)`, `[u8; 4]`, `Pair<u8>`, `_`. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::{GenArg, TyId, TyKind, Types};

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
            TyKind::Infer(_) => String::from("_"),
            TyKind::Adt(_) => match self.adt(t) {
                Some(a) if a.args.is_empty() => a.name.clone(),
                Some(a) => {
                    let args: Vec<String> = a.args.iter().map(|g| self.arg(*g)).collect();
                    format!("{}<{}>", a.name, args.join(", "))
                }
                None => String::from("{unknown}"),
            },
        }
    }

    /** How a message names the generic argument `g`. */
    pub fn arg(&self, g: GenArg) -> String {
        match g {
            GenArg::Type(t) => self.display(t),
            GenArg::Const(n) => format!("{n}"),
        }
    }
}
