/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Random expressions and arguments, from a seeded generator, so a failure repeats. */

use crate::prop_model::Ty;

/** A xorshift generator. */
pub(crate) struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /** A number below `n`, which is not zero. */
    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    /** A value of `t`: often one at an edge of its range, else any. */
    pub fn value(&mut self, t: Ty) -> i128 {
        let edges = [0, 1, -1, 2, t.min(), t.max(), t.min() + 1, t.max() - 1];
        let v = match self.below(3) {
            0 => edges[self.below(8) as usize],
            1 => i128::from(self.below(17)) - 8,
            _ => t.min() + (i128::from(self.next()) % (t.max() - t.min() + 1)),
        };
        if (t.min()..=t.max()).contains(&v) {
            v
        } else {
            0
        }
    }
}
