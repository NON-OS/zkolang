// NONOS Operating System (AGPL-3.0-or-later)
//! The two soundness points, each stated once. Every prove and verify in the
//! shield reads one of these; nothing restates the numbers. Two files that
//! happen to agree are one silent downward drift from not agreeing, and a
//! soundness parameter is the last place to learn that.
//!
//! DEV is the rate-one-half point every test and the byte-digest gate run at:
//! fast, and honest about being a development setting. DEPLOYMENT is the
//! money point, 32 queries against a rate-1/16 domain with 16 bits of grind,
//! which is what the registered verifier keys and the on-chain verifier hold.
//! The two are different on purpose; what they share is this discipline.

/// The development point: tests, gates, local emits.
pub mod dev {
    /// FRI queries drawn.
    pub const N_QUERIES: usize = 32;
    /// Proof-of-work bits on the FRI transcript.
    pub const GRIND_BITS: u32 = 8;
    /// Extra blowup over the minimal rate-one-half domain.
    pub const EXTRA_BLOWUP_BITS: u32 = 0;
}

/// The deployment point: registered keys, production vectors, settlement.
pub mod deployment {
    /// FRI queries drawn.
    pub const N_QUERIES: usize = 32;
    /// Proof-of-work bits on the FRI transcript.
    pub const GRIND_BITS: u32 = 16;
    /// Extra blowup over the minimal rate-one-half domain: rate 1/16.
    pub const EXTRA_BLOWUP_BITS: u32 = 3;
}
