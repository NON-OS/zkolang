/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The thresholds of the cost warnings (section 15.3), which a manifest's `[cost]` may set. */

use super::manifest::Manifest;

/** A loop warns past `unroll_warn` iterations, a runtime index past `dyn_index_warn` elements. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Limits {
    pub unroll_warn: u64,
    pub dyn_index_warn: u64,
}

impl Default for Limits {
    fn default() -> Limits {
        Limits {
            unroll_warn: 1024,
            dyn_index_warn: 64,
        }
    }
}

impl Manifest {
    /** The thresholds the manifest's `[cost]` sets, the defaults where it sets none. */
    pub fn limits(&self) -> Limits {
        let d = Limits::default();
        Limits {
            unroll_warn: self.unroll_warn.unwrap_or(d.unroll_warn),
            dyn_index_warn: self.dyn_index_warn.unwrap_or(d.dyn_index_warn),
        }
    }
}
