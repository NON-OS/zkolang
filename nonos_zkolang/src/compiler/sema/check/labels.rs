/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The labels that qualify one part of a labelled value. */

use crate::compiler::tir::Labels;

/**
 * The labels of the part at `path` of a value labelled `labels`: a label on the part or on
 * what encloses it qualifies the whole part, and a label inside it keeps its place there.
 */
pub(crate) fn sub_labels(labels: &Labels, path: &[u32]) -> Labels {
    let mut out = Labels::default();
    for (p, l) in &labels.0 {
        if path.starts_with(p) {
            out.0.push((alloc::vec::Vec::new(), *l));
        } else if p.starts_with(path) {
            out.0
                .push((p.get(path.len()..).unwrap_or(&[]).to_vec(), *l));
        }
    }
    out
}
