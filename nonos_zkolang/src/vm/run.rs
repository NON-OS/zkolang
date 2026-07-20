/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The run loop: step through the program and stop at the halt.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::{ProveError, Vm};
use crate::isa::Program;
use crate::trace::{Row, Trace};

impl Vm {
    /// Run the program on inputs, the first `n_public` of which are public, returning
    /// the trace and the public boundary the proof commits to. The clock is the row
    /// index, so the trace is ordered by construction.
    pub fn run(
        &mut self,
        program: &Program,
        inputs: &[Fp],
        n_public: usize,
    ) -> Result<Trace, ProveError> {
        let mut rows: Vec<Row> = Vec::with_capacity(program.len());
        let mut outputs: Vec<Fp> = Vec::new();

        for (i, op) in program.iter().enumerate() {
            let clk = i as u64;
            let mut row = Row::at(clk);
            if self.step(*op, inputs, &mut outputs, &mut row, clk)? {
                rows.push(row);
                let n_pub = n_public.min(inputs.len());
                return Ok(Trace {
                    rows,
                    public_inputs: inputs[..n_pub].to_vec(),
                    public_outputs: outputs,
                });
            }
            rows.push(row);
        }
        Err(ProveError::NoHalt)
    }
}
