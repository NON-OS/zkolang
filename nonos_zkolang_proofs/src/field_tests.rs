/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The base field, checked against its definition. Every proof rests on Goldilocks
//! arithmetic being exactly arithmetic modulo `p = 2^64 - 2^32 + 1`, so the ring
//! laws and the reference reductions are pinned directly here rather than only
//! through the circuits that use them. Addition, subtraction, and negation are held
//! to the plain modular result; multiplication and squaring to a `u128` modulo; the
//! inverse to the defining relation `a * a^-1 = 1`. The generator is deterministic,
//! so any failure names the operands that produced it.

use nonos_stark::field::Fp;

const P: u128 = 0xFFFF_FFFF_0000_0001;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn fp(&mut self) -> Fp {
        Fp::from_u64(self.next())
    }
}

fn u(x: Fp) -> u128 {
    x.value() as u128
}

#[test]
fn add_sub_neg_match_modular_arithmetic() {
    let mut rng = Rng(0x000F_1E1D_0000_0001);
    for _ in 0..3_000_000 {
        let a = rng.fp();
        let b = rng.fp();
        assert!(
            a.value() < P as u64 && b.value() < P as u64,
            "operand not canonical"
        );

        // Addition and subtraction agree with the modular result.
        assert_eq!(u(a + b), (u(a) + u(b)) % P, "a + b");
        assert_eq!(u(a - b), (u(a) + P - u(b)) % P, "a - b");

        // Negation is the additive inverse, and it round-trips subtraction.
        assert_eq!(u((-a) + a), 0, "-a + a");
        assert_eq!((a + b) - b, a, "(a + b) - b");

        // Commutativity of the two operations that have it.
        assert_eq!(a + b, b + a, "a + b commutes");
    }
    assert_eq!(-Fp::ZERO, Fp::ZERO, "negating zero");
}

#[test]
fn mul_and_square_match_the_reference_and_the_ring_laws() {
    let mut rng = Rng(0x000F_F1CE_1234_5678);
    for _ in 0..3_000_000 {
        let a = rng.fp();
        let b = rng.fp();
        let c = rng.fp();

        assert_eq!(u(a * b), (u(a) * u(b)) % P, "a * b");
        assert_eq!(a * b, b * a, "a * b commutes");
        assert_eq!(a.square(), a * a, "a^2");

        // Distributivity ties addition and multiplication together, the one law a
        // broken carry correction in either would expose.
        assert_eq!(a * (b + c), a * b + a * c, "distributivity");
    }
}

#[test]
fn inverse_satisfies_its_defining_relation() {
    // The inverse is not otherwise checked in isolation, yet the DEEP quotient and
    // every ordered comparison depend on it. a * a^-1 must be one for every nonzero
    // a, and the convention inv(0) = 0 must hold.
    assert_eq!(Fp::ZERO.inv(), Fp::ZERO, "inv(0)");
    assert_eq!(Fp::ONE.inv(), Fp::ONE, "inv(1)");

    let mut rng = Rng(0x0BAD_F00D_CAFE);
    for _ in 0..1_000_000 {
        let a = rng.fp();
        if a == Fp::ZERO {
            continue;
        }
        assert_eq!(a * a.inv(), Fp::ONE, "a * inv(a) for a = {}", a.value());
    }
}
