// dims probe: what sets the recursion's constraint degree, and therefore its
// evaluation domain. The widest running product wins, and pack lets a class
// wider than the cap through un-split on purpose, so the cap is not the answer.
use crate::crypto::stark::air::Air;
use crate::recursion_assembly::{assemble, Tamper};

#[test]
#[ignore]
fn probe_group_widths() {
    let asm = assemble(Tamper::None);
    let w = asm.wired.group_widths();
    let max = w.iter().copied().max().unwrap_or(0);
    let mut hist = alloc::vec![0usize; max + 1];
    for k in &w {
        hist[*k] += 1;
    }
    let t = 1usize << asm.wired.log_trace_len();
    let cols = asm.wired.trace_width() + asm.wired.periodic_columns().len();
    let domain = |deg: usize| (2 * (deg.max(1) * t).next_power_of_two()) << 3;
    let gb = |deg: usize| (cols as u128 * domain(deg) as u128 * 8) / (1024 * 1024 * 1024);
    let rd = asm.wired.region_degrees();
    let mut rh = alloc::vec![0usize; rd.iter().copied().max().unwrap_or(0) + 1];
    for k in &rd {
        rh[*k] += 1;
    }
    std::eprintln!(
        "GROUPS n={} max_width={max} degree={} hist={hist:?}",
        w.len(),
        asm.wired.constraint_degree()
    );
    std::eprintln!(
        "REGIONS n={} max_degree={} hist={rh:?}",
        rd.len(),
        rd.iter().copied().max().unwrap_or(0)
    );
    // The domain follows deg * t, and t is the span rounded up to a power of
    // two, so slack in the span is bought back at full price.
    // The raw row count before rounding. Every query block has the same shape,
    // so the last region is as tall as its opposite number one block back.
    let o = &asm.region_offsets;
    let n = o.len();
    let raw = o[n - 1] + (o[n - 5] - o[n - 6]);
    std::eprintln!(
        "RAW rows={raw} rounded_span={} product_needs={} fits_in_2^18={}",
        asm.lay.span,
        raw + 1,
        raw + 1 <= 262144
    );
    std::eprintln!(
        "SPAN span={} t={t} headroom={} deg_t={} rounded={}",
        asm.lay.span,
        t - asm.lay.span,
        asm.wired.constraint_degree() * t,
        (asm.wired.constraint_degree() * t).next_power_of_two()
    );
    // What each achievable max width would cost. The domain steps when the
    // product of degree and trace length crosses a power of two, so the saving
    // is a cliff rather than a slope.
    for cut in (1..=max).rev() {
        std::eprintln!(
            "  max_width={cut} degree={} total_GB={}",
            cut + 1,
            gb(cut + 1)
        );
    }
}
