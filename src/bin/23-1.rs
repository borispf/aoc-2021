use crate::Square::{Hall, Room};
use std::collections::{HashMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Square {
    Hall { lr: i8 },
    Room { lr: i8, ud: i8 },
}

impl Square {
    fn to_usize(self) -> usize {
        match self {
            Hall { lr } => lr,
            Room { lr, ud } => 8 + lr + ud,
        }
        .try_into()
        .unwrap()
    }
}

#[cfg(test)]
const SQUARES: [Square; 19] = [
    Hall { lr: 0 },
    Hall { lr: 1 },
    Hall { lr: 2 },
    Hall { lr: 3 },
    Hall { lr: 4 },
    Hall { lr: 5 },
    Hall { lr: 6 },
    Hall { lr: 7 },
    Hall { lr: 8 },
    Hall { lr: 9 },
    Hall { lr: 10 },
    Room { lr: 2, ud: 1 },
    Room { lr: 2, ud: 2 },
    Room { lr: 4, ud: 1 },
    Room { lr: 4, ud: 2 },
    Room { lr: 6, ud: 1 },
    Room { lr: 6, ud: 2 },
    Room { lr: 8, ud: 1 },
    Room { lr: 8, ud: 2 },
];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pods: [Square; 8],
    occupied: [bool; 19],
}

impl State {
    fn is_done(&self) -> bool {
        self.pods
            .iter()
            .cloned()
            .enumerate()
            .all(|(p, s)| home(p, s))
            && self.occupied[11..].iter().all(|x| *x)
    }

    fn n(&self) -> Vec<(usize, State)> {
        let mut n = vec![];
        for pod in 0..8 {
            let mut next_sqs = vec![];
            match self.pods[pod] {
                hall @ Hall { .. } => {
                    let home = home_room(pod);
                    if !path_clear(hall, Room { lr: home, ud: 1 }, &self.occupied) {
                        continue;
                    }
                    for ud in [1, 2] {
                        let next_sq = Room { lr: home, ud };
                        if self.occupied[next_sq.to_usize()] {
                            break;
                        }
                        next_sqs.push(next_sq)
                    }
                }
                room @ Room { lr, ud } => {
                    if ud == 2 && self.occupied[room.to_usize() - 1] {
                        continue;
                    }
                    for next_lr in (0..=lr - 1).rev() {
                        if self.occupied[next_lr as usize] {
                            break;
                        }
                        if bad_hall(next_lr) {
                            continue;
                        }
                        next_sqs.push(Hall { lr: next_lr });
                    }
                    for next_lr in lr + 1..=10 {
                        if self.occupied[next_lr as usize] {
                            break;
                        }
                        if bad_hall(next_lr) {
                            continue;
                        }
                        next_sqs.push(Hall { lr: next_lr });
                    }
                }
            }
            for next_sq in next_sqs {
                let mut next = self.clone();
                next.pods[pod] = next_sq;
                next.occupied[next_sq.to_usize()] = true;
                next.occupied[self.pods[pod].to_usize()] = false;
                n.push((energy(pod) * d(self.pods[pod], next_sq), next));
            }
        }
        n
    }
}

fn path_clear(a: Square, b: Square, occupied: &[bool]) -> bool {
    match (a, b) {
        (Hall { .. }, Room { .. }) => path_clear(b, a, occupied),
        (room @ Room { lr, ud }, hall @ Hall { .. }) => {
            let room_hall = Hall { lr };
            let mut ij = [hall.to_usize(), room_hall.to_usize()];
            ij.sort();
            for k in ij[0]..=ij[1] {
                if k == room.to_usize() || k == hall.to_usize() {
                    continue;
                }
                if occupied[k] {
                    return false;
                }
            }
            if ud == 2 && occupied[room.to_usize() - 1] {
                false
            } else {
                true
            }
        }
        _ => unimplemented!(),
    }
}

fn bad_hall(lr: i8) -> bool {
    lr == 2 || lr == 4 || lr == 6 || lr == 8
}

fn home_room(pod: usize) -> i8 {
    (2 + 2 * (pod / 2)) as i8
}

fn home(pod: usize, sq: Square) -> bool {
    match sq {
        Room { lr, .. } if lr == home_room(pod) => true,
        _ => false,
    }
}

const fn energy(pod: usize) -> usize {
    10usize.pow((pod / 2) as u32)
}

fn d(a: Square, b: Square) -> usize {
    match (a, b) {
        (Hall { lr: lra }, Hall { lr: lrb }) => (lra - lrb).abs(),
        (Hall { lr }, Room { lr: lrr, ud: udr }) | (Room { lr: lrr, ud: udr }, Hall { lr }) => {
            udr + (lr - lrr).abs()
        }
        (Room { lr: lra, ud: uda }, Room { lr: lrb, ud: udb }) => {
            if lra == lrb {
                (uda - udb).abs()
            } else {
                (lra - lrb).abs() + uda + udb
            }
        }
    }
    .try_into()
    .unwrap()
}

fn dykstra(start: State) -> (usize, Vec<State>) {
    let mut prev = HashMap::<State, State>::new();
    let mut cost = HashMap::<State, usize>::new();
    let mut heap = BTreeSet::<(usize, State)>::new();

    heap.insert((0, start.clone()));
    cost.insert(start.clone(), 0);

    let mut end: Option<(usize, State)> = None;

    while !heap.is_empty() {
        let e = heap.iter().next().unwrap().clone();
        heap.remove(&e);
        let (s_cost, s) = e;
        if s.is_done() {
            end = Some((s_cost, s));
            break;
        }
        for (c, ss) in s.n() {
            let new_cost = c + s_cost;
            let prev_cost = cost.get(&ss).cloned().unwrap_or(usize::MAX / 2);
            if new_cost < prev_cost {
                heap.remove(&(prev_cost, ss.clone()));
                heap.insert((new_cost, ss.clone()));
                cost.insert(ss.clone(), new_cost);
                prev.insert(ss.clone(), s.clone());
            }
        }
    }
    let (total_cost, mut cur_s) = end.unwrap().clone();
    let mut path = vec![cur_s.clone()];
    while let Some(prev_s) = prev.get(&cur_s) {
        path.push(prev_s.clone());
        cur_s = prev_s.clone();
    }
    path.reverse();
    (total_cost, path)
}

fn main() {
    let start_state = State {
        pods: [
            Room { lr: 6, ud: 2 }, // A
            Room { lr: 8, ud: 2 }, // A
            Room { lr: 2, ud: 1 }, // B
            Room { lr: 4, ud: 1 }, // B
            Room { lr: 2, ud: 2 }, // C
            Room { lr: 4, ud: 2 }, // C
            Room { lr: 6, ud: 1 }, // D
            Room { lr: 8, ud: 1 }, // D
        ],
        occupied: [
            false, false, false, false, false, false, false, false, false, false, false, true,
            true, true, true, true, true, true, true,
        ],
    };

    let (cost, path) = dykstra(start_state);
    for s in path {
        println!("{}", s);
    }
    dbg!(cost);
}

#[test]
fn test_square_usize() {
    for (i, sq) in SQUARES.iter().enumerate() {
        assert_eq!(i, sq.to_usize());
    }
}
#[test]
fn test_d_sym() {
    use itertools::Itertools;
    for (a, b) in SQUARES.iter().cartesian_product(SQUARES.iter()) {
        assert_eq!(d(*a, *b), d(*b, *a));
    }
}
#[test]
fn test_energy() {
    assert_eq!(1, energy(0));
    assert_eq!(1, energy(1));
    assert_eq!(10, energy(2));
    assert_eq!(10, energy(3));
    assert_eq!(100, energy(4));
    assert_eq!(100, energy(5));
    assert_eq!(1000, energy(6));
    assert_eq!(1000, energy(7));
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut chars = [b' '; 19];
        chars[self.pods[0].to_usize()] = b'A';
        chars[self.pods[1].to_usize()] = b'A';
        chars[self.pods[2].to_usize()] = b'B';
        chars[self.pods[3].to_usize()] = b'B';
        chars[self.pods[4].to_usize()] = b'C';
        chars[self.pods[5].to_usize()] = b'C';
        chars[self.pods[6].to_usize()] = b'D';
        chars[self.pods[7].to_usize()] = b'D';
        writeln!(f, "·············")?;

        write!(f, "·")?;
        write!(f, "{}", String::from_utf8(chars[..11].to_vec()).unwrap())?;
        writeln!(f, "·")?;

        writeln!(
            f,
            "···{}·{}·{}·{}···",
            chars[11] as char, chars[13] as char, chars[15] as char, chars[17] as char
        )?;
        writeln!(
            f,
            "  ·{}·{}·{}·{}·  ",
            chars[12] as char, chars[14] as char, chars[16] as char, chars[18] as char
        )?;
        write!(f, "  ·········  ")
    }
}
