// NONOS Operating System (AGPL-3.0-or-later)

use super::domain::{tag, NULL_DOMAIN, SPEND_DOMAIN};
use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;

pub struct Keys {
    pub spend_pk: [Fp; RATE],
    pub nk: [Fp; RATE],
}

/// Both keys descend from one secret. A free nullifier key would let the same
/// commitment yield a fresh nullifier per key, and would let anyone who has seen
/// a commitment retire a note they do not own.
pub fn derive(h: &Poseidon, sk: [Fp; RATE]) -> Keys {
    Keys {
        spend_pk: h.compress(&sk, &tag(SPEND_DOMAIN)),
        nk: h.compress(&sk, &tag(NULL_DOMAIN)),
    }
}

/// The leaf position is in the preimage. Identical notes commit identically, so
/// without it two deposits share a nullifier and spending one locks the other.
pub fn nullifier(h: &Poseidon, nk: [Fp; RATE], cm: [Fp; RATE], leaf_index: u64) -> [Fp; RATE] {
    let t = h.compress(&nk, &cm);
    h.compress(&t, &tag(leaf_index))
}

#[cfg(test)]
mod handoff_tests {
    use super::*;
    use crate::shield::note::POOL_LOG_ROUNDS;

    /// Unpack a 32 byte digest as the circuit's four limbs: the bytes are
    /// big endian over `[limb3, limb2, limb1, limb0]`, so limb 0 is the last
    /// eight bytes. Getting this backwards is the most likely way a wallet
    /// and the circuit disagree while both look correct.
    fn limbs(hex: &str) -> [Fp; RATE] {
        let b: Vec<u8> = (0..32)
            .map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
            .collect();
        let mut out = [Fp::ZERO; RATE];
        for (j, slot) in out.iter_mut().enumerate() {
            let hi = 24 - 8 * j;
            let mut w = [0u8; 8];
            w.copy_from_slice(&b[hi..hi + 8]);
            *slot = Fp::from_u64(u64::from_be_bytes(w));
        }
        out
    }

    /// The live note the contracts lane put in the pool at leaf 1, checked
    /// against the derivation the circuit constrains.
    ///
    /// The point is not that their arithmetic is suspect. It is that a note
    /// whose `spend_pk` is not `compress(sk, tag(SPEND_DOMAIN))` under these
    /// exact parameters is a note no prover can witness a preimage for, and
    /// the failure would show up hours later as an unprovable statement
    /// rather than as a mismatch. One second here, or an afternoon there.
    #[test]
    fn the_live_note_derives_from_its_secret() {
        let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
        let sk = limbs("9c4b73105bd8c464250d949588b61807aee31121ea9d916ff6bf6105dab3ff7e");
        let keys = derive(&h, sk);

        let want_spend = limbs("34fa40101c68ad0be214e8f04c7213b9be559c9db4c40b8ab99e0875c28df6b4");
        let want_nk = limbs("476caa1bc9f1a6a13ee910f1d823aed193dec6105a19aae8c8599b7fe7c2acbe");

        assert_eq!(
            keys.spend_pk, want_spend,
            "spend_pk does not descend from this sk"
        );
        assert_eq!(keys.nk, want_nk, "nk does not descend from this sk");

        /*
         * The limb order the pool packed into its commitment preimage, as
         * they sent it, against the same unpacking. If these disagree the
         * two sides are hashing different field elements from the same hex.
         */
        assert_eq!(
            keys.spend_pk.map(|v| v.to_u64()),
            [
                13375137245205231284,
                13715040441383259018,
                16290901870878266297,
                3817434072090193163
            ],
            "the spend_pk limbs are not the ones the pool committed"
        );
    }
}
