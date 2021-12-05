use itertools::Itertools;
use std::cmp::{max, min};
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut floor = [[0u16; 1000]; 1000];
    for line in stdin.lock().lines() {
        let l = line
            .unwrap()
            .split(' ')
            .flat_map(|s| s.split(','))
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect_vec();
        if l[0] != l[2] && l[1] != l[3] {
            continue;
        }
        for r in min(l[0], l[2])..=max(l[0], l[2]) {
            for c in min(l[1], l[3])..=max(l[1], l[3]) {
                floor[r][c] += 1;
            }
        }
    }
    for r in &floor {
        for c in r {
            match c {
                0 => print!("."),
                1..=9 => print!("{}", c),
                _ => print!("+"),
            }
        }
        println!()
    }
    let double_cross = floor
        .iter()
        .flat_map(|r| r.iter())
        .filter(|f| **f >= 2)
        .count();
    println!("{:?}", double_cross);
}
