/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Placing an instruction, and whatever it lets be placed at once. */

use alloc::vec;

use super::order::Walk;

impl Walk<'_> {
    /** Place `u`, whose dependences are placed, then everything that becomes eager. */
    pub(super) fn place(&mut self, u: usize) {
        let mut work = vec![u];
        while let Some(x) = work.pop() {
            if self.placed[x] {
                continue;
            }
            /* A constant or public input it reads is written just before it. */
            for &d in &self.g.deps[x] {
                if !self.placed[d] {
                    self.placed[d] = true;
                    self.order.push(d);
                }
            }
            self.placed[x] = true;
            self.order.push(x);
            for &c in &self.g.users[x] {
                if self.eager(c) {
                    work.push(c);
                }
            }
            let reads = &self.g.deps[x][..self.g.reads[x]];
            for (k, &o) in reads.iter().enumerate() {
                let (Some(l), false) = (self.left.get_mut(o), reads[..k].contains(&o)) else {
                    continue;
                };
                *l = l.saturating_sub(1);
                if *l == 1 {
                    let last = self.g.users[o].iter().copied().find(|&c| !self.placed[c]);
                    work.extend(last.filter(|&c| self.eager(c)));
                }
            }
        }
    }

    /**
     * Whether `c` is placed now: its dependences are placed, or are constants or public
     * inputs, and it is a sink, or an operation that is the last reader of an operand held
     * in a register, so that it frees one for the one it takes.
     */
    fn eager(&self, c: usize) -> bool {
        let g = self.g;
        let ready = |d: &usize| self.placed[*d] || g.recoverable[*d];
        if self.placed[c] || !g.deps[c].iter().all(ready) {
            return false;
        }
        let frees = |o: &usize| self.left[*o] == 1 && !g.recoverable[*o];
        g.sink[c] || g.op[c] && g.deps[c].iter().take(g.reads[c]).any(frees)
    }
}
