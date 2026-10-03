/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Buffers the page fills: every one aligned to 8, so one call serves bytes and words. */

use std::alloc::{alloc, dealloc, Layout};

fn layout(len: usize) -> Option<Layout> {
    Layout::from_size_align(len, 8).ok()
}

/** A buffer of `len` bytes, or null when `len` is 0 or too large to lay out. */
#[no_mangle]
pub extern "C" fn zk_alloc(len: usize) -> *mut u8 {
    match layout(len) {
        /* SAFETY: a nonzero size and a valid layout. */
        Some(l) if len != 0 => unsafe { alloc(l) },
        _ => core::ptr::null_mut(),
    }
}

/**
 * # Safety
 * `ptr` came from `zk_alloc(len)` with this `len` and is not used after.
 */
#[no_mangle]
pub unsafe extern "C" fn zk_free(ptr: *mut u8, len: usize) {
    if let (false, Some(l)) = (ptr.is_null() || len == 0, layout(len)) {
        /* SAFETY: the caller's contract: `ptr` is `zk_alloc(len)`'s, with this layout. */
        unsafe { dealloc(ptr, l) }
    }
}
