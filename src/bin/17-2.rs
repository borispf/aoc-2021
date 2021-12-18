use itertools::Itertools;

type State = (i32, i32, i32, i32);
type Target = std::ops::RangeInclusive<i32>;

const TX: Target = 137..=171i32;
const TY: Target = -98..=-73i32;

fn main() {
    dbg!(solve(TX, TY));
}

fn step((x, y, vx, vy): State) -> State {
    (x + vx, y + vy, 0.max(vx - 1), vy - 1)
}

fn is_hit(s0: State, tx: &Target, ty: &Target) -> bool {
    let mut s = s0;
    loop {
        if tx.contains(&s.0) && ty.contains(&s.1) {
            return true;
        }
        if s.1 < *ty.start() { return false; }
        if s.0 < *tx.start() && s.2 == 0 { return false; }
        s = step(s);
    }
}

fn solve(tx: Target, ty: Target) -> usize {
    let mut hits = 0;
    for (vx, vy) in (0..=*tx.end()).cartesian_product(*ty.start()..=(1 - *ty.start())) {
        if is_hit((0, 0, vx, vy), &tx, &ty) {
            // println!("{} {}", vx, vy);
            hits += 1
        }
    }
    hits
}

#[test]
fn test_examples() {
    assert!(is_hit((0, 0, 7, 2), &(20..=30), &(-10..=-5)));
    assert_eq!(112, solve(20..=30, -10..=-5));
}
