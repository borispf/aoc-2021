// use itertools::Itertools;
// use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let input = {
        let mut s = String::new();
        stdin.lock().read_to_string(&mut s)?;
        s
    };
    let mut r = vec![Vec::new()];
    for line in input.split_whitespace() {
        let mut l = vec![10000; 1];
        l.extend(line.as_bytes().iter().map(|b| (*b - b'0') as isize));
        l.push(10000);
        r.push(l);
    }
    r[0] = r[r.len() - 1].clone();
    r[0].fill(10000);
    r.push(r[0].clone());
    let mut tr = r.clone();
    for v in &mut tr {
        v.fill(100_000);
    }
    let mut q = BinaryHeap::new();
    tr[1][1] = 0;
    q.push((0, tr[1][1], 1, 1));
    while let Some((_pij, rij, i, j)) = q.pop() {
        if rij != tr[i][j] || rij >= r[0][0] {
            continue;
        }
        // println!("{} {} {}", i, j, rij);
        for (ni, nj) in n4(i, j) {
            let rnij = rij + r[ni][nj];
            if rnij < tr[ni][nj] {
                tr[ni][nj] = rnij;
                q.push((-rnij, rnij, ni, nj));
            }
        }
    }
    // dbg!(tr);
    dbg!(r[0][0]);
    dbg!(tr[tr.len() - 2][tr.len() - 2]);
    Ok(())
}

fn n4(i: usize, j: usize) -> [(usize, usize); 4] {
    [
        (i + 1, j),
        (i - 1, j),
        (i, j + 1),
        (i, j - 1),
        // (i + 1, j - 1),
        // (i + 1, j + 1),
        // (i - 1, j + 1),
        // (i - 1, j - 1),
    ]
}
