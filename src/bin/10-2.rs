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
    let mut scores = Vec::new();
    for line in input.lines() {
        if let Some(score) = incomplete_score(line.trim()) {
            scores.push(score);
        }
    }
    scores.sort_unstable();
    dbg!(scores[scores.len() / 2]);
    Ok(())
}

fn incomplete_score(s: &str) -> Option<i128> {
    let mut close = Vec::new();
    for b in s.as_bytes() {
        match *b {
            b'(' => close.push(b')'),
            b'[' => close.push(b']'),
            b'{' => close.push(b'}'),
            b'<' => close.push(b'>'),
            _ => {
                if Some(*b) != close.pop() {
                    return None;
                }
            }
        }
    }
    let mut score = 0;
    for b in close.iter().rev() {
        score *= 5;
        match *b {
            b')' => score += 1,
            b']' => score += 2,
            b'}' => score += 3,
            b'>' => score += 4,
            _ => panic!("{:?} {:?}", b, close),
        }
    }
    Some(score)
}
