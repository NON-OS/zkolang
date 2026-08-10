// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{owned, plain, secret};
use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::join::{intent_parts, IntentParts, Settle, Spend};
use crate::shield::key::Break;

pub(super) fn intent(seed: u64, price: u64, flip: Option<usize>) -> IntentParts {
    let sks = [secret(seed), secret(seed + 100)];
    let ins = [owned(sks[0], seed, 1000), owned(sks[1], seed + 10, 2000)];
    let outs = [plain(seed + 20, 1500), plain(seed + 30, 1200)];
    let st = Settle {
        assoc_root: [Fp::from_u64(9); RATE],
        clearing_price: price,
        recipient: 0xBEEF,
    };
    intent_parts(
        [Spend { note: &ins[0], sk: sks[0] }, Spend { note: &ins[1], sk: sks[1] }],
        [&outs[0], &outs[1]],
        200,
        100,
        Break::None,
        st,
        flip,
    )
}
