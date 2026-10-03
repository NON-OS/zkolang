/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The edition 2025 compiler takes any text, and a program it compiles is evaluated on
 * inputs taken from the same bytes, without a panic either way.
 */

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_zkolang::{compile_source, evaluate};
use nonos_zkolang_fuzz::{text, values};

fuzz_target!(|data: &[u8]| {
    let Ok(ops) = compile_source(&text(data)) else {
        return;
    };
    let word = |v: Vec<i128>| -> Vec<u64> { v.into_iter().map(|x| x as u64).collect() };
    let public = word(values(data, 4));
    let secret = word(values(&data[..data.len() / 2], 4));
    let _ = evaluate(&ops, &public, &secret);
});
