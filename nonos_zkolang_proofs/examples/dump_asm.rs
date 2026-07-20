use nonos_zkolang::{compile_source, to_asm};
fn main() {
    let p = compile_source("input x; let y = x * x * x; output y;").unwrap();
    print!("{}", to_asm(&p));
}
