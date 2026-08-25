// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{owned, plain, secret};
use crate::shield::join::{join_split, JoinSplit, Settle, Spend};
use crate::shield::key::Break;
use crate::shield::note::Note;

/// 1000 + 2000 spent, 1500 + 1200 created, 200 out publicly, 100 in fees.
pub(crate) fn balanced_flip(brk: Break, flip: Option<usize>) -> JoinSplit {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let outs = [plain(20, 1500), plain(30, 1200)];
    build(&ins, &outs, sks, 200, 100, brk, flip)
}

pub(crate) fn balanced(brk: Break) -> JoinSplit {
    balanced_flip(brk, None)
}

/// The same spend against the tree the pool deploys. `balanced` builds a minimal
/// instance, which is what a binding gate wants and is not what a transfer costs.
pub(crate) fn balanced_deployed(brk: Break) -> JoinSplit {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let outs = [plain(20, 1500), plain(30, 1200)];
    let st = Settle { clearing_price: 1_000_000, recipient: 0xBEEF };
    crate::shield::join::join_split_at(
        super::depth::DEPLOYED,
        [Spend { note: &ins[0], sk: sks[0] }, Spend { note: &ins[1], sk: sks[1] }],
        [&outs[0], &outs[1]],
        200,
        100,
        brk,
        st,
        None,
    )
}

pub(crate) fn build(
    ins: &[Note; 2],
    outs: &[Note; 2],
    sks: [[crate::crypto::stark::field::Fp; crate::crypto::stark::air::RATE]; 2],
    public_amount: u64,
    fee: u64,
    brk: Break,
    flip: Option<usize>,
) -> JoinSplit {
    let st = Settle { clearing_price: 1_000_000, recipient: 0xBEEF };
    crate::shield::join::join_split_at(
        super::depth::MINIMAL,
        [Spend { note: &ins[0], sk: sks[0] }, Spend { note: &ins[1], sk: sks[1] }],
        [&outs[0], &outs[1]],
        public_amount,
        fee,
        brk,
        st,
        flip,
    )
}

pub(super) fn intent_at_price(price: u64) -> alloc::vec::Vec<crate::crypto::stark::field::Fp> {
    use crate::crypto::stark::air::RATE;
    use crate::crypto::stark::field::Fp;
    use crate::shield::join::publics::Intent;
    Intent {
        note_root: [Fp::from_u64(1); RATE],
        assoc_root: [Fp::from_u64(2); RATE],
        nf: [[Fp::from_u64(3); RATE], [Fp::from_u64(4); RATE]],
        out_cm: [[Fp::from_u64(5); RATE], [Fp::from_u64(6); RATE]],
        public_amount: 200,
        fee: 100,
        asset_id: 0,
        clearing_price: price,
        recipient: 0xBEEF,
    }
    .words()
}
