/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Size the trace to a power of two.

use super::params::MAX_LOG_T;

/// The smallest `log_t` whose trace holds `n` rows, or `None` past the cap. The
/// verifier-key helper sizes the trace the same way, so both agree.
pub(crate) fn choose_log_t(n: usize) -> Option<u32> {
    let mut lg = 1u32;
    while (1usize << lg) < n {
        lg += 1;
        if lg > MAX_LOG_T {
            return None;
        }
    }
    Some(lg)
}
