/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! SSA values. */

/** A value: the index of the instruction that defines it. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct V(pub u32);

impl V {
    /** The index of the defining instruction. */
    pub fn index(self) -> usize {
        self.0 as usize
    }
}
