/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A diagnostic renders for any offset, lines its caret up under tabs, and never points
 * at a line the file does not have. An offset inside a multibyte character, as an error
 * from the include-expanded text rendered under the caller's source can carry, used to
 * panic the renderer.
 */

use nonos_zkolang::{compile_source, render_error, CompileError};

fn diag(src: &str) -> String {
    let e = compile_source(src).expect_err("expected a compile error");
    render_error(src, &e)
}

#[test]
fn every_offset_renders() {
    let src = "// ø Kølang\noutput ø;\n";
    for at in 0..src.len() + 4 {
        let d = render_error(src, &CompileError::UnexpectedChar { at });
        assert!(d.contains("unexpected character"), "{d}");
    }
    let inside = render_error(src, &CompileError::UnexpectedChar { at: 4 });
    assert!(inside.contains("1:4"), "{inside}");
}

#[test]
fn the_caret_lines_up_under_tabs() {
    let d = diag("input x;\n\toutput x +;");
    assert!(d.contains("2:12"), "{d}");
    assert!(d.ends_with("   | \toutput x +;\n   | \t          ^"), "{d}");
}

#[test]
fn an_error_at_the_end_points_at_the_last_line() {
    let d = diag("input x;\noutput x\n\n");
    assert!(d.contains("2:9"), "{d}");
    assert!(d.contains("   | output x\n"), "{d}");
}

#[test]
fn a_lone_carriage_return_starts_a_line() {
    let d = diag("input x;\routput x +;\r");
    assert!(d.contains("2:11"), "{d}");
    assert!(d.contains("   | output x +;\n"), "{d}");
    let crlf = diag("input x;\r\noutput x +;\r\n");
    assert!(crlf.contains("2:11"), "{crlf}");
    assert!(crlf.contains("   | output x +;\n"), "{crlf}");
}
