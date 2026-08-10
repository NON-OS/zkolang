// NONOS Operating System (AGPL-3.0-or-later)

//! Every binding the grand product groups enforce, and the forgery that proves
//! it fires. The single permutation argument must keep all of them rejecting:
//! an aggregate test stays green while one binding silently unbinds, because a
//! random tamper lands somewhere else.

/// A binding, and the test that violates exactly it with everything else honest.
pub(super) struct Binding {
    pub what: &'static str,
    pub forgery: &'static str,
}

pub(super) const BINDINGS: &[Binding] = &[
    Binding { what: "note compress tree stays chained", forgery: "note_edge" },
    Binding { what: "value limbs are the balance row limbs", forgery: "unbound_value" },
    Binding { what: "spent note is the pool membership leaf", forgery: "unproven_note" },
    Binding { what: "spent note is the association leaf", forgery: "unlisted_note" },
    Binding { what: "key hierarchy stays chained", forgery: "key_edge" },
    Binding { what: "derived spend key is the committed one", forgery: "foreign_key" },
    Binding { what: "absorbed commitment is the proven one", forgery: "owns" },
    Binding { what: "noteRoot is the walked pool root", forgery: "publics" },
    Binding { what: "assocRoot is the walked association root", forgery: "publics" },
    Binding { what: "nf0 and nf1 are the derived nullifiers", forgery: "publics" },
    Binding { what: "outCm0 and outCm1 are the created commitments", forgery: "publics" },
    Binding { what: "public amount and fee are the summed values", forgery: "publics" },
    Binding { what: "asset id is the committed asset", forgery: "publics" },
    Binding { what: "the batch clears at one price", forgery: "batch_price" },
];

/// The list is the acceptance criterion for the shrink, so it must not rot: a
/// binding added without a forgery is a binding nothing proves.
#[test]
fn every_binding_names_its_forgery() {
    for b in BINDINGS {
        assert!(!b.forgery.is_empty(), "{} has no forgery", b.what);
    }
    assert_eq!(BINDINGS.len(), 14);
}
