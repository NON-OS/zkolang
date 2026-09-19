// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What a long proof is doing, and whether it should stop.
//!
//! A proof runs for a minute on a phone and hours on a server, and until now it
//! said nothing between its first instruction and its last. A shell could show
//! an indeterminate bar and a cancel button that did nothing, which is two
//! pieces of interface making a claim the code could not keep.
//!
//! The contract here is deliberately the cheapest one that works: the caller
//! polls, the prover never calls back. A callback from a rayon worker into
//! Kotlin or Swift is a whole extra contract about threads and lifetimes; a
//! poll is an atomic load from whatever thread the shell already has. Phase
//! timings are written as they complete, so the same object carries the live
//! position and the post-run breakdown without a second channel.
//!
//! Cancellation is honoured at phase boundaries. That is coarse by design: a
//! token read inside the parallel maps would cost a load per element for a
//! button a person presses once. On the shapes that matter a phase is seconds,
//! which is honest for an interface, and the alternative is a promise kept to
//! the microsecond that nobody asked for.

use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// The phases a proof passes through, in order. The index a caller polls is an
/// index into this table.
pub const PHASE_NAMES: [&str; 6] = [
    "trace commitment",
    "periodic commitment",
    "composition",
    "out of domain",
    "deep and fri",
    "query openings",
];

/// The number of phases, so a shell can size its own display without matching
/// on the table.
pub const PHASE_COUNT: usize = PHASE_NAMES.len();

/// Shared between a prover and whoever is watching it.
///
/// Every field is an atomic and nothing here allocates, so it works under
/// `no_std` and can be handed across an FFI boundary as a pointer without a
/// lock or a runtime.
#[derive(Debug)]
pub struct Progress {
    phase: AtomicUsize,
    cancelled: AtomicBool,
    done: AtomicBool,
    micros: [AtomicU64; PHASE_COUNT],
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

impl Progress {
    pub const fn new() -> Progress {
        Progress {
            phase: AtomicUsize::new(0),
            cancelled: AtomicBool::new(false),
            done: AtomicBool::new(false),
            micros: [
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
            ],
        }
    }

    /// The phase now running, as an index into [`PHASE_NAMES`]. A shell polls
    /// this; it never blocks and never waits on the prover.
    pub fn phase(&self) -> usize {
        self.phase.load(Ordering::Relaxed)
    }

    /// The name of the phase now running, or the last one if the proof is done.
    pub fn phase_name(&self) -> &'static str {
        PHASE_NAMES[self.phase().min(PHASE_COUNT - 1)]
    }

    /// Microseconds a completed phase took, or zero while it is still running
    /// or has not started. Readable during the run as well as after it.
    pub fn phase_micros(&self, phase: usize) -> u64 {
        self.micros
            .get(phase)
            .map(|m| m.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    /// Whether the proof ran to completion. Distinguishes a finished proof from
    /// one that stopped at the last phase, which the index alone cannot.
    pub fn finished(&self) -> bool {
        self.done.load(Ordering::Relaxed)
    }

    /// Ask the prover to stop. It is honoured at the next phase boundary, so
    /// this returns immediately and the proof ends shortly after.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    /// Record a finished phase and step to the next. Called by the prover.
    pub(crate) fn advance(&self, phase: usize, micros: u64) {
        if let Some(slot) = self.micros.get(phase) {
            slot.store(micros, Ordering::Relaxed);
        }
        self.phase.store(phase + 1, Ordering::Relaxed);
    }

    pub(crate) fn finish(&self) {
        self.done.store(true, Ordering::Relaxed);
    }
}

/*
 * The reporter a prover drives, kept here rather than in either prover.
 *
 * Both provers walk the same six phases and differ in their hash and their
 * blinding, so two copies of this would be two statements of one thing, which
 * is the drift every other duplicate in this system has eventually produced.
 * It times each phase, prints it when the build has somewhere to print, writes
 * it to the watch, and answers whether the caller has asked to stop.
 */
pub(crate) struct Phase<'a> {
    #[cfg(feature = "parallel")]
    at: std::time::Instant,
    index: usize,
    watch: Option<&'a Progress>,
}

impl<'a> Phase<'a> {
    pub(crate) fn start(watch: Option<&'a Progress>) -> Phase<'a> {
        Phase {
            #[cfg(feature = "parallel")]
            at: std::time::Instant::now(),
            index: 0,
            watch,
        }
    }

    /// Whether the caller has asked to stop. Read once per phase, which is the
    /// granularity the boundary offers and the granularity a button needs.
    pub(crate) fn cancelled(&self) -> bool {
        self.watch.map(|w| w.is_cancelled()).unwrap_or(false)
    }

    #[allow(unused_variables)]
    pub(crate) fn done(&mut self, what: &str) {
        #[allow(unused_mut, unused_assignments)]
        let mut micros = 0u64;
        #[cfg(feature = "parallel")]
        {
            let now = std::time::Instant::now();
            let took = now.duration_since(self.at);
            micros = took.as_micros() as u64;
            std::eprintln!("[prove] {what} in {took:?}");
            self.at = now;
        }
        if let Some(w) = self.watch {
            w.advance(self.index, micros);
        }
        self.index += 1;
    }

    /// Mark the whole proof complete, which the index alone cannot say.
    pub(crate) fn finish(&self) {
        if let Some(w) = self.watch {
            w.finish();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The table and the count are one statement, and a shell sizing a display
    /// from the count must not be able to index past the names.
    #[test]
    fn the_phase_table_and_count_agree() {
        assert_eq!(PHASE_COUNT, PHASE_NAMES.len());
        let p = Progress::new();
        for i in 0..PHASE_COUNT {
            p.phase.store(i, Ordering::Relaxed);
            assert!(!p.phase_name().is_empty());
        }
    }

    /// A phase index past the end still names something rather than panicking,
    /// because the prover sets the index one past the last phase when it ends
    /// and a shell polling at that moment must not take the process down.
    #[test]
    fn polling_after_the_last_phase_is_safe() {
        let p = Progress::new();
        p.advance(PHASE_COUNT - 1, 42);
        p.finish();
        assert_eq!(p.phase(), PHASE_COUNT);
        assert_eq!(p.phase_name(), PHASE_NAMES[PHASE_COUNT - 1]);
        assert_eq!(p.phase_micros(PHASE_COUNT - 1), 42);
        assert!(p.finished());
    }

    #[test]
    fn cancelling_is_visible_immediately() {
        let p = Progress::new();
        assert!(!p.is_cancelled());
        p.cancel();
        assert!(p.is_cancelled());
        assert!(!p.finished());
    }
}
