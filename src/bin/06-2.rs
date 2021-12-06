// use itertools::Itertools;
// use std::cmp::{max, min};
// use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut old = [0u128; 9];
    let mut new = [0u128; 9];

    for fish in line
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
    {
        old[fish] += 1;
    }

    for _day in 1..=256 {
        for i in 1..=8 {
            new[i - 1] = old[i];
        }
        new[8] = old[0];
        new[6] += old[0];
        old.copy_from_slice(&new);
    }
    println!("{:?}", old.iter().sum::<u128>());
}
