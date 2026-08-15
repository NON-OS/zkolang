// NONOS Operating System (AGPL-3.0-or-later)
//! The targeted forgeries the reject gate proves the assembly catches. Each
//! keeps its region internally consistent where possible, so the rejection
//! comes from the binding under attack rather than a local constraint.

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tamper {
    None,
    /// A DEEP trace value cut loose from its authenticated opening.
    ReboundTraceValue,
    /// The deep and comp values authenticated under each other's root.
    SwappedRoot,
    /// A DEEP batching coefficient the transcript never squeezed.
    OffTranscriptCoeff,
    /// The periodic columns recomputed at a point the composition never used.
    /// Both regions stay internally consistent, so only the periodic binding
    /// can reject it.
    PeriodicOffPoint,
    /// A fold chain descending on a beta the FRI transcript never squeezed. The
    /// chain is built from the beta it uses, so the fold algebra holds and only
    /// the transcript binding is left to catch it.
    OffTranscriptBeta,
    /// A DEEP divisor derived from an index other than the one query k's own
    /// openings authenticate. The point chain is an honest walk of whatever
    /// index it was given, so only the index binding ties it to the opened path.
    ForeignConsistencyIndex,
}
