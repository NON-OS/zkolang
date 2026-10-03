/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Random patterns for the `match`es of `match_gen`, from a small deterministic generator. */

use crate::match_gen::Pat;

/** A small deterministic generator. */
pub(crate) struct Rng(pub(crate) u64);

impl Rng {
    pub(crate) fn below(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n
    }

    fn byte(&mut self) -> u8 {
        [0u8, 1, 2, 100, 254, 255][self.below(6) as usize]
    }

    /** A pattern of an `E` (`kind` 0), a `bool` (1) or a `u8` (2). */
    pub(crate) fn pat(&mut self, kind: u8, depth: u32) -> Pat {
        let or = depth < 2 && self.below(6) == 0;
        if or {
            return Pat::Or(
                Box::new(self.pat(kind, depth + 1)),
                Box::new(self.pat(kind, depth + 1)),
            );
        }
        match (kind, self.below(5)) {
            (_, 0) => Pat::Wild,
            (0, 1) => Pat::A,
            (0, 2) => Pat::B(Box::new(self.pat(1, depth + 1))),
            (0, _) => Pat::C(Box::new(self.pat(2, depth + 1))),
            (1, k) => Pat::Bool(k % 2 == 0),
            (_, _) => {
                let (a, b) = (self.byte(), self.byte());
                Pat::Range(a.min(b), a.max(b))
            }
        }
    }
}
