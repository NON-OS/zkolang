// NONOS Operating System (AGPL-3.0-or-later)

use alloc::vec::Vec;

pub(crate) struct Uf(Vec<usize>);

impl Uf {
    pub(crate) fn new(n: usize) -> Self {
        Uf((0..n).collect())
    }

    pub(crate) fn find(&mut self, x: usize) -> usize {
        let mut r = x;
        while self.0[r] != r {
            r = self.0[r];
        }
        let mut c = x;
        while self.0[c] != c {
            let next = self.0[c];
            self.0[c] = r;
            c = next;
        }
        r
    }

    pub(crate) fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.0[ra] = rb;
        }
    }
}
