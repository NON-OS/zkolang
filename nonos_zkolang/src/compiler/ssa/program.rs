/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A whole program in SSA form, with the shape of its inputs and outputs. */

use alloc::vec::Vec;

use super::{Inst, Site, V};

/**
 * A program: its instructions, where each defines the value of its index, the site of
 * each, and how many public and secret input slots and output slots it has (section
 * 12.2). An instruction past the end of `sites` has the default site.
 */
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Ssa {
    pub insts: Vec<Inst>,
    pub sites: Vec<Site>,
    pub n_public: u16,
    pub n_secret: u16,
    pub n_outputs: u16,
}

impl Ssa {
    /** The instruction that defines `v`. */
    pub fn get(&self, v: V) -> Option<&Inst> {
        self.insts.get(v.index())
    }

    /** The site of the instruction at index `i`. */
    pub fn site(&self, i: usize) -> Site {
        self.sites.get(i).copied().unwrap_or_default()
    }

    /** How many input slots there are, public and secret. */
    pub fn n_inputs(&self) -> usize {
        usize::from(self.n_public) + usize::from(self.n_secret)
    }

    /** How many advice values the program reads. */
    pub fn n_advice(&self) -> usize {
        self.insts
            .iter()
            .filter(|i| matches!(i, Inst::Advice(_)))
            .count()
    }
}
