/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Hand out input, output and advice indices. The machine names each by a sixteen-bit
//! index, and a program that unrolls past that many would otherwise wrap its counter back
//! onto indices already in use, reading one input where it meant another. Each index is
//! taken with checked arithmetic and running out is an error.

use super::state::Compiler;
use crate::lang::CompileError;

impl Compiler {
    /// The index of the next public input.
    pub(crate) fn take_public(&mut self) -> Result<u16, CompileError> {
        let idx = self.next_public;
        self.next_public = idx.checked_add(1).ok_or(CompileError::IoLimit)?;
        Ok(idx)
    }

    /// The index of the next secret, which follows the public prefix.
    pub(crate) fn take_secret(&mut self) -> Result<u16, CompileError> {
        let idx = self
            .n_public
            .checked_add(self.next_secret)
            .ok_or(CompileError::IoLimit)?;
        self.next_secret = self
            .next_secret
            .checked_add(1)
            .ok_or(CompileError::IoLimit)?;
        Ok(idx)
    }

    /// The index of the next public output.
    pub(crate) fn take_output(&mut self) -> Result<u16, CompileError> {
        let idx = self.next_output;
        self.next_output = idx.checked_add(1).ok_or(CompileError::IoLimit)?;
        Ok(idx)
    }

    /// The input index of the next advice bit, which follows the public inputs and secrets.
    pub(crate) fn take_advice(&mut self) -> Result<u16, CompileError> {
        let idx = self
            .n_public
            .checked_add(self.n_secret)
            .and_then(|b| b.checked_add(self.next_advice))
            .ok_or(CompileError::IoLimit)?;
        self.next_advice = self
            .next_advice
            .checked_add(1)
            .ok_or(CompileError::IoLimit)?;
        Ok(idx)
    }
}
