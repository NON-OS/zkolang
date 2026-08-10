// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{owned, plain, secret};
use crate::shield::join::{join_split, JoinSplit, Spend};
use crate::shield::key::Break;
use crate::shield::note::Note;

/// 1000 + 2000 spent, 1500 + 1200 created, 200 out publicly, 100 in fees.
pub(super) fn balanced_flip(brk: Break, flip: Option<usize>) -> JoinSplit {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let outs = [plain(20, 1500), plain(30, 1200)];
    build(&ins, &outs, sks, 200, 100, brk, flip)
}

pub(super) fn balanced(brk: Break) -> JoinSplit {
    balanced_flip(brk, None)
}

pub(super) fn build(
    ins: &[Note; 2],
    outs: &[Note; 2],
    sks: [[crate::crypto::stark::field::Fp; crate::crypto::stark::air::RATE]; 2],
    public_amount: u64,
    fee: u64,
    brk: Break,
    flip: Option<usize>,
) -> JoinSplit {
    join_split(
        [Spend { note: &ins[0], sk: sks[0] }, Spend { note: &ins[1], sk: sks[1] }],
        [&outs[0], &outs[1]],
        public_amount,
        fee,
        brk,
        flip,
    )
}
