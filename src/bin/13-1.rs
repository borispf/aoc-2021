use itertools::Itertools;
// use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let input = {
        let mut s = String::new();
        stdin.lock().read_to_string(&mut s)?;
        s
    };
    let mut paper = HashSet::<[i64; 2]>::new();
    let mut folds = Vec::new();
    for line in input.lines() {
        let l = line.split(',').collect_vec();
        if l.len() == 2 {
            let x = l[0].to_owned();
            let y = l[1].to_owned();
            paper.insert([
                x.parse::<i64>().unwrap(),
                y.parse::<i64>().unwrap(),
            ]);
        } else {
            if l[0].len() == 0 {
                continue;
            }
            let l = l[0].split('=').collect_vec();
            folds.push((
                (l[0].as_bytes()[l[0].len() - 1] - b'x') as usize,
                l[1].parse::<i64>().unwrap(),
            ))
        }
    }
    for (i, (axis, coord)) in folds.iter().enumerate() {
        if i == 1 {
            dbg!(paper.len());
            // print_paper(&paper);
        }
        let mut new_marks = Vec::new();
        for mut xy in paper.drain() {
            // dbg!(xy);
            let line = 0 + *coord;
            xy[*axis] = line - (line - xy[*axis]).abs();
            // dbg!(xy);
            new_marks.push(xy);
        }
        paper.extend(new_marks);
    }
    // dbg!(paper.len());
    print_paper(&paper);
    // dbg!(folds);
    Ok(())
}

fn print_paper(paper: &HashSet<[i64; 2]>) {
    let max_x = paper.iter().map(|xy| xy[0]).max().unwrap();
    let max_y = paper.iter().map(|xy| xy[1]).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if paper.contains(&[x, y]) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
