use ndarray::prelude::*;
use itertools::Itertools;

fn main() {
    // let input = include_str!("../../inputs/22-sample.txt");
    let input = include_str!("../../inputs/22.txt");
    let mut core = Array::<i32, _>::zeros((101, 101, 101));
    for line in input.lines() {
        let parts = line.split(' ').collect_vec();
        let mut s = Vec::<usize>::new();
        for coord in parts[1].split(',') {
            let lohi = coord.split("..").collect_vec();
            let lo = 50 + lohi[0][2..].parse::<i64>().unwrap().clamp(-50, 51);
            let hi = 50 + (1 + lohi[1].parse::<i64>().unwrap()).clamp(-50, 51);
            s.push(lo.try_into().unwrap());
            s.push(hi.try_into().unwrap());
        }
        let onoff = if parts[0] == "on" { 1 } else { 0 };
        dbg!(onoff);
        // if s[0] == s[1] || s[2] == s[3] || s[4] == s[5] {
        //     continue;
        // }
        core.slice_mut(s![s[0]..s[1], s[2]..s[3], s[4]..s[5]]).fill(onoff);
    }
    dbg!(core.sum());
}
