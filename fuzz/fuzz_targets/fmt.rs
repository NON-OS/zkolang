/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The formatter never changes a token or a comment, and its layout is a fixed point: a
 * formatted file formats to itself.
 */

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_zkolang::compiler::fmt::{format, FmtError};
use nonos_zkolang_fuzz::text;

fuzz_target!(|data: &[u8]| {
    match format(&text(data)) {
        Err(FmtError::Changed) => panic!("the layout changed a token or a comment"),
        Err(FmtError::Syntax(_)) => {}
        Ok(out) => match format(&out) {
            Ok(again) => assert_eq!(again, out, "a formatted file formats differently"),
            Err(_) => panic!("a formatted file no longer formats"),
        },
    }
});
