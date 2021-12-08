// use itertools::Itertools;
// use std::cmp::{max, min};
// use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut old = [0u64; 9];

    for fish in line
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
    {
        old[fish] += 1;
    }

    for _day in 1..=80 {
        old.rotate_left(1);
        old[6] += old[8];
    }
    println!("{:?}", old);
    println!("{:?}", old.iter().sum::<u64>());
}
