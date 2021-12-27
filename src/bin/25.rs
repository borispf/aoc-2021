use itertools::Itertools;
// use std::collections::HashSet;
// use std::rc::Rc;

const R: u8 = b'>';
const D: u8 = b'v';
const F: u8 = b'.';

fn main() {
    // let input = include_bytes!("../../inputs/25-sample.txt");
    let input = include_bytes!("../../inputs/25.txt");

    let mut f = input
        .split(|b| *b == b'\n')
        .map(|r| r.to_vec())
        .collect_vec();
    let n_i = f.len();
    let n_j = f[0].len();

    print_floor(&f);
    for step in 0.. {
        let mut g = f.clone();
        let mut any_moved = false;
        for (i, j) in (0..n_i).cartesian_product(0..n_j) {
            if f[i][j] == R && f[i][(j + 1) % n_j] == F {
                g[i][j] = F;
                g[i][(j + 1) % n_j] = R;
                any_moved = true;
            }
        }
        f = g;
        g = f.clone();
        for (i, j) in (0..n_i).cartesian_product(0..n_j) {
            if f[i][j] == D && f[(i + 1) % n_i][j] == F {
                g[i][j] = F;
                g[(i + 1) % n_i][j] = D;
                any_moved = true;
            }
        }
        f = g;
        if step % 100 == 0 {
            println!("Step {}:", step + 1);
        }
        // print_floor(&f);
        if !any_moved {
            println!("{}", step + 1);
            break;
        }
    }
}

fn print_floor(f: &[Vec<u8>]) {
    for line in f {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
}
