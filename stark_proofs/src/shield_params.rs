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

/// Conjectured FRI security in bits at a soundness point: each query catches a
/// non-low-degree codeword with probability the rate sets, `log2(1/rate)` bits,
/// and the grind adds its bits on top. The rate is `1 / 2^(1 + extra_blowup)`,
/// so the per-query yield is `1 + extra_blowup` bits.
pub const fn security_bits(n_queries: usize, grind_bits: u32, extra_blowup_bits: u32) -> u32 {
    n_queries as u32 * (1 + extra_blowup_bits) + grind_bits
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The money point clears 128-bit soundness. This is the gate that stops a
    /// silent downward drift of the settlement rate: change the deployment
    /// numbers below the line and the suite goes red here, not on chain.
    #[test]
    fn the_deployment_point_is_at_least_128_bit() {
        let bits = security_bits(
            deployment::N_QUERIES,
            deployment::GRIND_BITS,
            deployment::EXTRA_BLOWUP_BITS,
        );
        assert!(bits >= 128, "deployment soundness is {bits} bits, below the 128-bit floor");
    }

    /// The development point is deliberately weaker, and labelled so. If it ever
    /// reaches deployment strength the distinction has collapsed and a test may
    /// be running at money cost by accident.
    #[test]
    fn the_development_point_is_below_the_money_point() {
        let dev_bits = security_bits(dev::N_QUERIES, dev::GRIND_BITS, dev::EXTRA_BLOWUP_BITS);
        let dep_bits = security_bits(
            deployment::N_QUERIES,
            deployment::GRIND_BITS,
            deployment::EXTRA_BLOWUP_BITS,
        );
        assert!(dev_bits < dep_bits, "the development point is not weaker than deployment");
    }
}
