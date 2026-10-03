/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A zKølang format 7 proof verified in a page: `nox_verify` against the image and the
 * pins of `zkolang statement`, behind four exports and no imports.
 *
 * ```text
 * zk_alloc(len) -> ptr        a buffer the page fills, aligned to 8
 * zk_free(ptr, len)           give it back
 * zk_verify(statement, statement_len, proof, proof_len, words, words_len) -> code
 * zk_reason(buf, len) -> len  why the last zk_verify refused, UTF-8
 * ```
 *
 * The statement is laid end to end: the image's keccak256, the periodic root and the
 * parameter identity, 32 bytes each; the word count as a little-endian u64; the image.
 * The code is 0 for a proof that verifies, `nox_verify`'s refusal code for one that does
 * not, and 100 for a null or misaligned buffer or a short statement. Words are
 * little-endian u64.
 */

mod buffer;
mod reason;
mod verify;
