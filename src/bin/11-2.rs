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
    let mut e = [[11; 12]; 12];
    for (i, line) in input.split_whitespace().enumerate() {
        for (j, b) in line.as_bytes().iter().enumerate() {
            e[i + 1][j + 1] = b - b'0';
        }
    }
    let mut flashes = 0;
    for step in 0..1000 {
        let mut st = Vec::new();
        for (i, j) in (1..=10).cartesian_product(1..=10) {
            e[i][j] += 1;
            if e[i][j] == 10 {
                st.push((i, j));
            }
        }
        while let Some((i, j)) = st.pop() {
            flashes += 1;
            for (ni, nj) in n8(i, j) {
                e[ni][nj] += 1;
                if e[ni][nj] == 10 {
                    st.push((ni, nj));
                }
            }
        }
        let mut flashed = 0;
        for (i, j) in (1..=10).cartesian_product(1..=10) {
            if e[i][j] >= 10 {
                e[i][j] = 0;
                flashed += 1;
            }
        }
        if flashed == 100 {
            dbg!(step);
            break;
        }
    }
    dbg!(flashes);
    Ok(())
}

fn n8(i: usize, j: usize) -> [(usize, usize); 8] {
    [
        (i + 1, j),
        (i - 1, j),
        (i, j + 1),
        (i, j - 1),
        (i + 1, j - 1),
        (i + 1, j + 1),
        (i - 1, j + 1),
        (i - 1, j - 1),
    ]
}
