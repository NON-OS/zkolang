/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The formatter: lines indented by the lines that opened the brackets around them, and one
 * level more where an expression goes on; blank lines and trailing whitespace tidied; a
 * block comment moved as one; the same layout a second time; and a file that does not
 * parse left alone.
 */

use nonos_zkolang::compiler::fmt::{format, FmtError};

const MESSY: &str = "fn  main(a: public u8)   -> u8 {\n\n\n  let x = a\n  + 1;   \n      if x > 3 {\n  x\n        } else {\n\n 0 }\n\n\n}\n\n\n";

const TIDY: &str = "fn  main(a: public u8)   -> u8 {\n    let x = a\n        + 1;\n    if x > 3 {\n        x\n    } else {\n        0 }\n}\n";

const TWO_OPEN: &str = "fn f(s: u8) -> u8 {\nid(match s {\n0 => 1,\n_ => 2,\n})\n}\n";

const TWO_OPEN_TIDY: &str =
    "fn f(s: u8) -> u8 {\n    id(match s {\n        0 => 1,\n        _ => 2,\n    })\n}\n";

const COMMENT: &str = "fn f() {\n/**\n * Doc,\n *   indented.\n */\nlet x = 1;\n}\n";

const COMMENT_TIDY: &str =
    "fn f() {\n    /**\n     * Doc,\n     *   indented.\n     */\n    let x = 1;\n}\n";

#[test]
fn lines_are_indented_and_tidied() {
    assert_eq!(format(MESSY).expect("formats"), TIDY);
    assert_eq!(format(TIDY).expect("formats"), TIDY);
    assert_eq!(format(TWO_OPEN).expect("formats"), TWO_OPEN_TIDY);
    assert_eq!(format(COMMENT).expect("formats"), COMMENT_TIDY);
}

#[test]
fn a_file_that_does_not_parse_is_not_formatted() {
    assert!(matches!(format("fn f( {\n"), Err(FmtError::Syntax(_))));
}
