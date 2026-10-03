/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A program image read back, by the rules `nox_verify` parses it by: counts, exempt rows,
 * operations over earlier results, constraints, rows as powers of the trace generator,
 * and boundaries, with no byte left over. `None` for any image those rules refuse.
 */

use nonos_stark::field::Fp;
use nonos_stark::fri::root_of_unity;
use nonos_zkolang_format7::{Op, Transition};

use super::cursor::Cur;
use super::source::{Boundaries, Src};

/** The tape, the trace length and the boundaries of image `b`. */
pub fn read(b: &[u8]) -> Option<(Transition, u32, Boundaries)> {
    let mut c = Cur(b, 0);
    let n: Vec<u32> = (0..6).map(|_| c.u16()).collect::<Option<_>>()?;
    let [log_t, exempt] = c.take::<2>()?;
    let (t, g) = (1u64 << log_t, root_of_unity(log_t.into()));
    if exempt != 1 || c.fp()? != g.pow(t - 1) {
        return None;
    }
    let mut ops = Vec::new();
    for i in 0..n[0] {
        let (tag, earlier) = (c.take::<1>()?[0], |x: u32| (x < i).then_some(x));
        ops.push(match tag {
            0 => Op::Const(c.fp()?, c.fp()?),
            1 => Op::Input(c.u16().filter(|&k| k < n[4] + n[5] + 4)?),
            2 => Op::Add(earlier(c.u16()?)?, earlier(c.u16()?)?),
            3 => Op::Sub(earlier(c.u16()?)?, earlier(c.u16()?)?),
            4 => Op::Mul(earlier(c.u16()?)?, earlier(c.u16()?)?),
            _ => return None,
        });
    }
    let outputs: Vec<u32> = (0..n[1])
        .map(|_| c.u16().filter(|&o| o < n[0]))
        .collect::<Option<_>>()?;
    let powers: Vec<Fp> = (0..t).map(|r| g.pow(r)).collect();
    let mut rows = Vec::new();
    for _ in 0..n[3] {
        let v = c.fp()?;
        rows.push(powers.iter().position(|p| *p == v)?);
    }
    let mut bnd = Vec::new();
    for _ in 0..n[2] {
        let col = c.take::<1>()?[0] as usize;
        let row = *rows.get(c.u16()? as usize)?;
        let src = match c.take::<1>()?[0] {
            0 => Src::Value(c.fp()?),
            1 => Src::Word(c.take::<1>()?[0].into()),
            _ => return None,
        };
        bnd.push((col, row, src));
    }
    let (n_frame, n_periodic) = (n[4] as usize, n[5] as usize);
    let tape = Transition {
        ops,
        outputs,
        n_frame,
        n_periodic,
    };
    (c.1 == b.len()).then_some((tape, log_t.into(), bnd))
}
