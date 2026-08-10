// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// The frozen intent tuple, in the order settleBatch decodes it. Digests occupy
/// four rows each; the scalars one.
pub(crate) struct Intent {
    pub note_root: [Fp; RATE],
    pub nf: [[Fp; RATE]; 2],
    pub out_cm: [[Fp; RATE]; 2],
    pub public_amount: u64,
    pub fee: u64,
    pub asset_id: u64,
}

pub(crate) const NOTE_ROOT: usize = 0;
pub(crate) const NF0: usize = 4;
pub(crate) const NF1: usize = 8;
pub(crate) const OUT_CM0: usize = 12;
pub(crate) const OUT_CM1: usize = 16;
pub(crate) const PUBLIC_AMOUNT: usize = 20;
pub(crate) const FEE: usize = 21;
pub(crate) const ASSET_ID: usize = 22;
pub(crate) const WORDS: usize = 23;

impl Intent {
    pub fn words(&self) -> Vec<Fp> {
        let mut w = Vec::with_capacity(WORDS);
        w.extend_from_slice(&self.note_root);
        w.extend_from_slice(&self.nf[0]);
        w.extend_from_slice(&self.nf[1]);
        w.extend_from_slice(&self.out_cm[0]);
        w.extend_from_slice(&self.out_cm[1]);
        w.push(Fp::from_u64(self.public_amount));
        w.push(Fp::from_u64(self.fee));
        w.push(Fp::from_u64(self.asset_id));
        w
    }
}
