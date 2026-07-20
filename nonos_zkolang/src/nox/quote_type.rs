/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A priced quote for proving one run.

/// What proving one run costs and how the fee splits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Quote {
    /// The proving work, trace rows times width, the fee's cost driver.
    pub cells: u64,
    /// The flat floor component.
    pub base_micronox: u64,
    /// The work-proportional component.
    pub compute_micronox: u64,
    /// The full price the buyer pays, base plus compute.
    pub total_micronox: u64,
    /// The protocol cut, the network's revenue on this proof.
    pub protocol_fee_micronox: u64,
    /// What the prover is paid, the remainder after the protocol cut.
    pub prover_micronox: u64,
}
