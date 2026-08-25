// Probe: do same-kind regions carry identical periodic columns across queries?
// If they do, 164 regions collapse to 9 kinds and the periodic count with them.
use crate::crypto::stark::air::Air;
use crate::recursion_assembly::{deep, fri, inner, points, Tamper};

#[test]
#[ignore]
fn probe_region_kind_duplication() {
    let h = inner::hasher();
    let inner = inner::join_split(&h);
    let ft = fri::fri_transcript(&h, &inner);

    let (d0, _, _) = deep::deep_region_k(&h, &inner, 0, Tamper::None);
    let (d1, _, _) = deep::deep_region_k(&h, &inner, 1, Tamper::None);
    let f0 = fri::fri_fold_k(&inner, &ft, 0, Tamper::None);
    let f1 = fri::fri_fold_k(&inner, &ft, 1, Tamper::None);
    let p0 = points::point_regions_k(&[true, false, true], f0.ik, ft.log_n, Tamper::None);
    let p1 = points::point_regions_k(&[true, false, true], f1.ik, ft.log_n, Tamper::None);

    let au0 = crate::recursion_assembly::auth::auth_side_k(&h, &inner, f0.ik, 0, Tamper::None);
    let au1 = crate::recursion_assembly::auth::auth_side_k(&h, &inner, f1.ik, 1, Tamper::None);
    let mut auth_same = 0usize;
    let mut fp_same = 0usize;
    for k in 1..8usize {
        let fk = fri::fri_fold_k(&inner, &ft, k, Tamper::None);
        let ak = crate::recursion_assembly::auth::auth_side_k(&h, &inner, fk.ik, k, Tamper::None);
        let pk = points::point_regions_k(&[true, false, true], fk.ik, ft.log_n, Tamper::None);
        if ak.region.periodic_columns() == au0.region.periodic_columns() {
            auth_same += 1;
        }
        if pk.fp.periodic_columns() == p0.fp.periodic_columns() {
            fp_same += 1;
        }
    }
    let mut distinct: alloc::vec::Vec<alloc::vec::Vec<alloc::vec::Vec<crate::crypto::stark::field::Fp>>> =
        alloc::vec::Vec::new();
    for k in 0..inner.proof.fri.queries.len() {
        let fk = fri::fri_fold_k(&inner, &ft, k, Tamper::None);
        let ak = crate::recursion_assembly::auth::auth_side_k(&h, &inner, fk.ik, k, Tamper::None);
        let p = ak.region.periodic_columns();
        if !distinct.contains(&p) {
            distinct.push(p);
        }
    }
    std::eprintln!(
        "DUP auth_matches_q0={auth_same}/7 fp_matches_q0={fp_same}/7 auth01={} \
         auth_distinct={}/{}",
        au0.region.periodic_columns() == au1.region.periodic_columns(),
        distinct.len(),
        inner.proof.fri.queries.len()
    );
    let per_query = d0.periodic_columns().len()
        + f0.fold.periodic_columns().len()
        + au0.region.periodic_columns().len()
        + p0.ip.periodic_columns().len()
        + p0.fp.periodic_columns().len();
    std::eprintln!(
        "KINDS deep={}/{} fold={}/{} ip={}/{} auth={} fp={} per_query_block={per_query} \
         over_32={} ",
        d0.periodic_columns().len(),
        d0.periodic_columns() == d1.periodic_columns(),
        f0.fold.periodic_columns().len(),
        f0.fold.periodic_columns() == f1.fold.periodic_columns(),
        p0.ip.periodic_columns().len(),
        p0.ip.periodic_columns() == p1.ip.periodic_columns(),
        au0.region.periodic_columns().len(),
        p0.fp.periodic_columns().len(),
        per_query * 32
    );
}
