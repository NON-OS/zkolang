/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A program image: the bytes a STARKs format 7 verifier reads a circuit from, as
 * `nox_verify`'s `Program::parse` reads them. A header of counts, the rows the transition
 * is exempt on, the transition's tape and its constraints, the rows the boundaries name,
 * then the boundaries, each pinned to a value or to a public word. Every number is
 * big-endian.
 */

use nonos_stark::air::Air;

use nonos_stark::fri::root_of_unity;
use nonos_zkolang::{Pin, StepAir};

use super::image_ops::{fp, ops};
use super::tape::Transition;
use super::word::word;

/** Why a program has no image. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageError {
    /** More than the format counts: of operations, rows, boundaries or public words. */
    TooLarge(&'static str),
}

fn small<T: TryFrom<usize>>(n: usize, what: &'static str) -> Result<T, ImageError> {
    T::try_from(n).map_err(|_| ImageError::TooLarge(what))
}

/** The image of `air` under the transition `t`, its statement of `inputs` input slots. */
pub fn image(t: &Transition, air: &StepAir, inputs: usize) -> Result<Vec<u8>, ImageError> {
    let (log_t, window) = (air.log_trace_len(), air.window_size());
    let (n, g) = (1u64 << log_t, root_of_unity(log_t));
    let pins = air.boundary_pins();
    let mut rows: Vec<usize> = pins.iter().map(|p| p.1).collect();
    rows.sort_unstable();
    rows.dedup();
    let counts = [t.ops.len(), t.outputs.len(), pins.len(), rows.len()];
    let mut b = Vec::new();
    for c in counts.into_iter().chain([t.n_frame, t.n_periodic]) {
        b.extend(small::<u16>(c, "a count")?.to_be_bytes());
    }
    b.extend([
        small::<u8>(log_t as usize, "log_t")?,
        small(window - 1, "window")?,
    ]);
    (1..window as u64).for_each(|k| fp(&mut b, g.pow(n - k)));
    ops(&mut b, t)?;
    rows.iter().for_each(|&r| fp(&mut b, g.pow(r as u64)));
    for (col, row, pin) in pins {
        b.push(small(col, "a column")?);
        let at = rows.binary_search(&row).unwrap_or_default();
        b.extend(small::<u16>(at, "a row")?.to_be_bytes());
        match pin {
            Pin::Value(v) => {
                b.push(0);
                fp(&mut b, v);
            }
            _ => {
                let k = word(pin, inputs).unwrap_or(usize::MAX);
                b.extend([1, small(k, "a public word")?]);
            }
        }
    }
    Ok(b)
}
