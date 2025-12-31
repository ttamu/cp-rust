use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {tc: usize,}
    for _ in 0..tc {
        input! {
            n: usize,
            h: u64,
            tlu: [(u64, u64, u64); n],
        }

        let ok = {
            let mut lo = h;
            let mut hi = h;
            let mut prev = 0;
            tlu.iter().all(|&(t, l, u)| {
                let dt = t - prev;
                lo = lo.saturating_sub(dt).max(1);
                hi += dt;
                lo = max(lo, l);
                hi = min(hi, u);
                prev = t;
                hi >= lo
            })
        };
        println!("{}", if ok { "Yes" } else { "No" });
    }
}
