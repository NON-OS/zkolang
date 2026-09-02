// NONOS Operating System (AGPL-3.0-or-later)
//! Regions 6 and 7 for one query: the two index-to-point product chains. Region 6
//! walks the consistency index bits to shift * omega^p_k, the x the DEEP check
//! divides by; region 7 walks the FRI query index bits to shift * omega^i_k, the
//! layer-zero point the fold chain descends from.

use super::auth::AuthSide;
use super::fri::FriSide;
use super::tamper::Tamper;
use crate::crypto::stark::air::IndexPoint;
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::fri::root_of_unity;
use alloc::vec::Vec;

pub struct PointSide {
    pub ip: IndexPoint,
    pub itrace: Vec<Fp>,
    pub pbits: usize,
    pub fp: IndexPoint,
    pub fptrace: Vec<Fp>,
    pub fbits: usize,
}

/// The two index-point regions for one query: `cons_dirs` are query k's
/// consistency-index bits (region 6), `ik` its FRI fold position (region 7).
pub fn point_regions_k(cons_dirs: &[bool], ik: usize, log_n: u32, tamper: Tamper) -> PointSide {
    let bo = root_of_unity(log_n);
    let shift = Fp::from_u64(7);
    let pbits = cons_dirs.len();
    let pidx: usize = cons_dirs
        .iter()
        .enumerate()
        .map(|(k, &b)| (b as usize) << k)
        .sum();
    let pidx = match tamper {
        Tamper::ForeignConsistencyIndex => pidx ^ 1,
        _ => pidx,
    };
    let ip = IndexPoint::new(bo, shift, pbits, pidx);
    let itrace = ip.trace();
    let fbits = (log_n - 1) as usize;
    let fp = IndexPoint::new(bo, shift, fbits, ik);
    let fptrace = fp.trace();
    PointSide {
        ip,
        itrace,
        pbits,
        fp,
        fptrace,
        fbits,
    }
}

/// Query-0 form, preserved for the current single-query assembly.
pub fn point_regions(fs: &FriSide, au: &AuthSide, tamper: Tamper) -> PointSide {
    point_regions_k(&au.cons_dirs, fs.i0, fs.log_n, tamper)
}
