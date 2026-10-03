/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Sites through the back end (section 15.2): scheduling moves each instruction with its
 * site, and no pass gives an instruction a site the program it was given did not have.
 */

use nonos_zkolang::compiler::driver::backend;
use nonos_zkolang::compiler::schedule::schedule;
use nonos_zkolang::compiler::ssa::{Site, Ssa, V};

use crate::prop_gen::Rng;
use crate::ssa_gen::program;

/** `ssa` with the instruction at index `i` given the site `i + 1`. */
fn sited(mut ssa: Ssa) -> Ssa {
    let n = u32::try_from(ssa.insts.len()).unwrap_or(u32::MAX);
    ssa.sites = (1..=n).map(Site).collect();
    ssa
}

#[test]
fn scheduling_moves_each_instruction_with_its_site() {
    let mut r = Rng(0x5173_5EED_0B1E_C7ED);
    let mut moved = 0;
    for _ in 0..40 {
        let old = sited(program(&mut r, 80));
        let new = schedule(&old);
        let mut at = vec![0u32; old.insts.len()];
        for k in 0..new.insts.len() {
            at[new.site(k).0 as usize - 1] = k as u32;
        }
        for k in 0..new.insts.len() {
            let o = new.site(k).0 as usize - 1;
            let renamed = old.insts[o].map(&mut |v| V(at[v.index()]));
            assert_eq!(renamed, new.insts[k], "instruction {o}, now at {k}");
            moved += usize::from(o != k);
        }
    }
    assert!(moved > 0, "no program was reordered");
}

#[test]
fn no_pass_invents_a_site() {
    let mut r = Rng(0x0B5E_55ED_5173_0001);
    for _ in 0..40 {
        let old = sited(program(&mut r, 80));
        let Ok(c) = backend(&old) else { continue };
        assert_eq!(c.ssa.sites.len(), c.ssa.insts.len());
        let n = old.insts.len() as u32;
        assert!(c.ssa.sites.iter().all(|s| (1..=n).contains(&s.0)));
    }
}
