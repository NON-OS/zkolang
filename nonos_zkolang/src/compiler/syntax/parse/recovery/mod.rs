/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Recovery. After an error the parser skips to a point where parsing can resume, closing
 * what the erroneous construct opened, so one run reports many errors and every error is
 * reported once.
 */

mod block_errors;
mod bracket_end;
mod owed;
mod recover;
mod recover_item;
mod recover_owed;
mod reserved;
mod skip_stmt;
mod stray_gap;

pub(super) use recover_item::starts_item;
