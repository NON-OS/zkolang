/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Close a block's scope. */

use alloc::vec::Vec;

use super::super::compiler::Compiler;

/** Where a block's scope began: its first local and its first hidden array. */
#[derive(Clone, Copy)]
pub(crate) struct BlockMark {
    pub(crate) syms: usize,
    pub(crate) hidden: usize,
}

impl Compiler {
    /**
     * Drop the block's locals back to `mark` and return the registers they alone held to the
     * pool, keeping the result registers and anything an outer name still aliases. The alias
     * check makes this sound: a register a live binding shares is never freed under the block.
     */
    pub(crate) fn close_block(&mut self, mark: BlockMark, result_regs: &[u8]) {
        let held: Vec<u8> = self
            .syms
            .split_off(mark.syms)
            .into_iter()
            .map(|(_, r)| r)
            .collect();
        for r in held {
            if !result_regs.contains(&r) && !self.reg_in_use(r) && !self.free.contains(&r) {
                self.free_reg(r);
            }
        }
        self.restore_hidden(mark.hidden);
    }
}
