/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A compiled subexpression.

pub(crate) struct Val {
    /// The register holding the value.
    pub(crate) reg: u8,
    /// Whether that register is a temporary, safe to free once consumed, rather than
    /// a live binding.
    pub(crate) temp: bool,
}
