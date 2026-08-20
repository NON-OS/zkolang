// NONOS Operating System (AGPL-3.0-or-later)

use super::bind_index::index_classes;
use super::bind_key::key_classes;
use super::bind_note::note_classes;
use crate::shield::wire_class::Class;
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
    /// First row of each spent note's position recovery chain.
    pub index: Vec<usize>,
    pub depth: usize,
    pub balance: usize,
}

pub(crate) fn classes(l: &Layout) -> Vec<Class> {
    let mut c = note_classes(l);
    c.extend(key_classes(l));
    c.extend(index_classes(l));
    c
}
