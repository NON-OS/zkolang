/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Each gadget rejects the other witnesses a prover might try: the bits of `v + p` for a
 * `field` element `v`, and a quotient one smaller with a remainder one divisor larger.
 * The honest witness is accepted first, so a rejection is the gadget's doing.
 */

use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::witness;
use nonos_zkolang::compiler::ssa::{Hint, Inst};

use crate::ssa_adversary::forged;
use crate::ssa_small::{accepts, compile, hint_of};

#[test]
fn field_bits_admit_only_the_canonical_representative() {
    let c = compile(&|b| {
        let v = b.emit(Inst::Input(0));
        b.emit(Inst::FieldBit(v, 3))
    });
    let inputs = [Fp::from_u64(5), Fp::ZERO];
    assert!(accepts(&c, &witness(&c, &inputs).expect("runs")));
    let big = 5u128 + u128::from(nonos_stark::field::P);
    let forge = |s: usize, _: &[Fp]| match hint_of(&c, s) {
        Some(Hint::Bit(_, k)) => Some(Fp::from_u64(((big >> k) & 1) as u64)),
        _ => None,
    };
    assert!(!accepts(&c, &forged(&c, &inputs, &forge)));
}

#[test]
fn division_admits_only_the_true_quotient_and_remainder() {
    let c = compile(&|b| {
        let (a, d) = (b.emit(Inst::Input(0)), b.emit(Inst::Input(1)));
        b.emit(Inst::RangeCheck(a, 8));
        b.emit(Inst::RangeCheck(d, 8));
        let q = b.emit(Inst::Quot(a, d, 8));
        let r = b.emit(Inst::Rem(a, d, 8));
        let k = b.konst(1000);
        let qk = b.mul(q, k);
        b.add(qk, r)
    });
    let inputs = [Fp::from_u64(20), Fp::from_u64(6)];
    assert!(accepts(&c, &witness(&c, &inputs).expect("runs")));
    for (dq, dr) in [(-1i64, 6i64), (1, -6)] {
        let forge = |s: usize, _: &[Fp]| match hint_of(&c, s) {
            Some(Hint::Quot(..)) => Some(Fp::from_u64((3 + dq) as u64)),
            Some(Hint::Rem(..)) => Some(
                Fp::from_u64(2)
                    + if dr < 0 {
                        -Fp::from_u64(6)
                    } else {
                        Fp::from_u64(6)
                    },
            ),
            _ => None,
        };
        assert!(!accepts(&c, &forged(&c, &inputs, &forge)), "{dq} {dr}");
    }
}
