// use itertools::Itertools;
// use std::cmp::{max, min};
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let input = {
        let mut s = String::new();
        stdin.lock().read_to_string(&mut s)?;
        s
    };
    let h = {
        let mut h = [[10; 102]; 102];
        for (i, line) in input.split_whitespace().enumerate() {
            for (j, b) in line.as_bytes().iter().enumerate() {
                h[i + 1][j + 1] = b - b'0';
            }
        }
        h
    };
    let mut total_risk = 0i64;
    for i in 1..=100 {
        for j in 1..=100 {
            if h[i][j]
                < n4(i, j)
                    .into_iter()
                    .map(|(ni, nj)| h[ni][nj])
                    .min()
                    .unwrap()
            {
                total_risk += 1 + h[i][j] as i64;
            }
        }
    }
    dbg!(total_risk);
    Ok(())
}

fn n4(i: usize, j: usize) -> [(usize, usize); 4] {
    [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
}
