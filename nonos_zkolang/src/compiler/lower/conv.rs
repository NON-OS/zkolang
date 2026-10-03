/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Conversions (sections 7.8 and 7.9). `checked_from` checks the source's integer value
 * against the target's range: a `field` or unsigned source is non-negative, so a signed
 * target needs it below `2^(N-1)`. `wrapping_from` keeps the low `N` bits. Bits are the
 * pattern's, and a `field` element's 64 bits must be canonical to convert back.
 */

use super::cx::Lower;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::V;

impl<'p> Lower<'p> {
    /** `T::checked_from(x)`, `x` of kind `from`, `T` of kind `to`. */
    pub(super) fn checked_from(&mut self, x: V, from: &TyKind, to: &TyKind) -> V {
        let signed_src = matches!(from, TyKind::Int(s) if s.signed());
        match to {
            TyKind::Bool => {
                self.require_below(x, 1);
                return self.guarded(x, 0);
            }
            TyKind::Int(t) if t.signed() && signed_src => self.check_int(x, *t),
            TyKind::Int(t) if t.signed() => self.require_below(x, t.bits() - 1),
            TyKind::Int(t) => self.require_below(x, t.bits()),
            _ => {}
        }
        x
    }

    /** `T::wrapping_from(x)`: the low `N` bits of `x`'s pattern, as `T`. */
    pub(super) fn wrapping_from(&mut self, x: V, from: &TyKind, to: &TyKind) -> V {
        let TyKind::Int(t) = *to else {
            return x;
        };
        let bits = match from {
            TyKind::Int(s) if s.signed() => {
                let lift = self.b.konst(1i128 << 32);
                let u = self.b.add(x, lift);
                self.low_bits(u, t.bits(), 33)
            }
            TyKind::Int(s) => self.low_bits(x, t.bits(), s.bits()),
            _ => self.low_bits(x, t.bits(), 64),
        };
        self.pattern_value(&bits, t)
    }
}
