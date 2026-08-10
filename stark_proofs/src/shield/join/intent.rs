// NONOS Operating System (AGPL-3.0-or-later)

use super::publics::Intent;
use super::stack::Stack;
use crate::crypto::stark::air::Publics;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// The claimed tuple, and the region that pins it. `flip` perturbs one claimed
/// word so a test can show that word is tied to the cell computing it.
pub(crate) fn publics_region(
    s: &Stack,
    public_amount: u64,
    fee: u64,
    asset_id: u64,
    flip: Option<usize>,
) -> (Vec<Fp>, Publics) {
    let intent = Intent {
        note_root: s.root,
        nf: s.nf,
        out_cm: s.out_cm,
        public_amount,
        fee,
        asset_id,
    }
    .words();
    let mut claimed = intent.clone();
    if let Some(i) = flip {
        claimed[i] = claimed[i] + Fp::ONE;
    }
    (intent, Publics { log_t: 5, words: claimed })
}
