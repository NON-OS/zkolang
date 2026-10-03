/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Why the last verification refused, kept for the page to read. */

use std::sync::Mutex;

static REASON: Mutex<&'static str> = Mutex::new("");

/** Keep `why` as the last verification's reason. */
pub(crate) fn record(why: &'static str) {
    if let Ok(mut r) = REASON.lock() {
        *r = why;
    }
}

/**
 * # Safety
 * `buf` is null or points to `len` writable bytes. Copies at most `len` bytes of the last
 * reason and returns its full length.
 */
#[no_mangle]
pub unsafe extern "C" fn zk_reason(buf: *mut u8, len: usize) -> usize {
    let r = REASON.lock().map(|g| *g).unwrap_or("");
    if !buf.is_null() {
        /* SAFETY: the caller's contract gives `len` writable bytes at `buf`. */
        unsafe { core::ptr::copy_nonoverlapping(r.as_ptr(), buf, r.len().min(len)) };
    }
    r.len()
}
