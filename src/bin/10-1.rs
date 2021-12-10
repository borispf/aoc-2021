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
    let mut total_error = 0;
    for line in input.lines() {
        total_error += error_score(line.trim());
    }
    dbg!(total_error);
    Ok(())
}

fn error_score(s: &str) -> i128 {
    let mut close = Vec::new();
    for b in s.as_bytes() {
        match *b {
            b'(' => close.push(b')'),
            b'[' => close.push(b']'),
            b'{' => close.push(b'}'),
            b'<' => close.push(b'>'),
            _ => {
                if Some(*b) != close.pop() {
                    match b {
                        b')' => return 3,
                        b']' => return 57,
                        b'}' => return 1197,
                        b'>' => return 25137,
                        _ => {},
                    }
                }
            }
        }
    }
    0
}
