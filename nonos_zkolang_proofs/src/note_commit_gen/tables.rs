/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

use super::mds::mds;
use nonos_stark::air::{Poseidon, WIDTH};
use std::fmt::Write as _;

/// One mixing function per round, the round constant as the constant term.
pub(super) fn tables(h: &Poseidon, rounds: usize, s: &mut String) {
    let m = mds(h);
    for r in 0..rounds {
        let rc = h.round_constant(r);
        writeln!(s, "fn mix{r}(s) = [").unwrap();
        for (j, row) in m.iter().enumerate() {
            let mut line = format!("    {}", rc[j].value());
            for (i, c) in row.iter().enumerate() {
                write!(line, " + {} * s[{}]", c.value(), i).unwrap();
            }
            if j + 1 < WIDTH {
                line.push(',');
            }
            writeln!(s, "{line}").unwrap();
        }
        writeln!(s, "];").unwrap();
    }
}

pub(super) fn perm(rounds: usize, s: &mut String) {
    for r in 0..rounds {
        let lanes: Vec<String> = (0..WIDTH).map(|i| format!("s7(state[{i}])")).collect();
        writeln!(s, "let sb = [{}];", lanes.join(", ")).unwrap();
        writeln!(s, "let state = mix{r}(sb);").unwrap();
    }
}
