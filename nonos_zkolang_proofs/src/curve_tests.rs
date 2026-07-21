/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Elliptic curve point addition, proven end to end. A reference addition over the field
//! picks two points, derives the unique curve through them, and computes their group-law sum;
//! the point_add circuit is then required to accept that opening and to reject a tampered sum.
//! The reference is plain field arithmetic, so this checks that the language computes the group
//! law the curve gadget names, not a model of it.

use std::fs;
use std::path::PathBuf;

use nonos_stark::field::Fp;
use nonos_zkolang::{expand_includes, prove_source_with_inputs};

fn stdlib_resolve(name: &str) -> Option<String> {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    fs::read_to_string(base.join("stdlib").join(name)).ok()
}

fn load(rel: &str) -> String {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    let src = fs::read_to_string(base.join(rel)).expect("read");
    expand_includes(&src, &mut stdlib_resolve).expect("expand")
}

// Pick two points, derive the curve `y^2 = x^3 + a*x + b` through them (unique for distinct x),
// and add them by the group law. Returns `(a, b, x1, y1, x2, y2, x3, y3)` as field values.
fn reference() -> [u64; 8] {
    let (x1, y1) = (Fp::from_u64(2), Fp::from_u64(3));
    let (x2, y2) = (Fp::from_u64(5), Fp::from_u64(7));
    let cube = |x: Fp| x * x * x;
    // a = (y1^2 - y2^2 - (x1^3 - x2^3)) / (x1 - x2);  b = y1^2 - x1^3 - a*x1.
    let a = (y1 * y1 - y2 * y2 - (cube(x1) - cube(x2))) * (x1 - x2).inv();
    let b = y1 * y1 - cube(x1) - a * x1;
    // The chord slope, then the reflected third intersection.
    let s = (y2 - y1) * (x2 - x1).inv();
    let x3 = s * s - x1 - x2;
    let y3 = s * (x1 - x3) - y1;
    // All three points must lie on the derived curve.
    let on = |x: Fp, y: Fp| y * y - (cube(x) + a * x + b);
    assert_eq!(on(x1, y1), Fp::ZERO);
    assert_eq!(on(x2, y2), Fp::ZERO);
    assert_eq!(on(x3, y3), Fp::ZERO);
    [
        a.value(),
        b.value(),
        x1.value(),
        y1.value(),
        x2.value(),
        y2.value(),
        x3.value(),
        y3.value(),
    ]
}

// Pick a point and a curve coefficient `a`, derive `b` so the point lies on the curve, and
// double it by the tangent-line group law. Returns `(a, b, x, y, x3, y3)` as field values.
fn reference_double() -> [u64; 6] {
    let (x, y) = (Fp::from_u64(2), Fp::from_u64(3));
    let a = Fp::from_u64(5);
    let cube = |x: Fp| x * x * x;
    let b = y * y - cube(x) - a * x;
    let (two, three) = (Fp::from_u64(2), Fp::from_u64(3));
    // s = (3x^2 + a) / (2y);  x3 = s^2 - 2x;  y3 = s(x - x3) - y.
    let s = (three * x * x + a) * (two * y).inv();
    let x3 = s * s - x - x;
    let y3 = s * (x - x3) - y;
    let on = |x: Fp, y: Fp| y * y - (cube(x) + a * x + b);
    assert_eq!(on(x, y), Fp::ZERO);
    assert_eq!(on(x3, y3), Fp::ZERO);
    [
        a.value(),
        b.value(),
        x.value(),
        y.value(),
        x3.value(),
        y3.value(),
    ]
}

#[test]
fn point_addition_proves_and_binds_the_sum() {
    let src = load("examples/curve/point_add.zkl");
    let v = reference();
    let report = prove_source_with_inputs(&src, &v).expect("run");
    assert!(report.verified, "an honest point addition was rejected");
    assert_eq!(report.outputs, vec![v[6], v[7]], "the sum coordinates");

    // A wrong sum has no proof: the group-law and on-curve constraints fail together.
    let mut bad = v;
    bad[6] = bad[6].wrapping_add(1);
    let verified = prove_source_with_inputs(&src, &bad)
        .map(|r| r.verified)
        .unwrap_or(false);
    assert!(!verified, "a forged point sum verified");
}

#[test]
fn point_doubling_proves_and_binds_the_double() {
    let src = load("examples/curve/point_double.zkl");
    let v = reference_double();
    let report = prove_source_with_inputs(&src, &v).expect("run");
    assert!(report.verified, "an honest point doubling was rejected");
    assert_eq!(report.outputs, vec![v[4], v[5]], "the double coordinates");

    let mut bad = v;
    bad[4] = bad[4].wrapping_add(1);
    let verified = prove_source_with_inputs(&src, &bad)
        .map(|r| r.verified)
        .unwrap_or(false);
    assert!(!verified, "a forged point double verified");
}

// Compute `5*P` by double-and-add over the field: P, 2P, 4P, then 5P = 4P + P. Returns
// `(a, b, px, py, p2x, p2y, p4x, p4y, p5x, p5y)` as field values.
fn reference_scalar() -> [u64; 10] {
    let (px, py) = (Fp::from_u64(2), Fp::from_u64(3));
    let a = Fp::from_u64(5);
    let cube = |x: Fp| x * x * x;
    let b = py * py - cube(px) - a * px;
    let dbl = |x: Fp, y: Fp| {
        let s = (Fp::from_u64(3) * x * x + a) * (Fp::from_u64(2) * y).inv();
        let x3 = s * s - x - x;
        (x3, s * (x - x3) - y)
    };
    let add = |x1: Fp, y1: Fp, x2: Fp, y2: Fp| {
        let s = (y2 - y1) * (x2 - x1).inv();
        let x3 = s * s - x1 - x2;
        (x3, s * (x1 - x3) - y1)
    };
    let (p2x, p2y) = dbl(px, py);
    let (p4x, p4y) = dbl(p2x, p2y);
    let (p5x, p5y) = add(p4x, p4y, px, py);
    let on = |x: Fp, y: Fp| y * y - (cube(x) + a * x + b);
    for (x, y) in [(px, py), (p2x, p2y), (p4x, p4y), (p5x, p5y)] {
        assert_eq!(on(x, y), Fp::ZERO);
    }
    [
        a.value(),
        b.value(),
        px.value(),
        py.value(),
        p2x.value(),
        p2y.value(),
        p4x.value(),
        p4y.value(),
        p5x.value(),
        p5y.value(),
    ]
}

#[test]
fn scalar_multiplication_proves_five_p() {
    let src = load("examples/curve/scalar_mul.zkl");
    let v = reference_scalar();
    let report = prove_source_with_inputs(&src, &v).expect("run");
    assert!(report.verified, "an honest 5*P was rejected");
    assert_eq!(report.outputs, vec![v[8], v[9]], "the 5*P coordinates");

    // A wrong final point has no proof: a broken rung fails the chain.
    let mut bad = v;
    bad[8] = bad[8].wrapping_add(1);
    let verified = prove_source_with_inputs(&src, &bad)
        .map(|r| r.verified)
        .unwrap_or(false);
    assert!(!verified, "a forged 5*P verified");
}
