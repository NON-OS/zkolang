/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Write the MiMC permutation to `examples/mimc.zkl`, from the one source of truth
//! in the crate. Run with `cargo run -p nonos_zkolang_proofs --example gen_mimc`.

use nonos_zkolang_proofs::mimc;

fn main() {
    let src = mimc::source();
    std::fs::write("examples/mimc.zkl", &src).expect("write examples/mimc.zkl");
    eprintln!("wrote examples/mimc.zkl ({} bytes)", src.len());
}
