use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let depths: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().trim().parse::<i64>().unwrap())
        .collect();
    let smoothed: Vec<i64> = depths.windows(3).map(|w| w.iter().sum()).collect();
    let increases = smoothed.windows(2).filter(|w| w[0] < w[1]).count();
    println!("{}", increases);
}
