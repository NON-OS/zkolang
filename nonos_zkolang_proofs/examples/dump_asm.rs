use nonos_zkolang::{compile_source_full, to_asm};
fn main() {
    let p = compile_source_full("input x; let y = x * x * x; output y;").unwrap();
    print!("{}", to_asm(&p));
}
