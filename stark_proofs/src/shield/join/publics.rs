// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// The frozen intent tuple in the order settleBatch decodes it. Digests occupy
/// four rows each, scalars one.
pub struct Intent {
    pub note_root: [Fp; RATE],
    pub assoc_root: [Fp; RATE],
    pub nf: [[Fp; RATE]; 2],
    pub out_cm: [[Fp; RATE]; 2],
    pub public_amount: u64,
    pub fee: u64,
    pub asset_id: u64,
    pub clearing_price: u64,
    pub recipient: u64,
}

pub const NOTE_ROOT: usize = 0;
pub const ASSOC_ROOT: usize = 4;
pub const NF0: usize = 8;
pub const NF1: usize = 12;
pub const OUT_CM0: usize = 16;
pub const OUT_CM1: usize = 20;
pub const PUBLIC_AMOUNT: usize = 24;
pub const FEE: usize = 25;
pub const ASSET_ID: usize = 26;
pub const CLEARING_PRICE: usize = 27;
pub const RECIPIENT: usize = 28;
pub const WORDS: usize = 29;

impl Intent {
    pub fn words(&self) -> Vec<Fp> {
        let mut w = Vec::with_capacity(WORDS);
        w.extend_from_slice(&self.note_root);
        w.extend_from_slice(&self.assoc_root);
        w.extend_from_slice(&self.nf[0]);
        w.extend_from_slice(&self.nf[1]);
        w.extend_from_slice(&self.out_cm[0]);
        w.extend_from_slice(&self.out_cm[1]);
        for v in [self.public_amount, self.fee, self.asset_id, self.clearing_price, self.recipient]
        {
            w.push(Fp::from_u64(v));
        }
        w
    }
}
