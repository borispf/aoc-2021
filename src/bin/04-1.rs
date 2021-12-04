use itertools::Itertools;
use std::io::BufRead;

type Board = [u8; 25];

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let draws: Vec<_> = line
        .trim()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let boards: Vec<Board> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .collect_vec()
        .chunks(6)
        .map(|it| {
            it.join(" ")
                .split_whitespace()
                .map(str::parse::<u8>)
                .filter_map(Result::ok)
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect();
    let bb = boards.iter().min_by_key(|b| score(*b, &draws).0).unwrap();
    let (bb_score, bb_mark) = score(bb, &draws);
    let bb_unmarked_sum: usize = bb.iter().zip(bb_mark.iter()).map(|(b, m)| (b * (1 - m)) as usize).sum();
    println!("{:?}", bb_unmarked_sum * draws[bb_score - 1] as usize);
}

fn score(b: &Board, draws: &[u8]) -> (usize, Board) {
    let mut marked: Board = [0; 25];
    for (score, d) in draws.iter().enumerate() {
        if let Some(idx) = b.iter().position(|x| x == d) {
            marked[idx] = 1;
            if done(marked) {
                return (score + 1, marked);
            }
        }
    }
    (usize::MAX, marked)
}

fn done(b: Board) -> bool {
    // Rows
    if b.chunks(5).any(|row| row.iter().sum::<u8>() == 5) {
        return true;
    }
    // Columns
    for c in 0..5 {
        if b[c..].iter().step_by(5).sum::<u8>() == 5 {
            return true;
        }
    }
    false
}
