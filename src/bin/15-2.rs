use itertools::Itertools;
// use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::io::Read;

const SIDE: isize = 10_000_000;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let input = {
        let mut s = String::new();
        stdin.lock().read_to_string(&mut s)?;
        s
    };
    let mut r = Vec::new();
    for line in input.split_whitespace() {
        let mut l = line.as_bytes().iter().map(|b| (*b - b'0') as isize).collect_vec();
        let mut ol = l.clone();
        for _ in 1..5 {
            for olr in ol.iter_mut() {
                *olr = 1 + *olr % 9
            }
            l.extend(ol.clone());
        }
        r.push(l);
    }
    let mut or = r.clone();
    for _ in 1..5 {
        for orr in or.iter_mut() {
            for orrv in orr.iter_mut() {
                *orrv = 1 + *orrv % 9
            }
        }
        r.extend(or.clone());
    }
    // dbg!(&r);
    for row in r.iter_mut() {
        row.insert(0, SIDE);
        row.push(SIDE);
    }
    let mut top = r[0].clone();
    top.fill(SIDE);
    // dbg!(&top);
    r.insert(0, top.clone());
    r.push(top);

    let mut tr = r.clone();
    for v in &mut tr {
        v.fill(SIDE);
    }

    // print_map(&r);

    // dbg!(&r, &tr);

    let mut q = BinaryHeap::new();
    tr[1][1] = 0;
    q.push((0, tr[1][1], 1, 1));
    while let Some((_pij, rij, i, j)) = q.pop() {
        if rij != tr[i][j] || rij >= r[0][0] {
            continue;
        }
        if i == tr.len() - 2 && j == tr.len() - 2 {
            break;
        }
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

fn print_map(r: &Vec<Vec<isize>>) {
    for row in r {
        for v in row {
            if *v <= 9 {
                print!("{}", *v);
            }
            else {
                print!("#");
            }
        }
        println!();
    }
}
