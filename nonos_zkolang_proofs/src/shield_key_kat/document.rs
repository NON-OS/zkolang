/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

use super::derive::{hasher, limbs, secret, tag, NULL_DOMAIN, POOL_LOG_ROUNDS, SPEND_DOMAIN};
use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;

const DERIVATION: &str = "spend_pk=compress(sk,[SPEND_DOMAIN,0,0,0]); nk=compress(sk,[NULL_DOMAIN,0,0,0]); nf=compress(compress(nk,cm),[leaf_index,0,0,0])";

fn digits(d: [Fp; RATE]) -> String {
    let v: Vec<String> = d.iter().map(|x| x.value().to_string()).collect();
    format!("[{}]", v.join(","))
}

fn row(h: &Poseidon, seed: u64, value: u64, leaf_index: u64) -> String {
    let sk = secret(seed);
    let spend_pk = h.compress(&sk, &tag(SPEND_DOMAIN));
    let nk = h.compress(&sk, &tag(NULL_DOMAIN));
    let b = [seed + 5, seed + 6, seed + 7, seed + 8];
    let cm = h.commit_note(&limbs(value, 0, spend_pk, b));
    let nf = h.compress(&h.compress(&nk, &cm), &tag(leaf_index));
    let mut s = String::new();
    s.push_str(&format!("{{\"sk\":{},", digits(sk)));
    s.push_str(&format!("\"spend_pk\":{},", digits(spend_pk)));
    s.push_str(&format!("\"nk\":{},", digits(nk)));
    s.push_str(&format!("\"value\":{},\"asset_id\":0,", value));
    s.push_str(&format!(
        "\"blinding\":[{},{},{},{}],",
        b[0], b[1], b[2], b[3]
    ));
    s.push_str(&format!("\"cm\":{},", digits(cm)));
    s.push_str(&format!(
        "\"leaf_index\":{},\"nf\":{}}}",
        leaf_index,
        digits(nf)
    ));
    s
}

pub(super) fn document() -> String {
    let h = hasher();
    let cases: Vec<String> = [
        (1u64, 1_000u64, 0u64),
        (2, 5_000_000, 7),
        (3, 4_294_967_295, 31),
    ]
    .into_iter()
    .map(|(s, v, l)| row(&h, s, v, l))
    .collect();
    let mut s = String::from("{\"artifact\":\"shield-key-hierarchy\",");
    s.push_str(&format!("\"rounds\":{},", 1u32 << POOL_LOG_ROUNDS));
    s.push_str(&format!("\"spend_domain\":{},", SPEND_DOMAIN));
    s.push_str(&format!("\"null_domain\":{},", NULL_DOMAIN));
    s.push_str(&format!("\"derivation\":\"{}\",", DERIVATION));
    s.push_str(&format!("\"cases\":[{}]}}\n", cases.join(",")));
    s
}
