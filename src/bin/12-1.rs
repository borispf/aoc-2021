use itertools::Itertools;
// use std::cmp::{max, min};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let input = {
        let mut s = String::new();
        stdin.lock().read_to_string(&mut s)?;
        s
    };
    let mut n = HashMap::<String, Vec<String>>::new();
    for line in input.lines() {
        let l = line.split('-').collect_vec();
        let u = l[0].to_owned();
        let v = l[1].to_owned();
        n.entry(u.clone()).or_default().push(v.clone());
        n.entry(v.clone()).or_default().push(u.clone());
    }
    let mut vis = HashSet::<String>::new();
    let num = dfs("start", &mut vis, &n);
    dbg!(num);
    Ok(())
}

fn dfs(u: &str, vis: &mut HashSet<String>, n: &HashMap<String, Vec<String>>) -> usize {
    if u == "end" { return 1; }
    vis.insert(u.to_owned());
    let mut paths = 0;
    for v in n.get(u).unwrap().iter() {
        if big(v) || !vis.contains(&*v) {
            paths += dfs(v, vis, n);
        }
    }
    vis.remove(u);
    paths
}

fn big(u: &str) -> bool {
    u.as_bytes()[0].is_ascii_uppercase()
}
