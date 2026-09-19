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

use super::constants::ROUND_CONSTANTS;

/*
 * The round written out, lane by lane, with every index and every rotation a
 * literal. The table driven form this replaces walked rho and pi through a
 * lane order table with a rotation table beside it, one variable rotate and
 * two indexed loads per lane, and the compiler could not see through it: the
 * permutation ran at about a hundred megabytes a second on a core that does
 * four times that. The state is indexed `x + 5 * y`, the rotations are the
 * standard rho offsets by `(x, y)`, and pi sends lane `(x, y)` to
 * `(y, 2x + 3y)`. Same permutation, pinned by the published digests below.
 */
pub(crate) fn keccak_f(state: &mut [u64; 25]) {
    let a = state;
    for rc in ROUND_CONSTANTS.iter().take(24) {
        // theta
        let c0 = a[0] ^ a[5] ^ a[10] ^ a[15] ^ a[20];
        let c1 = a[1] ^ a[6] ^ a[11] ^ a[16] ^ a[21];
        let c2 = a[2] ^ a[7] ^ a[12] ^ a[17] ^ a[22];
        let c3 = a[3] ^ a[8] ^ a[13] ^ a[18] ^ a[23];
        let c4 = a[4] ^ a[9] ^ a[14] ^ a[19] ^ a[24];
        let d0 = c4 ^ c1.rotate_left(1);
        let d1 = c0 ^ c2.rotate_left(1);
        let d2 = c1 ^ c3.rotate_left(1);
        let d3 = c2 ^ c4.rotate_left(1);
        let d4 = c3 ^ c0.rotate_left(1);
        for y in 0..5 {
            a[5 * y] ^= d0;
            a[5 * y + 1] ^= d1;
            a[5 * y + 2] ^= d2;
            a[5 * y + 3] ^= d3;
            a[5 * y + 4] ^= d4;
        }

        // rho and pi
        let mut b = [0u64; 25];
        b[0] = a[0];
        b[10] = a[1].rotate_left(1);
        b[20] = a[2].rotate_left(62);
        b[5] = a[3].rotate_left(28);
        b[15] = a[4].rotate_left(27);
        b[16] = a[5].rotate_left(36);
        b[1] = a[6].rotate_left(44);
        b[11] = a[7].rotate_left(6);
        b[21] = a[8].rotate_left(55);
        b[6] = a[9].rotate_left(20);
        b[7] = a[10].rotate_left(3);
        b[17] = a[11].rotate_left(10);
        b[2] = a[12].rotate_left(43);
        b[12] = a[13].rotate_left(25);
        b[22] = a[14].rotate_left(39);
        b[23] = a[15].rotate_left(41);
        b[8] = a[16].rotate_left(45);
        b[18] = a[17].rotate_left(15);
        b[3] = a[18].rotate_left(21);
        b[13] = a[19].rotate_left(8);
        b[14] = a[20].rotate_left(18);
        b[24] = a[21].rotate_left(2);
        b[9] = a[22].rotate_left(61);
        b[19] = a[23].rotate_left(56);
        b[4] = a[24].rotate_left(14);

        // chi
        for y in 0..5 {
            let r = 5 * y;
            a[r] = b[r] ^ (!b[r + 1] & b[r + 2]);
            a[r + 1] = b[r + 1] ^ (!b[r + 2] & b[r + 3]);
            a[r + 2] = b[r + 2] ^ (!b[r + 3] & b[r + 4]);
            a[r + 3] = b[r + 3] ^ (!b[r + 4] & b[r]);
            a[r + 4] = b[r + 4] ^ (!b[r] & b[r + 1]);
        }

        // iota
        a[0] ^= *rc;
    }
}

/// The widest rate the permutation admits, in bytes: the whole state. A block
/// buffer of this size holds any rate's partial block without an allocation.
const STATE_BYTES: usize = 200;

pub struct Keccak {
    state: [u64; 25],
    /*
     * The tail of an incomplete block, at most `rate - 1` bytes, in a fixed
     * array. This was a `Vec`, and every full block took it and left an empty
     * one behind, so the next byte allocated again: one allocation and one
     * free per 136 bytes absorbed. A settlement proof pushes about a terabyte
     * through this sponge from forty cores at once, and those cores were
     * queueing on the allocator rather than hashing. Nothing here allocates
     * now, and the digest is the same digest.
     */
    buffer: [u8; STATE_BYTES],
    buffered: usize,
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
            buffer: [0u8; STATE_BYTES],
            buffered: 0,
            rate: (1600 - capacity) / 8,
            output_len,
            suffix,
        }
    }

    /*
     * Absorb as the data arrives, keeping only the tail of an incomplete block.
     *
     * This used to append every byte to a buffer and permute nothing until
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
        if self.buffered > 0 {
            let want = self.rate - self.buffered;
            let take = core::cmp::min(want, rest.len());
            self.buffer[self.buffered..self.buffered + take].copy_from_slice(&rest[..take]);
            self.buffered += take;
            rest = &rest[take..];
            if self.buffered < self.rate {
                // The block is still short, which means the input ran out.
                return;
            }
            self.absorb_buffer();
        }
        while rest.len() >= self.rate {
            let (block, tail) = rest.split_at(self.rate);
            let mut lanes = [0u64; 25];
            xor_block(&mut lanes, block);
            for (s, l) in self.state.iter_mut().zip(lanes.iter()) {
                *s ^= *l;
            }
            keccak_f(&mut self.state);
            rest = tail;
        }
        self.buffer[..rest.len()].copy_from_slice(rest);
        self.buffered = rest.len();
    }

    /// The buffered block into the state, then the permutation. The buffer is
    /// a full rate block when this is called.
    fn absorb_buffer(&mut self) {
        let mut lanes = [0u64; 25];
        xor_block(&mut lanes, &self.buffer[..self.rate]);
        for (s, l) in self.state.iter_mut().zip(lanes.iter()) {
            *s ^= *l;
        }
        keccak_f(&mut self.state);
        self.buffered = 0;
    }

    /// The pad and the last block. Everything before it is already absorbed.
    /// The tail is shorter than the rate, so the suffix always fits and the
    /// padded block is exactly one block.
    fn absorb(&mut self) {
        self.buffer[self.buffered] = self.suffix;
        for b in &mut self.buffer[self.buffered + 1..self.rate] {
            *b = 0;
        }
        self.buffer[self.rate - 1] |= 0x80;
        self.buffered = self.rate;
        self.absorb_buffer();
    }

    /// The digest, as an array. Every hasher in the crate is a 256 bit one,
    /// and the Merkle trees of one settlement proof take a quarter of a
    /// billion digests from forty cores at once, so this returns the array
    /// and allocates nothing. The output length is fixed at construction and
    /// checked here rather than read: a wider squeeze would need a second
    /// permutation and nothing asks for one.
    pub fn finalize32(mut self) -> [u8; 32] {
        debug_assert_eq!(
            self.output_len, 32,
            "every sponge in the crate squeezes 32 bytes"
        );
        self.absorb();
        let mut out = [0u8; 32];
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = (self.state[i / 8] >> ((i % 8) * 8)) as u8;
        }
        out
    }
}

/// A block of bytes as the little endian lanes it lands in: whole lanes
/// eight bytes at a time, and the bytes of a trailing partial lane one by
/// one. The byte by byte form this replaces did a division and a shift per
/// byte for every byte the sponge ever saw.
fn xor_block(lanes: &mut [u64; 25], block: &[u8]) {
    let mut chunks = block.chunks_exact(8);
    for (lane, chunk) in lanes.iter_mut().zip(&mut chunks) {
        let mut b = [0u8; 8];
        b.copy_from_slice(chunk);
        *lane ^= u64::from_le_bytes(b);
    }
    let rest = chunks.remainder();
    if !rest.is_empty() {
        let mut b = [0u8; 8];
        b[..rest.len()].copy_from_slice(rest);
        lanes[block.len() / 8] ^= u64::from_le_bytes(b);
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

    fn hash(chunks: &[&[u8]]) -> [u8; 32] {
        let mut k = Keccak::new(512, 32, 0x01);
        for c in chunks {
            k.update(c);
        }
        k.finalize32()
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

    /// How fast the sponge is, in the two shapes the prover feeds it: one
    /// long buffer per leaf, and one eight byte value at a time per periodic
    /// column. Not a correctness test; run with `--ignored --nocapture` and
    /// read the two lines. A settlement proof hashes about a terabyte
    /// through here, so a factor here is an hour on the box.
    #[test]
    #[ignore]
    #[cfg(feature = "parallel")]
    fn throughput() {
        let data: Vec<u8> = (0..(32usize << 20)).map(|i| (i * 31 + 7) as u8).collect();
        let t = std::time::Instant::now();
        let mut k = Keccak::new(512, 32, 0x01);
        k.update(&data);
        let d = k.finalize32();
        let s = t.elapsed().as_secs_f64();
        std::println!(
            "KECCAK bulk {:.0} MB/s (digest byte {})",
            data.len() as f64 / 1e6 / s,
            d[0]
        );

        let cols = 2649usize;
        let leaves = 4096usize;
        let t = std::time::Instant::now();
        let mut acc = 0u64;
        for leaf in 0..leaves {
            let mut k = Keccak::new(512, 32, 0x01);
            for c in 0..cols {
                k.update(&((leaf * cols + c) as u64).to_le_bytes());
            }
            acc ^= k.finalize32()[0] as u64;
        }
        let s = t.elapsed().as_secs_f64();
        std::println!(
            "KECCAK streamed {:.0} leaves/s, {:.0} MB/s (acc {acc})",
            leaves as f64 / s,
            (leaves * cols * 8) as f64 / 1e6 / s
        );
    }
}
