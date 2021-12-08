use itertools::Itertools;
// use std::cmp::{max, min};
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let input = {
        let mut s = String::new();
        stdin.lock().read_to_string(&mut s)?;
        s
    };
    let tokens = input.split_whitespace().filter(|s| *s != "|").collect_vec();
    let lines = tokens.chunks(14).collect_vec();
    let mut ans = 0;
    for l in &lines {
        for (i, t) in l.iter().enumerate() {
            if i >= 10 && (t.len() <= 4 || t.len() == 7) {
                ans += 1;
            }
        }
    }
    println!("{:?}", ans);
    Ok(())
}
