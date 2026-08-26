// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::field::Fp;

/// Terms the circuit does not compute: the settlement destination and the batch
/// clearing price. They are
/// inputs to the statement rather than outputs of it, so a boundary is the whole
/// binding: the proof is void for any other value, which is what stops a settler
/// substituting one. Price uniformity across a batch is a separate constraint
/// and lands with the batch assembly.
#[derive(Clone, Copy)]
pub struct Settle {
    pub clearing_price: u64,
    pub recipient: u64,
}
