use itertools::Itertools;
// use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::Read;

type RuleMap = HashMap<[u8; 2], u8>;
type CharCounts = HashMap<u8, usize>;
type CountCache = HashMap<(u8, u8, usize), CharCounts>;

const DEPTH: usize = 10;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let input = {
        let mut s = String::new();
        stdin.lock().read_to_string(&mut s)?;
        s
    };
    let in_parts = input.split("\n\n").collect_vec();
    let start_pattern = in_parts[0].trim().as_bytes();
    let mut rules = RuleMap::new();
    for r in in_parts[1].lines().map(str::as_bytes) {
        rules.insert(r[0..2].try_into().unwrap(), r[6]);
    }
    let mut counts = CharCounts::new();
    let mut cache = CountCache::new();
    for (l, r) in start_pattern.iter().zip(start_pattern[1..].iter()) {
        *counts.entry(*l).or_default() += 1;
        for (char, count) in mid_counts(*l, *r, DEPTH, &rules, &mut cache) {
            *counts.entry(char).or_default() += count;
        }
    }
    *counts
        .entry(start_pattern[start_pattern.len() - 1])
        .or_default() += 1;
    dbg!(std::str::from_utf8(start_pattern).unwrap());
    print_rules(&rules);
    dbg!(DEPTH);
    print_counts(&counts);
    dbg!(counts.values().max().unwrap() - counts.values().min().unwrap());
    Ok(())
}

fn mid_counts(l: u8, r: u8, depth: usize, rules: &RuleMap, cache: &mut CountCache) -> CharCounts {
    let mut counts = CharCounts::new();
    if depth == 0 {
        return counts;
    }
    if let Some(cached) = cache.get(&(l, r, depth)) {
        return cached.clone();
    }
    if let Some(m) = rules.get(&[l, r]) {
        *counts.entry(*m).or_default() += 1;
        for (char, count) in mid_counts(l, *m, depth - 1, rules, cache) {
            *counts.entry(char).or_default() += count;
        }
        for (char, count) in mid_counts(*m, r, depth - 1, rules, cache) {
            *counts.entry(char).or_default() += count;
        }
    }
    cache.insert((l, r, depth), counts.clone());
    return counts;
}

fn print_rules(rules: &RuleMap) {
    for (key, val) in rules {
        println!("{} -> {}", std::str::from_utf8(key).unwrap(), *val as char)
    }
}

fn print_counts(counts: &CharCounts) {
    for (key, val) in counts {
        println!("{} -> {}", *key as char, val);
    }
}
