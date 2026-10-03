/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The tape and the constraints of a program image, and its field elements. */

use nonos_stark::field::Fp;

use super::image::ImageError;
use super::rec::Op;
use super::tape::Transition;

/** `v` as eight big-endian bytes. */
pub(crate) fn fp(b: &mut Vec<u8>, v: Fp) {
    b.extend(v.to_u64().to_be_bytes());
}

fn handle(x: u32) -> Result<[u8; 2], ImageError> {
    u16::try_from(x)
        .map(u16::to_be_bytes)
        .map_err(|_| ImageError::TooLarge("an operation"))
}

/**
 * Each operation of `t` as its tag and operands: 0 a constant's two components, 1 an
 * input, 2, 3 and 4 an addition, subtraction and multiplication of earlier results. Then
 * the constraints, each the operation that computes it.
 */
pub(crate) fn ops(b: &mut Vec<u8>, t: &Transition) -> Result<(), ImageError> {
    for op in &t.ops {
        let (tag, x, y) = match *op {
            Op::Const(c0, c1) => {
                b.push(0);
                fp(b, c0);
                fp(b, c1);
                continue;
            }
            Op::Input(k) => (1, k, None),
            Op::Add(a, c) => (2, a, Some(c)),
            Op::Sub(a, c) => (3, a, Some(c)),
            Op::Mul(a, c) => (4, a, Some(c)),
        };
        b.push(tag);
        b.extend(handle(x)?);
        if let Some(y) = y {
            b.extend(handle(y)?);
        }
    }
    for &o in &t.outputs {
        b.extend(handle(o)?);
    }
    Ok(())
}
