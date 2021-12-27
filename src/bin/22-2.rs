use itertools::Itertools;
use ndarray::prelude::*;

fn main() {
    // let input = include_str!("../../inputs/22-sample.txt");
    let input = include_str!("../../inputs/22.txt");
    let n = input.lines().count();
    let mut core = Array::<u8, _>::zeros((2 * n - 1, 2 * n - 1, 2 * n - 1));
    let mut part = [vec![], vec![], vec![]];
    let mut rect = vec![];
    for line in input.lines() {
        let parts = line.split(' ').collect_vec();
        let mut s = vec![];
        for (i, coord) in parts[1].split(',').enumerate() {
            let lohi = coord.split("..").collect_vec();
            let lo = lohi[0][2..].parse::<i64>().unwrap();
            let hi = 1 + lohi[1].parse::<i64>().unwrap();
            s.push(lo);
            s.push(hi);
            part[i].push(lo);
            part[i].push(hi);
        }
        let onoff = if parts[0] == "on" { 1 } else { 0 };
        rect.push((onoff, s));
    }
    part[0].sort();
    part[1].sort();
    part[2].sort();
    for (onoff, r) in rect {
        let mut s = vec![];
        for i in 0..3 {
            let lo = part[i].binary_search(&r[2 * i + 0]).unwrap();
            let hi = part[i].binary_search(&r[2 * i + 1]).unwrap();
            s.push(lo);
            s.push(hi);
        }
        // println!("{:?} {:?}", onoff, s);
        core.slice_mut(s![s[0]..s[1], s[2]..s[3], s[4]..s[5]])
            .fill(onoff);
    }
    let mut sum = 0;
    for ((x, y, z), elem) in core.indexed_iter() {
        if *elem == 0 {
            continue;
        }
        sum += *elem as i64
            * (part[0][x + 1] - part[0][x])
            * (part[1][y + 1] - part[1][y])
            * (part[2][z + 1] - part[2][z]);
    }
    dbg!(sum);
}
