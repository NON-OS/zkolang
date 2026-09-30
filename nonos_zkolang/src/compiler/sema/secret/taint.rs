/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What a value's label depends on (section 13.1): a secret, or the labels of parts of the
 * enclosing function's parameters, which are not known until a call gives them. Each
 * part is a *slot* (see `slots`). A slot past the 128th is taken as secret, which is
 * safe: it can only add errors.
 */

/** The label of a value, as a secret flag and the parameter slots whose labels it takes. */
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Taint {
    pub secret: bool,
    pub slots: u128,
}

impl Taint {
    pub const PUBLIC: Taint = Taint {
        secret: false,
        slots: 0,
    };
    pub const SECRET: Taint = Taint {
        secret: true,
        slots: 0,
    };

    /** The label of parameter slot `k`, whatever the caller passes. */
    pub fn slot(k: usize) -> Taint {
        match u32::try_from(k).ok().and_then(|k| 1u128.checked_shl(k)) {
            Some(bit) => Taint {
                secret: false,
                slots: bit,
            },
            None => Taint::SECRET,
        }
    }

    /** The least upper bound of two labels. */
    pub fn join(self, o: Taint) -> Taint {
        Taint {
            secret: self.secret || o.secret,
            slots: self.slots | o.slots,
        }
    }

    /** Whether the value is public whatever the arguments. */
    pub fn is_public(self) -> bool {
        !self.secret && self.slots == 0
    }

    /** The label this one stands for once slot `k` has label `args(k)`. */
    pub fn apply(self, args: &dyn Fn(usize) -> Taint) -> Taint {
        let mut out = Taint {
            secret: self.secret,
            slots: 0,
        };
        for k in 0..128 {
            if self.slots & (1u128 << k) != 0 {
                out = out.join(args(k));
            }
        }
        out
    }
}
