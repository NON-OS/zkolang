/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Text nested far past the parser's budget parses on a small stack, in every construct. */

use crate::front_check::check;

#[test]
fn deep_mixed_brackets_parse_without_overflow() {
    let run = |src: String| {
        std::thread::Builder::new()
            .stack_size(2 << 20)
            .spawn(move || check(&src))
    };
    let opens = [
        "(",
        "[",
        "{",
        "fn f() {",
        "let x = (",
        "match x { _ => (",
        "<",
        "-",
        "!",
    ];
    for open in opens {
        let src = format!("fn g() {{ {} }}", open.repeat(20_000));
        run(src).expect("spawn").join().expect("no overflow");
    }
    let types = format!(
        "fn h(a: {}u8{}) {{}}",
        "[(".repeat(5_000),
        ",); 2]".repeat(5_000)
    );
    run(types).expect("spawn").join().expect("no overflow");
}
