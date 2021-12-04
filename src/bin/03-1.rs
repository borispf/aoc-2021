use std::io::BufRead;

const N_BITS: u128 = 12;

fn main() {
    let stdin = std::io::stdin();
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut numbers = stdin
        .lock()
        .lines()
        .map(|l| u128::from_str_radix(l.unwrap().trim(), 2).unwrap())
        .collect::<Vec<_>>();
    for i in 0..N_BITS {
        let mask = 1 << i;
        numbers.sort_by_key(|x| x & mask);
        let mid = numbers[numbers.len() / 2];
        gamma += mid & mask;
        epsilon += !mid & mask;
    }
    println!("{:?}", gamma * epsilon);
}
