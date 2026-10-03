/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The verification export. */

use nonos_zkolang_format7::verify_pinned;

use super::reason::record;

const BAD_BUFFER: u32 = 100;

/** A statement's pins, its word count and its image. */
type Parts<'a> = ([[u8; 32]; 3], usize, &'a [u8]);

/** The pins, the word count and the image laid end to end in `s`, if it holds them. */
fn statement(s: &[u8]) -> Option<Parts<'_>> {
    let pin = |k: usize| <[u8; 32]>::try_from(s.get(32 * k..32 * k + 32)?).ok();
    let n = u64::from_le_bytes(s.get(96..104)?.try_into().ok()?);
    Some((
        [pin(0)?, pin(1)?, pin(2)?],
        usize::try_from(n).ok()?,
        s.get(104..)?,
    ))
}

/**
 * # Safety
 * `statement` points to `statement_len` readable bytes, `proof` to `proof_len`, and
 * `words` to `words_len` readable u64s; all from `zk_alloc`, which aligns them to 8.
 */
#[no_mangle]
pub unsafe extern "C" fn zk_verify(
    statement_ptr: *const u8,
    statement_len: usize,
    proof: *const u8,
    proof_len: usize,
    words: *const u64,
    words_len: usize,
) -> u32 {
    let null = statement_ptr.is_null() || proof.is_null() || words.is_null();
    if null || !(words as usize).is_multiple_of(8) {
        record("a null or misaligned buffer");
        return BAD_BUFFER;
    }
    /* SAFETY: non-null and aligned, and the caller's contract gives the lengths. */
    let (s, proof, words) = unsafe {
        (
            core::slice::from_raw_parts(statement_ptr, statement_len),
            core::slice::from_raw_parts(proof, proof_len),
            core::slice::from_raw_parts(words, words_len),
        )
    };
    let Some((pins, n_words, image)) = statement(s) else {
        record("a statement shorter than its pins and word count");
        return BAD_BUFFER;
    };
    match verify_pinned(image, pins, n_words, proof, words) {
        Ok(()) => {
            record("");
            0
        }
        Err((r, why)) => {
            record(why);
            r.code()
        }
    }
}
