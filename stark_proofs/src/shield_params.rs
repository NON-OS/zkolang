// NONOS Operating System (AGPL-3.0-or-later)
//! The soundness points, each stated once. Every prove and verify in the
//! shield reads one of these; nothing restates the numbers. Two files that
//! happen to agree are one silent downward drift from not agreeing, and a
//! soundness parameter is the last place to learn that.
//!
//! DEV is the rate-one-half point every test and the byte-digest gate run at:
//! fast, and honest about being a development setting. DEPLOYMENT is the
//! settlement point, 32 queries against a rate-1/16 domain with 16 bits of
//! grind, which is what the registered verifier keys and the on-chain verifier
//! hold; the high rate buys a small proof at the cost of a prover that walks a
//! 16x domain, the right trade for a proof that lands on chain once per batch.
//! TRANSFER is the same 144 bits reached the other way, for the proof a sender
//! makes: more queries against a rate-1/4 domain, so the prover walks a quarter
//! of the settlement domain and the proof is verified off chain by the
//! recursion, where its size does not matter. The points differ on purpose;
//! what they share is this discipline, and one soundness floor.

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

/// The transfer point: the proof a sender makes, verified off chain by the
/// recursion. It reaches the same 144 bits as settlement with more queries
/// against a lower rate, so the prover walks a quarter of the settlement domain.
/// The proof is larger, which costs nothing off chain; what a transaction costs
/// is prover time, and this is the point that minimises it at full soundness.
/// Measured on the deployed circuit, 64 queries costs the recursion 12.5 percent
/// more outer rows than 56 and nothing else, the same log trace length, degree,
/// width and domain, so the full 144 bits are taken rather than settling for 128.
pub mod transfer {
    /// FRI queries drawn.
    pub const N_QUERIES: usize = 64;
    /// Proof-of-work bits on the FRI transcript.
    pub const GRIND_BITS: u32 = 16;
    /// Extra blowup over the minimal rate-one-half domain: rate 1/4.
    pub const EXTRA_BLOWUP_BITS: u32 = 1;
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

    /// The transfer point is never weaker than settlement. A sender's proof is
    /// verified off chain, but it still has to be unforgeable, and moving it to a
    /// cheaper rate must not quietly lower the statement's soundness: the policy
    /// is that the two points share one floor, and this is the gate that holds it.
    #[test]
    fn the_transfer_point_is_not_weaker_than_settlement() {
        let transfer_bits = security_bits(
            transfer::N_QUERIES,
            transfer::GRIND_BITS,
            transfer::EXTRA_BLOWUP_BITS,
        );
        let settlement_bits = security_bits(
            deployment::N_QUERIES,
            deployment::GRIND_BITS,
            deployment::EXTRA_BLOWUP_BITS,
        );
        assert!(transfer_bits >= 128, "transfer soundness is {transfer_bits} bits, below 128");
        assert!(
            transfer_bits >= settlement_bits,
            "transfer soundness is {transfer_bits} bits, below settlement's {settlement_bits}"
        );
    }

    /// The transfer point earns its place by walking a smaller domain than
    /// settlement: that is the whole reason it exists, so a sender does not pay
    /// the settlement prover's 16x domain for a proof nobody posts on chain. If
    /// its blowup ever reaches settlement's the two have collapsed into one.
    #[test]
    fn the_transfer_point_walks_a_smaller_domain_than_settlement() {
        assert!(
            transfer::EXTRA_BLOWUP_BITS < deployment::EXTRA_BLOWUP_BITS,
            "the transfer domain is not smaller than the settlement domain"
        );
    }
}
