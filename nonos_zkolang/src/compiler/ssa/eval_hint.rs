/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Computing an advice value from its hint. */

use nonos_stark::field::Fp;

use super::{Hint, V};

/** The advice `h` computes from the values so far, or `None` if it reads one not yet. */
pub fn hint(h: Hint, values: &[Fp]) -> Option<Fp> {
    let get = |v: V| values.get(v.index()).map(|x| x.value());
    Some(Fp::from_u64(match h {
        Hint::Bit(a, k) => get(a)?.checked_shr(u32::from(k)).unwrap_or(0) & 1,
        Hint::Quot(a, b) => get(a)?.checked_div(get(b)?).unwrap_or(0),
        Hint::Rem(a, b) => {
            let (a, b) = (get(a)?, get(b)?);
            a.checked_rem(b).unwrap_or(a)
        }
        Hint::Copy(a) => get(a)?,
    }))
}
