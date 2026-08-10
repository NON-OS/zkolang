// NONOS Operating System (AGPL-3.0-or-later)

use super::bind_key::key_groups;
use super::bind_note::note_groups;
use crate::crypto::stark::air::GpGroup;
use alloc::vec::Vec;

pub(crate) struct Layout {
    pub span: usize,
    pub span_op: usize,
    pub note: Vec<usize>,
    pub member: Vec<usize>,
    pub key: Vec<usize>,
    pub key_span: Vec<usize>,
    pub leaf_col: Vec<usize>,
    pub assoc: Vec<usize>,
    pub assoc_col: Vec<usize>,
    pub depth: usize,
    pub balance: usize,
}

pub(crate) fn groups(l: &Layout) -> Vec<GpGroup> {
    let mut g = note_groups(l);
    g.extend(key_groups(l));
    g
}
