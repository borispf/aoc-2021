use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let (hori, depth) = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .fold((0, 0), step);
    println!("{}", hori * depth);
}

fn step(state: (i64, i64), line: String) -> (i64, i64) {
    let (hori, depth) = state;
    let line_split: Vec<_> = line.trim().split(' ').collect();
    let amount: i64 = line_split[1].parse().unwrap();
    match line_split[0] {
        "forward" => (hori + amount, depth),
        "up" => (hori, depth - amount),
        "down" => (hori, depth + amount),
        _ => panic!(),
    }
}
