/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A recorded transition run again on values, as a verifier reading the image runs it. */

use nonos_stark::field::{Fp, Fp2};

use super::rec::Op;
use super::tape::Transition;

/** The constraints of `t` at the window `frame` and the periodic values `periodic`. */
pub fn replay(t: &Transition, frame: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
    let mut v: Vec<Fp2> = Vec::with_capacity(t.ops.len());
    for op in &t.ops {
        let x = match *op {
            Op::Const(c0, c1) => Fp2::new(c0, c1),
            Op::Input(k) if (k as usize) < t.n_frame => frame[k as usize],
            Op::Input(k) => periodic[k as usize - t.n_frame],
            Op::Add(a, b) => v[a as usize] + v[b as usize],
            Op::Sub(a, b) => v[a as usize] - v[b as usize],
            Op::Mul(a, b) => v[a as usize] * v[b as usize],
        };
        v.push(x);
    }
    t.outputs.iter().map(|&o| v[o as usize]).collect()
}

fn mix(x: u64) -> u64 {
    let x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    let x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/** A point off every structure the AIR has: `w` frame values and `p` periodic ones. */
pub fn probe(w: usize, p: usize) -> (Vec<Fp2>, Vec<Fp2>) {
    let at = |i: usize| {
        let i = i as u64 * 2 + 1;
        Fp2::new(Fp::from_u64(mix(i)), Fp::from_u64(mix(i + 1)))
    };
    ((0..w).map(at).collect(), (w..w + p).map(at).collect())
}
