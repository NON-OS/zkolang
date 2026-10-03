/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Tests for reading integer literals. */

use super::number::{int_literal, IntLit, IntLitError};
use crate::compiler::syntax::int_ty::IntTy;

#[test]
fn reads_every_radix_and_suffix() {
    assert_eq!(int_literal("1_000").map(|l| l.value), Ok(1000));
    assert_eq!(int_literal("0xff").map(|l| l.value), Ok(255));
    assert_eq!(int_literal("0o17").map(|l| l.value), Ok(15));
    assert_eq!(int_literal("0b1010").map(|l| l.value), Ok(10));
    assert_eq!(
        int_literal("7u8"),
        Ok(IntLit {
            value: 7,
            suffix: Some(IntTy::U8)
        })
    );
    assert_eq!(
        int_literal("18446744073709551615").map(|l| l.value),
        Ok(u64::MAX)
    );
}

#[test]
fn refuses_malformed_literals() {
    assert_eq!(int_literal("0x"), Err(IntLitError::NoDigits));
    assert_eq!(int_literal("0b102"), Err(IntLitError::BadDigit));
    assert_eq!(int_literal("12abc"), Err(IntLitError::BadSuffix));
    assert_eq!(
        int_literal("18446744073709551616"),
        Err(IntLitError::TooLarge)
    );
    assert_eq!(int_literal("0o9"), Err(IntLitError::BadDigit));
}
