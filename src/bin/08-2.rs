use itertools::Itertools;
// use std::cmp::{max, min};
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let input = {
        let mut s = String::new();
        stdin.lock().read_to_string(&mut s)?;
        s
    };
    let tokens = input.split_whitespace().filter(|s| *s != "|").map(parse_seg).collect_vec();
    let lines = tokens.chunks(14).collect_vec();
    let mut num_sum = 0;
    for line in &lines {
        let mut line = Vec::from(*line);
        let (codes, digits) = line.split_at_mut(10);
        codes.sort_by_key(|c| c.count_ones());
        let mut d2c = [0u8; 10];

        d2c[1] = codes[0];
        d2c[7] = codes[1];
        d2c[4] = codes[2];
        d2c[8] = codes[9];

        // Find 0
        for c in &codes[6..9] {
            if u8::count_ones(c & (d2c[4] & !d2c[1])) == 1 {
                d2c[0] = *c;
            }
        }
        // Find 6
        for c in &codes[6..9] {
            if u8::count_ones(c & d2c[1]) == 1 {
                d2c[6] = *c;
            }
        }
        // Find 9
        for c in &codes[6..9] {
            if u8::count_ones(c & d2c[4]) == 4 {
                d2c[9] = *c;
            }
        }
        // Find 3
        for c in &codes[3..6] {
            if u8::count_ones(c & d2c[1]) == 2 {
                d2c[3] = *c;
            }
        }
        d2c[5] = d2c[6] & d2c[9];
        // Find 2
        for c in &codes[3..6] {
            if *c != d2c[3] && *c != d2c[5] {
                d2c[2] = *c;
            }
        }
        let mut num = 0;
        for digit in digits {
            num *= 10;
            for (i, c) in d2c.iter().enumerate() {
                if c == digit {
                    num += i;
                    break;
                }
            }
        }
        dbg!(num);
        // dbg!(d2c);
        num_sum += num;
    }
    // println!("{:?}", ans);
    println!("{:?}", num_sum);
    Ok(())
}

fn parse_seg(s: &str) -> u8 {
    let mut code = 0u8;
    for b in s.as_bytes() {
        code |= 1 << (b - b'a');
    }
    code
}
