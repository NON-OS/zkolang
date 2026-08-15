// NONOS Operating System (AGPL-3.0-or-later)

/// The deployed tree depth. Used where the convention itself is under test.
pub(super) const DEPLOYED: usize = 32;

/// A forgery rejects through the permutation structure, not the tree size, so
/// the binding suite runs on a minimal instance and stays a change gate rather
/// than a release gate. The full depth run remains the release gate.
pub(super) const MINIMAL: usize = 3;
