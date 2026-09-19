// NONOS Operating System (AGPL-3.0-or-later)

use super::publics::Intent;
use super::settle::{address_limbs, Settle};
use super::stack::Stack;
use crate::crypto::stark::air::Publics;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// The claimed tuple, and the region that pins it. `flip` perturbs one claimed
/// word so a test can show that word is tied to the cell computing it.
pub fn publics_region(
    s: &Stack,
    public_amount: u64,
    fee: u64,
    asset_id: u64,
    st: Settle,
    flip: Option<usize>,
) -> (Vec<Fp>, Publics) {
    /*
     * Settlement fields ride the intent only when value actually leaves the
     * pool. A transfer settles nothing, so a payout address and a clearing
     * price among its public words are either a name the sender can be found
     * by or a channel it can be tagged through, and both sit on chain forever
     * whether or not anything acts on them.
     *
     * Forced here rather than left to the caller. A wallet that passed a live
     * Settle alongside a transfer would publish an address the statement makes
     * no use of, and the transfer would look like every other one until
     * somebody read the calldata. A contract should refuse the same pairing,
     * but a reverted transaction has already published its arguments, so the
     * place to stop it is before anything is signed.
     */
    let settles = public_amount != 0;
    let intent = Intent {
        note_root: s.root,
        assoc_root: s.assoc_root,
        nf: s.nf,
        out_cm: s.out_cm,
        public_amount,
        fee,
        asset_id,
        clearing_price: if settles { st.clearing_price } else { 0 },
        recipient: if settles {
            address_limbs(&st.recipient)
        } else {
            [Fp::ZERO; crate::crypto::stark::air::RATE]
        },
    }
    .words();
    let mut claimed = intent.clone();
    if let Some(i) = flip {
        claimed[i] = claimed[i] + Fp::ONE;
    }
    (
        intent,
        Publics {
            log_t: 5,
            words: claimed,
        },
    )
}
