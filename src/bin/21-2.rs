use cached::proc_macro::cached;
use itertools::Itertools;

fn mod10(x: usize) -> usize {
    (x - 1) % 10 + 1
}

#[cached]
fn wins(p: usize, sq: [usize; 2], sc: [usize; 2]) -> [u128; 2] {
    if sc[1 - p] >= 21 {
        let mut w = [0, 0];
        w[1 - p] += 1;
        return w;
    }
    let mut wt = [0, 0];
    for ((d1, d2), d3) in (1..=3).cartesian_product(1..=3).cartesian_product(1..=3) {
        let roll = d1 + d2 + d3;
        let mut sq2 = sq;
        let mut sc2 = sc;
        sq2[p] = mod10(sq2[p] + roll);
        sc2[p] += sq2[p];
        let [w0, w1] = wins(1 - p, sq2, sc2);
        wt[0] += w0;
        wt[1] += w1;
    }
    wt
}

fn main() {
    dbg!(wins(0, [4, 9], [0, 0]));
}
