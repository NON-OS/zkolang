// NONOS Operating System (AGPL-3.0-or-later)
//! The compose pipeline: record an inner's transition evaluation on a tape,
//! then lay the tape onto a strip of bounded-degree rows. `tape` records,
//! `replay` proves the recording faithful. The strip compiler follows.

mod layout;
mod pack;
mod strip;
mod replay;
mod schedule;
mod tape;

pub use layout::{strip_layout, Lane, LinForm, Row, Source, StripLayout};
pub use pack::{pack, Packed};
pub use replay::{const_mask, eval, mul_count, witnessed_muls};
pub use schedule::{audit, plan, Plan, Slot};
pub use strip::{check_region, strip_plan};
pub use tape::{begin, snapshot, Cell, Node};
