/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

use super::derive::{hasher, limbs, position, secret, tag};
use super::derive::{DEAD_DOMAIN, NULL_DOMAIN, POOL_LOG_ROUNDS, SPEND_DOMAIN};
use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;

const DERIVATION: &str = "spend_pk=compress(sk,[SPEND_DOMAIN,0,0,0]); nk=compress(sk,[NULL_DOMAIN,0,0,0]); nf=compress(compress(nk,cm),[leaf_index,live?0:DEAD_DOMAIN,0,0])";
const COMMITMENT: &str =
    "owner=compress(spend_pk,blinding); cm=compress([value_lo,value_hi,asset_id,NOTE_DOMAIN],owner)";

fn digits(d: [Fp; RATE]) -> String {
    let v: Vec<String> = d.iter().map(|x| x.value().to_string()).collect();
    format!("[{}]", v.join(","))
}

fn row(h: &Poseidon, seed: u64, value: u64, leaf_index: u64) -> String {
    let sk = secret(seed);
    let spend_pk = h.compress(&sk, &tag(SPEND_DOMAIN));
    let nk = h.compress(&sk, &tag(NULL_DOMAIN));
    let b = [seed + 5, seed + 6, seed + 7, seed + 8];
    let owner = h.commit_owner(&spend_pk, &b.map(Fp::from_u64));
    let cm = h.commit_note(&limbs(value, 0, spend_pk, b));
    let nf = |live| h.compress(&h.compress(&nk, &cm), &position(leaf_index, live));
    let mut s = format!("{{\"sk\":{},", digits(sk));
    s.push_str(&format!("\"spend_pk\":{},", digits(spend_pk)));
    s.push_str(&format!("\"nk\":{},", digits(nk)));
    s.push_str(&format!("\"value\":{value},\"asset_id\":0,"));
    s.push_str(&format!("\"blinding\":{},", digits(b.map(Fp::from_u64))));
    s.push_str(&format!("\"owner\":{},", digits(owner)));
    s.push_str(&format!("\"cm\":{},", digits(cm)));
    s.push_str(&format!("\"leaf_index\":{leaf_index},"));
    s.push_str(&format!("\"nf\":{},", digits(nf(true))));
    s.push_str(&format!("\"nf_dead\":{}}}", digits(nf(false))));
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
    s.push_str(&format!("\"spend_domain\":{SPEND_DOMAIN},"));
    s.push_str(&format!("\"null_domain\":{NULL_DOMAIN},"));
    s.push_str(&format!("\"dead_domain\":{DEAD_DOMAIN},"));
    s.push_str(&format!("\"derivation\":\"{DERIVATION}\","));
    s.push_str(&format!("\"commitment\":\"{COMMITMENT}\","));
    s.push_str(&format!("\"cases\":[{}]}}\n", cases.join(",")));
    s
}
