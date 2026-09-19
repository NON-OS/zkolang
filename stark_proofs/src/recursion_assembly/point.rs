// NONOS Operating System (AGPL-3.0-or-later)
//! Which outer an emitter assembles, decided once.
//!
//! The settlement outer and the transfer outer are the same recursion over
//! different inners, and every binary that emits something about one of them
//! had grown its own copy of the choice: the flag, the query count, the
//! environment check, the assembly call. Three copies agreed today. The fourth
//! would have been the one that did not, and a shape emitted over the wrong
//! inner is indistinguishable from a right one on inspection.
//!
//! A transfer inner is proved at rate 1/4 and the outer authenticates its
//! domain at whatever `NONOS_INNER_EXTRA` says. The check that those agree
//! lives here, so a binary cannot assemble the transfer point without it.

use super::build::{assemble_over_wired, assemble_real_wired, Wiring};
use super::{inner, Assembly, Tamper};
use crate::shield_params::{deployment, transfer};

/// The outer to assemble.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Point {
    /// The recursion over the settlement inner, at the registered keys' point.
    Settlement,
    /// The recursion over a transfer inner proved at `queries` queries.
    Transfer { queries: usize },
    /// The recursion over a spend of notes that exist in the deployed pool,
    /// rather than over a fixture.
    ///
    /// Shape-identical to `Settlement`: the outer's periodic columns are a
    /// property of the inner's shape, not of its values, and a live spend is
    /// the same join-split at the same depth and query count. So this point
    /// has the settlement point's periodic root and its cached tree, and the
    /// proof costs the steady state rather than a tree build. If that ever
    /// stopped being true the emitter's own root check would refuse it,
    /// which is why the supplied root is worth passing.
    Spend { recipient: u64, clearing_price: u64 },
    /// The recursion over `inners` settlement inners under one outer, which is
    /// what a batch of that many intents is.
    ///
    /// The batch changes the trace length and nothing else: the width, the
    /// degree, the constraint set and the group count are the single-inner
    /// ones, and each inner carries its own statement into the outer's
    /// publics. It is a distinct point all the same, because the trace length
    /// sets the evaluation domain, the domain sets the periodic root, and a
    /// verifier bakes that root. So a pool can only settle a batch size whose
    /// structure was emitted and whose verifier was deployed.
    Batch { inners: usize },
}

impl Point {
    /// Read the point off a command line: `transfer` selects the transfer
    /// outer, `queries=N` sets its inner query count, and neither means the
    /// settlement outer.
    pub fn from_args(args: &[String]) -> Point {
        let at_transfer = args.iter().any(|a| a == "transfer");
        let queries = args
            .iter()
            .find_map(|a| {
                a.strip_prefix("queries=")
                    .and_then(|v| v.parse::<usize>().ok())
            })
            .unwrap_or(transfer::N_QUERIES);
        let batch = args.iter().find_map(|a| {
            a.strip_prefix("batch=")
                .and_then(|v| v.parse::<usize>().ok())
        });
        /*
         * `spend` proves against the notes in the deployed pool. The payout
         * address and the clearing price are the only parts of that intent a
         * caller chooses; everything else is fixed by what is on chain.
         */
        let spend = args.iter().any(|a| a == "spend");
        let recipient = args
            .iter()
            .find_map(|a| {
                a.strip_prefix("recipient=")
                    .and_then(|v| u64::from_str_radix(v.trim_start_matches("0x"), 16).ok())
            })
            .unwrap_or(0x7408_ae4c);
        let clearing_price = args
            .iter()
            .find_map(|a| a.strip_prefix("price=").and_then(|v| v.parse::<u64>().ok()))
            .unwrap_or(1_000_000);
        match (at_transfer, spend, batch) {
            (true, _, _) => Point::Transfer { queries },
            (false, true, _) => Point::Spend {
                recipient,
                clearing_price,
            },
            (false, false, Some(n)) if n > 1 => Point::Batch { inners: n },
            _ => Point::Settlement,
        }
    }

    /// Whether an argument belongs to this selector rather than to the caller.
    /// Every emitter takes its output path as the first argument that is not a
    /// flag, so a flag missing from a filter becomes a filename.
    pub fn is_flag(arg: &str) -> bool {
        arg == "transfer"
            || arg == "spend"
            || arg.starts_with("queries=")
            || arg.starts_with("batch=")
            || arg.starts_with("recipient=")
            || arg.starts_with("price=")
    }

    pub fn name(self) -> &'static str {
        match self {
            Point::Settlement => "settlement",
            Point::Transfer { .. } => "transfer",
            Point::Batch { .. } => "batch",
            Point::Spend { .. } => "spend",
        }
    }

    /// How many inner statements ride this outer.
    pub fn inners(self) -> usize {
        match self {
            Point::Settlement | Point::Transfer { .. } | Point::Spend { .. } => 1,
            Point::Batch { inners } => inners,
        }
    }

    /// The inner's query count, which the outer's shape depends on.
    pub fn inner_queries(self) -> usize {
        match self {
            Point::Settlement | Point::Batch { .. } | Point::Spend { .. } => inner::NQ,
            Point::Transfer { queries } => queries,
        }
    }

    /// The blowup the inner was proved at, which the outer authenticates.
    pub fn inner_extra(self) -> u32 {
        match self {
            Point::Settlement | Point::Batch { .. } | Point::Spend { .. } => inner::extra(),
            Point::Transfer { .. } => transfer::EXTRA_BLOWUP_BITS,
        }
    }

    /// How every emitted artifact argues its copy constraint.
    ///
    /// One value, read by every binary that emits a structure, a proof or a
    /// set of constraint values, because three binaries that each decided this
    /// for themselves would agree until the day one of them did not, and a
    /// proof built over a different wiring than the structure describes is
    /// indistinguishable from a right one until a verifier refuses it.
    pub fn emit_wiring() -> Wiring {
        Wiring::Chained
    }

    /// Assemble the outer with its witness, refusing a transfer point whose
    /// environment would authenticate the inner at the wrong rate.
    pub fn assemble(self) -> Result<Assembly, String> {
        self.assemble_wired(Wiring::Packed)
    }

    /// The same, with the copy constraint argued either way.
    ///
    /// The emits take `Chained`, which spends 368 sigma columns where the
    /// packed form spends 2,025. The gates and the reject cases stay on
    /// `Packed`, so a circuit nobody moved keeps measuring what it measured.
    pub fn assemble_wired(self, wiring: Wiring) -> Result<Assembly, String> {
        /*
         * Every point but the transfer proves its inner at the deployment
         * rate, and the environment must not be able to lower it silently.
         *
         * `NONOS_INNER_EXTRA` exists so a wiring gate can re-prove an inner
         * cheaply, and its own comment says emits and vectors never set it. A
         * script set it for a transfer step and it applied to the settlement
         * steps beside it, so a settlement set went out with its inner at
         * rate 1/4: 32 queries at 2 bits plus 16 of grinding is 80 bits, not
         * 144, and nothing emitted named the inner rate so nothing caught it.
         * A verifier was deployed on it and a pool gated on that.
         *
         * The comment is now a refusal.
         */
        if self
            != (Point::Transfer {
                queries: self.inner_queries(),
            })
            && inner::extra() != deployment::EXTRA_BLOWUP_BITS
        {
            return Err(format!(
                "the {} point proves its inner at the deployment blowup {}, but \
                 NONOS_INNER_EXTRA says {}; refusing to emit an artifact whose \
                 soundness is not the one its parameters claim",
                self.name(),
                deployment::EXTRA_BLOWUP_BITS,
                inner::extra()
            ));
        }
        match self {
            Point::Settlement => Ok(assemble_real_wired(Tamper::None, wiring)),
            Point::Transfer { queries } => {
                if inner::extra() != transfer::EXTRA_BLOWUP_BITS {
                    return Err(format!(
                        "the transfer assembly needs NONOS_INNER_EXTRA={} (found {}); refusing \
                         to assemble a shape that does not match its inner",
                        transfer::EXTRA_BLOWUP_BITS,
                        inner::extra()
                    ));
                }
                let h = inner::hasher();
                let inner_at = inner::shield_join_split_at(
                    &h,
                    queries,
                    transfer::GRIND_BITS,
                    transfer::EXTRA_BLOWUP_BITS,
                );
                Ok(assemble_over_wired(&h, inner_at, Tamper::None, usize::MAX, wiring))
            }
            /*
             * A batch is `inners` copies of the settlement inner laid end to
             * end. `assemble_many` returns an aggregate carrying one layout
             * per inner; they share the trace and differ only in where their
             * regions sit, so the shape every consumer reads is the first
             * one, and the count travels beside it as the point.
             */
            Point::Spend {
                recipient,
                clearing_price,
            } => {
                let h = inner::hasher();
                let js = crate::shield::live::unshield(recipient, clearing_price);
                /*
                 * A real spend is hiding. The seed is derived from the
                 * intent so a rebuild of the same spend reproduces the same
                 * proof, which is what makes an artifact checkable; a
                 * production signer draws it from the capsule's CSPRNG
                 * instead, because two spends under one seed would cancel
                 * their blinds and hand back the witness.
                 */
                let mut seed = [crate::crypto::stark::field::Fp::ZERO; 4];
                for (i, s) in seed.iter_mut().enumerate() {
                    *s = js.intent[i];
                }
                let inner_live = inner::shield_join_split_of(&h, js, Some(&seed));
                Ok(assemble_over_wired(&h, inner_live, Tamper::None, usize::MAX, wiring))
            }
            Point::Batch { inners } => {
                if inners == 0 {
                    return Err("a batch needs at least one inner".into());
                }
                let h = inner::hasher();
                let batch = (0..inners)
                    .map(|_| inner::shield_join_split(&h))
                    .collect::<alloc::vec::Vec<_>>();
                let agg = super::build::assemble_many_wired(&h, batch, usize::MAX, wiring);
                Ok(Assembly {
                    wired: agg.wired,
                    witness: agg.witness,
                    lay: agg
                        .lays
                        .into_iter()
                        .next()
                        .expect("a batch carries one layout per inner"),
                    publics: agg.publics,
                    n_groups: agg.n_groups,
                    region_offsets: agg.region_offsets,
                    kind_bodies: agg.kind_bodies,
                })
            }
        }
    }

    /// The assembly and the raw binds behind its groups, for anything that
    /// has to look at the wiring rather than at the argument built over it.
    /// The binds come from the layout the assembly settled on, so the two
    /// describe one circuit.
    pub fn assemble_with_binds(self) -> Result<(Assembly, Vec<super::groups::Bind>), String> {
        let asm = self.assemble()?;
        let binds = super::build::binds_for(&asm.lay);
        Ok((asm, binds))
    }
}
