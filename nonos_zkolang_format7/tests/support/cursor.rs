/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Big-endian numbers read off the front of a byte string. */

use nonos_stark::field::{Fp, P};

/** The bytes, and how many are read. */
pub struct Cur<'a>(pub &'a [u8], pub usize);

impl Cur<'_> {
    pub fn take<const N: usize>(&mut self) -> Option<[u8; N]> {
        let s = self.0.get(self.1..self.1 + N)?.try_into().ok()?;
        self.1 += N;
        Some(s)
    }
    pub fn u16(&mut self) -> Option<u32> {
        self.take::<2>().map(|b| u16::from_be_bytes(b).into())
    }
    pub fn fp(&mut self) -> Option<Fp> {
        let v = u64::from_be_bytes(self.take::<8>()?);
        (v < P).then(|| Fp::from_u64(v))
    }
}
