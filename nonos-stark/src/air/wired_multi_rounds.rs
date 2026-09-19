// NONOS Operating System (AGPL-3.0-or-later)
//! The wired engine as a two round AIR.
//!
//! Nothing here decides anything. The engine already knows where its
//! permutation columns start, how to adopt a pair of challenges, and how to
//! fill the products over a trace whose region columns are final. This says
//! that those three together are what the two round prover needs.

use super::super::field::Fp;
use super::rounds::Permuted;
use super::wired_multi_ext::WiredMultiExt;

impl Permuted for WiredMultiExt {
    fn region_width(&self) -> usize {
        WiredMultiExt::region_width(self)
    }

    fn set_challenges(&mut self, beta: Fp, gamma: Fp) {
        WiredMultiExt::set_challenges(self, beta, gamma)
    }

    fn fill_products(&self, trace: &mut [Fp]) {
        self.refill_products(trace)
    }
}
