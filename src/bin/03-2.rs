use std::io::BufRead;

const N_BITS: u128 = 12;

fn main() {
    let stdin = std::io::stdin();
    let mut o2 = stdin
        .lock()
        .lines()
        .map(|l| u128::from_str_radix(l.unwrap().trim(), 2).unwrap())
        .collect::<Vec<_>>();
    let mut co2 = o2.clone();

    for i in (0..N_BITS).rev() {
        let mask = 1 << i;
        o2.sort_by_key(|x| x & mask);
        let mc = mask & o2[o2.len() / 2];
        mask & o2[o2.len() / 2 - 1];
        o2 = o2.into_iter().filter(|x| x & mask == mc).collect();
        if o2.len() == 1 {
            break;
        }
    }

    for i in (0..N_BITS).rev() {
        let mask = 1 << i;
        co2.sort_by_key(|x| x & mask);
        let lc = dbg!(mask & !co2[co2.len() / 2]);
        dbg!(mask & !co2[co2.len() / 2 - 1]);
        co2 = co2.into_iter().filter(|x| x & mask == lc).collect();
        if co2.len() == 1 {
            break;
        }
        if co2.len() == 0 {
            dbg!(co2);
            panic!();
        }
    }
    println!("{:?}", o2[0] * co2[0]);
}
