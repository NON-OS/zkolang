// NONOS Operating System (AGPL-3.0-or-later)
// LOCAL WORKAROUND (not for landing): the constant_time + hash kernel-mirror
// includes were stripped here because this /tmp checkout lost the mirrored
// src/crypto source. They are dead for stark_proofs (nothing references them);
// the canonical tree keeps them where the kernel source resolves.

// The stark stack now lives in the nonos-stark crate. Re-export it here so the
// proofs run against the same source the kernel and bootloader link, addressed
// at the path the tests already use.
pub use nonos_stark as stark;
