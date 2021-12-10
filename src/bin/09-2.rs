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
        let mut h = [[9; 102]; 102];
        for (i, line) in input.split_whitespace().enumerate() {
            for (j, b) in line.as_bytes().iter().enumerate() {
                h[i + 1][j + 1] = b - b'0';
            }
        }
        h
    };
    let mut b = [[usize::MAX; 102]; 102];
    let mut s = Vec::<usize>::new();
    let mut num_basins = 0;
    let mut st = Vec::<(usize, usize, usize)>::new();
    for i in 1..=100 { for j in 1..=100 {
        if h[i][j] != 9 && b[i][j] == usize::MAX {
            b[i][j] = num_basins;
            num_basins += 1;
            s.push(0);
            st.push((i, j, b[i][j]))
        }
        while !st.is_empty() {
            let (i, j, bij) = st.pop().unwrap();
            s[bij] += 1;
            for (ni, nj) in n4(i, j) {
                if h[ni][nj] != 9 && b[ni][nj] == usize::MAX {
                    b[ni][nj] = bij;
                    st.push((ni, nj, bij));
                }
            }
        }
    }}
    // dbg!(s);
    s.sort();
    s.reverse();
    dbg!(s[..3].iter().product::<usize>());
    Ok(())
}

fn n4(i: usize, j: usize) -> [(usize, usize); 4] {
    [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
}
