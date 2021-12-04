use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let (hori, depth, _aim) = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .fold((0, 0, 0), step);
    println!("{}", hori * depth);
}

fn step(state: (i64, i64, i64), line: String) -> (i64, i64, i64) {
    let (hori, depth, aim) = state;
    let line_split: Vec<_> = line.trim().split(' ').collect();
    let x: i64 = line_split[1].parse().unwrap();
    match line_split[0] {
        "forward" => (hori + x, depth + aim * x, aim),
        "up" => (hori, depth, aim - x),
        "down" => (hori, depth, aim + x),
        _ => panic!(),
    }
}
