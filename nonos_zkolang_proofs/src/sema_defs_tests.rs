/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Items and imports: names resolve through modules and `use`, to a fixed point that
 * terminates on cycles, and duplicate, private, missing and ambiguous names are reported.
 */

use nonos_zkolang::compiler::sema::defs::{DefKind, PathError};

use crate::sema_defs_check::names;

#[test]
fn names_resolve_through_modules_and_imports() {
    let src = "mod a { pub mod b { pub fn f() {} } }\nuse a::b::f;\nuse a::b as c;\nmod d { use super::a::b::*; }\n";
    let (codes, found) = names(src, &[("", "f"), ("", "c"), ("d", "f"), ("", "c::f")]);
    assert_eq!(codes, Vec::<&str>::new());
    assert_eq!(
        found,
        vec![
            Ok(DefKind::Fn),
            Ok(DefKind::Mod),
            Ok(DefKind::Fn),
            Ok(DefKind::Fn)
        ]
    );
}

#[test]
fn imports_resolve_to_a_fixed_point_and_glob_cycles_end() {
    let chain = "use b::x as y;\nmod b { pub use super::c::x; }\nmod c { pub fn x() {} }\n";
    assert_eq!(names(chain, &[("", "y")]).1, vec![Ok(DefKind::Fn)]);
    let cycle = "mod a { pub use super::b::*; pub fn f() {} }\nmod b { pub use super::a::*; pub fn g() {} }\n";
    let (codes, found) = names(cycle, &[("a", "g"), ("b", "f")]);
    assert_eq!(
        (codes, found),
        (vec![], vec![Ok(DefKind::Fn), Ok(DefKind::Fn)])
    );
}

#[test]
fn duplicate_private_missing_and_ambiguous_names_are_reported() {
    assert_eq!(names("fn f() {}\nconst f: u8 = 1;\n", &[]).0, vec!["E0201"]);
    assert_eq!(
        names("mod a { pub fn x() {} }\nuse a::x;\nfn x() {}\n", &[]).0,
        vec!["E0201"]
    );
    assert_eq!(
        names("mod a { fn f() {} }\nuse a::f;\n", &[]).0,
        vec!["E0202"]
    );
    assert_eq!(names("use a::b;\n", &[]).0, vec!["E0200"]);
    assert_eq!(names("fn f() {}\nuse f::g;\n", &[]).0, vec!["E0205"]);
    let two = "mod a { pub fn x() {} }\nmod b { pub fn x() {} }\nuse a::*;\nuse b::*;\n";
    let (codes, found) = names(two, &[("", "x")]);
    assert_eq!((codes, found), (vec![], vec![Err(PathError::Ambiguous(0))]));
}
