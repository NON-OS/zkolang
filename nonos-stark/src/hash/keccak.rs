// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Keccak-f[1600], the permutation behind the keccak commitment path. The twenty-four rounds
//! run the standard theta, rho, pi, chi, and iota steps over the 25-lane state. This is the
//! hash the on-chain verifier's world uses, kept beside the Poseidon path so a proof can
//! commit under whichever hash its setting wants; the settlement path is keccak because that
//! is what an Ethereum verifier computes cheaply.

extern crate alloc;
use alloc::vec::Vec;

use super::constants::{PI_LANE, RHO_OFFSETS, ROUND_CONSTANTS};

pub(crate) fn keccak_f(state: &mut [u64; 25]) {
    for round in 0..24 {
        let mut c = [0u64; 5];
        for x in 0..5 {
            c[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
        }

        let mut d = [0u64; 5];
        for x in 0..5 {
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        }

        for x in 0..5 {
            for y in 0..5 {
                state[y * 5 + x] ^= d[x];
            }
        }

        let mut current = state[1];
        for i in 0..24 {
            let j = PI_LANE[i];
            let temp = state[j];
            state[j] = current.rotate_left(RHO_OFFSETS[i]);
            current = temp;
        }

        for y in 0..5 {
            let t = [
                state[y * 5 + 0],
                state[y * 5 + 1],
                state[y * 5 + 2],
                state[y * 5 + 3],
                state[y * 5 + 4],
            ];
            for x in 0..5 {
                state[y * 5 + x] = t[x] ^ ((!t[(x + 1) % 5]) & t[(x + 2) % 5]);
            }
        }

        state[0] ^= ROUND_CONSTANTS[round];
    }
}

pub struct Keccak {
    state: [u64; 25],
    buffer: Vec<u8>,
    rate: usize,
    pub(crate) output_len: usize,
    suffix: u8,
}

impl Keccak {
    pub fn new(capacity: usize, output_len: usize, suffix: u8) -> Self {
        assert!(capacity <= 1600);
        assert!(capacity % 8 == 0);

        Self {
            state: [0u64; 25],
            buffer: Vec::new(),
            rate: (1600 - capacity) / 8,
            output_len,
            suffix,
        }
    }

    /*
     * Absorb as the data arrives, keeping only the tail of an incomplete block.
     *
     * This used to append every byte to `buffer` and permute nothing until
     * `finalize`, which makes the type an accumulator rather than a sponge: a
     * hasher's memory grew with everything ever fed to it. On a wide circuit
     * that is fatal rather than untidy. Each wide periodic leaf absorbs one
     * value per periodic column, so at 2,649 columns one leaf held about 21 kB,
     * and the committer holds a leaf per row of a coset. A quarter of a million
     * rows times eight cosets in flight is tens of gigabytes of buffered input,
     * and it is what the kernel killed the prover for three times.
     *
     * The digest is unchanged. Padding only ever applies to the final block, so
     * absorbing the complete ones as they arrive and padding the remainder at
     * the end is the same sponge absorbing the same bytes in the same order.
     */
    pub fn update(&mut self, data: &[u8]) {
        let mut rest = data;
        if !self.buffer.is_empty() {
            let want = self.rate - self.buffer.len();
            let take = core::cmp::min(want, rest.len());
            self.buffer.extend_from_slice(&rest[..take]);
            rest = &rest[take..];
            if self.buffer.len() == self.rate {
                let block = core::mem::take(&mut self.buffer);
                self.absorb_block(&block);
            }
        }
        while rest.len() >= self.rate {
            let (block, tail) = rest.split_at(self.rate);
            self.absorb_block(block);
            rest = tail;
        }
        self.buffer.extend_from_slice(rest);
    }

    /// One full rate block into the state, then the permutation.
    fn absorb_block(&mut self, block: &[u8]) {
        for (i, &byte) in block.iter().enumerate() {
            self.state[i / 8] ^= (byte as u64) << ((i % 8) * 8);
        }
        keccak_f(&mut self.state);
    }

    /// The pad and the last block. Everything before it is already absorbed.
    fn absorb(&mut self) {
        self.buffer.push(self.suffix);

        while self.buffer.len() % self.rate != 0 {
            self.buffer.push(0);
        }

        if let Some(last) = self.buffer.last_mut() {
            *last |= 0x80;
        }

        let block = core::mem::take(&mut self.buffer);
        for chunk in block.chunks_exact(self.rate) {
            self.absorb_block(chunk);
        }
    }

    fn squeeze(&mut self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.output_len);
        let mut remaining = self.output_len;

        while remaining > 0 {
            let to_extract = core::cmp::min(remaining, self.rate);

            for i in 0..to_extract {
                let lane_idx = i / 8;
                let byte_idx = i % 8;
                let byte = (self.state[lane_idx] >> (byte_idx * 8)) as u8;
                output.push(byte);
            }

            remaining -= to_extract;

            if remaining > 0 {
                keccak_f(&mut self.state);
            }
        }

        output
    }

    pub fn finalize(mut self) -> Vec<u8> {
        self.absorb();
        self.squeeze()
    }
}

impl Drop for Keccak {
    fn drop(&mut self) {
        for lane in &mut self.state {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { core::ptr::write_volatile(lane, 0) };
        }
        for byte in &mut self.buffer {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(test)]
mod sponge_tests {
    use super::*;
    use alloc::vec::Vec;

    fn hash(chunks: &[&[u8]]) -> Vec<u8> {
        let mut k = Keccak::new(512, 32, 0x01);
        for c in chunks {
            k.update(c);
        }
        k.finalize()
    }

    /*
     * The sponge must not care how the input was handed to it.
     *
     * This is the property the type quietly did not need while `update` kept
     * every byte and permuted at the end, and it is the property that lets it
     * absorb as it goes. Sizes are chosen around the rate, 136 bytes here, so
     * the cases cover a partial block, an exact block, a block and a byte, and
     * a stream of single bytes, which is how a wide leaf actually feeds it.
     */
    #[test]
    fn a_chunked_update_hashes_the_same_as_one() {
        for len in [0usize, 1, 7, 135, 136, 137, 271, 272, 273, 1000, 21_192] {
            let data: Vec<u8> = (0..len).map(|i| (i * 31 + 7) as u8).collect();
            let whole = hash(&[&data]);
            let ones: Vec<&[u8]> = data.chunks(1).collect();
            let eights: Vec<&[u8]> = data.chunks(8).collect();
            let awkward: Vec<&[u8]> = data.chunks(137).collect();
            assert_eq!(whole, hash(&ones), "one byte at a time moved at {len}");
            assert_eq!(whole, hash(&eights), "eight at a time moved at {len}");
            assert_eq!(whole, hash(&awkward), "137 at a time moved at {len}");
        }
    }

    /// A pinned digest, so a future change to the sponge fails here rather than
    /// silently moving every commitment in the system.
    #[test]
    fn the_empty_and_abc_digests_are_pinned() {
        let empty = hash(&[&[]]);
        let abc = hash(&[b"abc"]);
        let hex = |v: &[u8]| {
            v.iter()
                .map(|b| alloc::format!("{b:02x}"))
                .collect::<alloc::string::String>()
        };
        /*
         * The published Keccak-256 vectors, which is the hash Ethereum uses
         * and therefore the one an on-chain verifier recomputes. These are not
         * the SHA3-256 values; the suffix here is 0x01, the legacy padding.
         * Pinned so a change to the sponge fails here instead of silently
         * moving every commitment in the system.
         */
        assert_eq!(
            hex(&empty),
            "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
        );
        assert_eq!(
            hex(&abc),
            "4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45"
        );
    }
}
