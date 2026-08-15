/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

use nonos_stark::air::{Poseidon, WIDTH};
use nonos_stark::field::Fp;

pub(super) const POOL_LOG_ROUNDS: u32 = 5;

/// The mixing matrix is private, but the S-box fixes zero and one, so a round on
/// the i-th unit vector with zero constants returns that column.
fn column(h: &Poseidon, i: usize) -> [Fp; WIDTH] {
    let mut unit = [Fp::ZERO; WIDTH];
    unit[i] = Fp::ONE;
    h.round_with_rc(&unit, &[Fp::ZERO; WIDTH])
}

pub(super) fn mds(h: &Poseidon) -> [[Fp; WIDTH]; WIDTH] {
    let cols: Vec<[Fp; WIDTH]> = (0..WIDTH).map(|i| column(h, i)).collect();
    let mut m = [[Fp::ZERO; WIDTH]; WIDTH];
    for (j, row) in m.iter_mut().enumerate() {
        for (i, cell) in row.iter_mut().enumerate() {
            *cell = cols[i][j];
        }
    }
    m
}
