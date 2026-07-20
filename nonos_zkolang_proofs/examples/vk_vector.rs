/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/
//! Emit the verifier-key descriptor byte-exact and a golden vk, so the SC helper can
//! gate against it. Confirms the manual layout equals verifier_key().
use nonos_stark::hash::keccak256;
use nonos_zkolang::{
    commit, compile_source, periodic_root, prove_source_with_inputs, verifier_key,
    REGISTRATION_RATE, TRACE_WIDTH,
};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn main() {
    let src = "input x; output x;";
    let program = compile_source(src).unwrap();
    let rate = REGISTRATION_RATE;
    let commit_bytes = commit(&program);
    let root = periodic_root(&program, rate).unwrap();
    let log_t = prove_source_with_inputs(src, &[7]).unwrap().log_trace_len;
    let width = TRACE_WIDTH as u32;

    let mut buf: Vec<u8> = Vec::new();
    buf.push(1u8); // WIRING_VERSION
    buf.extend_from_slice(&commit_bytes); // 32
    buf.extend_from_slice(&(log_t as u32).to_le_bytes()); // 4 LE
    buf.extend_from_slice(&width.to_le_bytes()); // 4 LE
    buf.extend_from_slice(&rate.to_le_bytes()); // 4 LE
    buf.extend_from_slice(&root); // 32

    let vk = verifier_key(&program, rate).unwrap();
    assert_eq!(keccak256(&buf), vk, "descriptor layout mismatch");

    println!("program           : {src}");
    println!("WIRING_VERSION u8 : 01");
    println!("commit    32      : {}", hex(&commit_bytes));
    println!(
        "log2N     u32 LE  : {log_t} -> {}",
        hex(&(log_t as u32).to_le_bytes())
    );
    println!(
        "width     u32 LE  : {width} -> {}",
        hex(&width.to_le_bytes())
    );
    println!("rate      u32 LE  : {rate} -> {}", hex(&rate.to_le_bytes()));
    println!("root      32      : {}", hex(&root));
    println!("descriptor {}B    : {}", buf.len(), hex(&buf));
    println!("vk keccak256      : {}", hex(&vk));
}
