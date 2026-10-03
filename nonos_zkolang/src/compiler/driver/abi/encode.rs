/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Leaf values to slots, each checked against its leaf's type. */

use alloc::vec::Vec;

use nonos_stark::field::{Fp, P};

use super::leaf::{slots, AbiError, Leaf};

/** The slots of `values`, one value per leaf of `leaves`. */
pub fn encode(leaves: &[Leaf], values: &[i128]) -> Result<Vec<Fp>, AbiError> {
    if leaves.len() != values.len() {
        return Err(AbiError::Count {
            expected: leaves.len(),
            got: values.len(),
        });
    }
    let p = i128::from(P);
    let mut out = Vec::with_capacity(slots(leaves));
    for (position, (leaf, &v)) in leaves.iter().zip(values).enumerate() {
        let ok = match leaf {
            Leaf::Bool => v == 0 || v == 1,
            Leaf::Field => (0..p).contains(&v),
            Leaf::Int(t) => t.contains(v),
            Leaf::Tag(n) => (0..i128::from(*n)).contains(&v),
            Leaf::Slot => (0..p).contains(&v),
        };
        if !ok {
            return Err(AbiError::Range { position });
        }
        if leaf.width() == 2 {
            let pattern = v.rem_euclid(1i128 << 64);
            out.push(Fp::from_u64((pattern & 0xFFFF_FFFF) as u64));
            out.push(Fp::from_u64((pattern >> 32) as u64));
        } else {
            out.push(Fp::from_u64(v.rem_euclid(p) as u64));
        }
    }
    Ok(out)
}
